import { Identity, SignIdentity } from "@dfinity/agent";
import { AuthClient, IdleOptions } from "@dfinity/auth-client";
import {
  AuthClientStorage,
  IdbStorage,
  KEY_STORAGE_DELEGATION,
  KEY_STORAGE_KEY,
} from "@dfinity/auth-client/lib/cjs/storage";

export enum AuthLoginType {
  Desktop,
  Mobile = "callback_url",
}

export type IdentityParam = {
    key?: string;
    delegation?: string;
}

export class Auth {
  private static identityQueryItem = "_identity";
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
            onAuthenticated(this);
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

  public static async create(options?: {
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
  }): Promise<Auth> {
    const storage = options?.storage ?? new IdbStorage();

    // preloads into the storage an already available identity
    await Auth.preloadStorage(storage);

    const client = await AuthClient.create({
      ...options,
      storage,
    });

    return new Auth(client, storage);
  }

  /**
   * Reloads into the storage from the query string an already available key and delegation.
   *
   * @param storage Auth client storage to store identity information
   */
  private static async preloadStorage(
    storage: AuthClientStorage
  ): Promise<void> {
    const url = Auth.currentURL();
    if (!window[Auth.identityQueryItem]) {
        return;
    }

    const identityBase64 = String(window[Auth.identityQueryItem] ?? "");
    const identityParam = Buffer.from(identityBase64, "base64").toString("ascii");
    const preload: IdentityParam = JSON.parse(identityParam);

    if (preload.key) {
        await storage.set(KEY_STORAGE_KEY, preload.key);
    }

    if (preload.delegation) {
        await storage.set(KEY_STORAGE_DELEGATION, preload.delegation);

    }
  }
  public async handleMultiPlatformLogin(): Promise<void> {
    const key = await this.storage.get(KEY_STORAGE_KEY) ?? undefined;
    const delegation = await this.storage.get(KEY_STORAGE_DELEGATION) ?? undefined;
    const identityParam: IdentityParam = { key, delegation };
    const preloadParam = Buffer.from(JSON.stringify(identityParam), "ascii").toString("base64");
    const url = Auth.currentURL();

    switch(this.loginType()) {
        case AuthLoginType.Mobile:
            const callback = url.searchParams.get(AuthLoginType.Mobile);
            const authCallback = new URL(url.searchParams.get(AuthLoginType.Mobile) ?? "");
            if (!callback?.length || authCallback.protocol !== "https:") {
              throw new Error("Invalid callback url");
            }

            authCallback.searchParams.append(Auth.identityQueryItem, preloadParam);

            // apple universal links require the user to tap to trigger the native app to handle the url
            document.write(`<a href="${authCallback.toString()}" class="button">back to app</a>`);
            break;
        default:
            // desktop is enabled by default and doesn't need a special condition
            break;
    }
  }
}
