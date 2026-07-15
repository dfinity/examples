import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
  type ReactNode,
} from "react";
import { AuthClient } from "@icp-sdk/auth/client";
import type { Identity } from "@icp-sdk/core/agent";
import { iiUrl } from "./env";

interface AuthContextValue {
  /** The authenticated identity, or `undefined` when logged out. */
  identity: Identity | undefined;
  /** True until the initial session-restore attempt has completed. */
  isInitializing: boolean;
  /** Opens Internet Identity and, on success, sets the identity. */
  login: () => Promise<void>;
  /** Signs out and clears the identity. */
  clear: () => Promise<void>;
}

const AuthContext = createContext<AuthContextValue | undefined>(undefined);

export function AuthProvider({ children }: { children: ReactNode }) {
  const [authClient, setAuthClient] = useState<AuthClient>();
  const [identity, setIdentity] = useState<Identity | undefined>(undefined);
  const [isInitializing, setIsInitializing] = useState(true);

  useEffect(() => {
    const client = new AuthClient({
      identityProvider: iiUrl,
      // Keep the session alive; expiry is handled explicitly via
      // useHandleAgentError when the delegation is rejected.
      idleOptions: { disableIdle: true },
    });
    setAuthClient(client);

    void (async () => {
      // getIdentity() restores a previous session from storage if present.
      await client.getIdentity();
      if (client.isAuthenticated()) {
        setIdentity(await client.getIdentity());
      }
      setIsInitializing(false);
    })();
  }, []);

  const login = useCallback(async () => {
    if (!authClient) return;
    await authClient.signIn();
    if (authClient.isAuthenticated()) {
      setIdentity(await authClient.getIdentity());
    }
  }, [authClient]);

  const clear = useCallback(async () => {
    if (!authClient) return;
    await authClient.signOut();
    setIdentity(undefined);
  }, [authClient]);

  const value = useMemo(
    () => ({ identity, isInitializing, login, clear }),
    [identity, isInitializing, login, clear]
  );

  return <AuthContext.Provider value={value}>{children}</AuthContext.Provider>;
}

export function useAuth(): AuthContextValue {
  const ctx = useContext(AuthContext);
  if (!ctx) {
    throw new Error("useAuth must be used within an AuthProvider");
  }
  return ctx;
}
