import { useEffect, useState } from "react";
import { BlankWorkspacePage } from "../pages/blank-workspace";
import { LoginPage } from "../pages/login";
import { readCurrentUser, type LoginResult } from "../shared/api";
import {
  clearStoredSession,
  createStoredSession,
  loadStoredSession,
  saveStoredSession,
  type StoredSession
} from "../shared/auth/session";

type SessionState =
  | { kind: "checking"; session: StoredSession }
  | { kind: "anonymous" }
  | { kind: "authenticated"; session: StoredSession };

export function App() {
  const [sessionState, setSessionState] = useState<SessionState>(() => {
    const session = loadStoredSession();

    return session ? { kind: "checking", session } : { kind: "anonymous" };
  });

  useEffect(() => {
    if (sessionState.kind !== "checking") {
      return undefined;
    }

    const controller = new AbortController();

    void readCurrentUser(sessionState.session.accessToken, fetch, controller.signal)
      .then((user) => {
        if (controller.signal.aborted) {
          return;
        }

        const verifiedSession = {
          ...sessionState.session,
          displayName: user.display_name,
          username: user.username
        };
        saveStoredSession(verifiedSession);
        setSessionState({ kind: "authenticated", session: verifiedSession });
      })
      .catch(() => {
        if (controller.signal.aborted) {
          return;
        }

        clearStoredSession();
        setSessionState({ kind: "anonymous" });
      });

    return () => controller.abort();
  }, [sessionState]);

  function handleAuthenticated(result: LoginResult) {
    const session = createStoredSession(result);
    saveStoredSession(session);
    setSessionState({ kind: "authenticated", session });
  }

  function handleSignOut() {
    clearStoredSession();
    setSessionState({ kind: "anonymous" });
  }

  if (sessionState.kind === "authenticated") {
    return (
      <BlankWorkspacePage
        onSignOut={handleSignOut}
        session={sessionState.session}
      />
    );
  }

  return <LoginPage onAuthenticated={handleAuthenticated} />;
}
