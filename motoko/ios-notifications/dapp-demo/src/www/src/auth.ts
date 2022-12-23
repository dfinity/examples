import {
  Identity,
  SignIdentity,
  DerEncodedPublicKey,
  PublicKey,
  Signature,
} from "@dfinity/agent";
import { AuthClient, IdleOptions } from "@dfinity/auth-client";
import {
  AuthClientStorage,
  IdbStorage,
  KEY_STORAGE_DELEGATION,
  KEY_STORAGE_KEY,
} from "@dfinity/auth-client/lib/cjs/storage";
import { fromHexString } from "@dfinity/candid";
import { Ed25519PublicKey } from "@dfinity/identity";

export enum AuthLoginType {
  Desktop,
  Mobile = "callback_url",
}

export type IdentityParam = {
  key?: string;
  delegation?: string;
};

export enum MultiPlatformLoggedInAction {
  Redirecting,
  Default,
}

/**
 * Used to login using an externally generated
 * keypair such as in a native app.
 */
class SessionIdentity extends SignIdentity {
  constructor(protected publicKey: PublicKey) {
    super();
    this.publicKey = publicKey;
  }

  public getPublicKey(): PublicKey {
    return this.publicKey;
  }

  public async sign(blob: ArrayBuffer): Promise<Signature> {
    throw new Error("Not implemented");
  }
}

export class Auth {
  private static sessionPublicKeyParam = "session";
  private static restoreKey = "__ii";
  private storage!: AuthClientStorage;
  private authClient!: AuthClient;
  private days = BigInt(1);
  private hours = BigInt(24);
  private nanoseconds = BigInt(3600000000000);

  constructor(authClient: AuthClient, storage: AuthClientStorage) {
    this.authClient = authClient;
    this.storage = storage;
  }

  private static currentURL(): URL {
    return new URL(window.location.href);
  }

  public login(onAuthenticated: (auth: Auth) => Promise<void>): void {
    this.authClient.login({
      onSuccess: async () => {
        if (await this.authClient.isAuthenticated()) {
          onAuthenticated(this);
        }
      },
      identityProvider:
        process.env.DFX_NETWORK === "ic"
          ? "https://identity.ic0.app/#authorize"
          : `http://localhost:${process.env.REPLICA_PORT}/?canisterId=${process.env.LOCAL_II_CANISTER}#authorize`,
      // Maximum authorization expiration is 30 days
      maxTimeToLive: this.days * this.hours * this.nanoseconds,
    });
  }

  public client(): AuthClient {
    return this.authClient;
  }

  public loginType(): AuthLoginType {
    if (Auth.currentURL().searchParams.has(AuthLoginType.Mobile)) {
      return AuthLoginType.Mobile;
    }

    return AuthLoginType.Desktop;
  }

  public static async create(
    options: {
      /**
       * An {@link Identity} to use as the base.
       *  By default, a new {@link AnonymousIdentity}
       */
      identity?: SignIdentity;
      /**
       * {@link AuthClientStorage}
       * @description Optional storage with get, set, and remove. Uses {@link LocalStorage} by default
       */
      storage?: AuthClientStorage;
      /**
       * Options to handle idle timeouts
       * @default after 10 minutes, invalidates the identity
       */
      idleOptions?: IdleOptions;
    } = {}
  ): Promise<Auth> {
    const storage = options.storage ?? new IdbStorage();
    const url = Auth.currentURL();

    if (url.searchParams.has(Auth.sessionPublicKeyParam)) {
      await Auth.cleanupStorage(storage);

      const session = url.searchParams.get(Auth.sessionPublicKeyParam) ?? "";
      const derPublicKey = fromHexString(session) as DerEncodedPublicKey;
      const publicKey = Ed25519PublicKey.fromDer(derPublicKey);

      options.identity = new SessionIdentity(publicKey);
    }

    // preloads into the storage an already available identity
    await Auth.preloadStorage(storage);

    const client = await AuthClient.create({
      ...options,
      storage,
    });

    return new Auth(client, storage);
  }

  private static async cleanupStorage(
    storage: AuthClientStorage
  ): Promise<void> {
    await storage.remove(KEY_STORAGE_KEY);
    await storage.remove(KEY_STORAGE_DELEGATION);
  }

  /**
   * Reloads into the storage from the window an already available key and delegation.
   *
   * @param storage Auth client storage to store identity information
   */
  private static async preloadStorage(
    storage: AuthClientStorage
  ): Promise<void> {
    const restore: IdentityParam = window?.[Auth.restoreKey] ?? {};

    if (!restore.delegation || !restore.key) {
      return;
    }

    await storage.set(KEY_STORAGE_KEY, restore.key);
    await storage.set(
      KEY_STORAGE_DELEGATION,
      Buffer.from(restore.delegation, "base64").toString("ascii")
    );
  }

  public async handleMultiPlatformLogin(): Promise<MultiPlatformLoggedInAction> {
    const url = Auth.currentURL();

    switch (this.loginType()) {
      case AuthLoginType.Mobile:
        const callback = url.searchParams.get(AuthLoginType.Mobile);
        const authCallback = new URL(
          url.searchParams.get(AuthLoginType.Mobile) ?? ""
        );
        if (
          !callback?.length ||
          authCallback.protocol !== "https:" ||
          authCallback.host.replace(".raw.", ".") !== url.host
        ) {
          throw new Error("Invalid callback url");
        }

        const delegations = await this.storage.get(KEY_STORAGE_DELEGATION);
        if (!delegations) {
          throw new Error("Missing delegations");
        }

        const base64Delegation = Buffer.from(delegations, "ascii").toString("base64");
        authCallback.hash = `${Auth.restoreKey}=${base64Delegation}`;

        // apple universal links require the user to tap to trigger the native app to handle the url
        document.write(
          `<a href="${authCallback.toString()}" class="button">back to app</a>`
        );
        return MultiPlatformLoggedInAction.Redirecting;
      default:
        // desktop is enabled by default and doesn't need a special condition
        return MultiPlatformLoggedInAction.Default;
    }
  }
}
