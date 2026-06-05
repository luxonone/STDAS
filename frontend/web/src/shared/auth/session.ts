import type { LoginResult } from "../api/auth";

const STORAGE_KEY = "stdas.session";

export interface StoredSession {
  accessToken: string;
  displayName: string;
  tokenType: "Bearer";
  username: string;
}

export function createStoredSession(result: LoginResult): StoredSession {
  return {
    accessToken: result.access_token,
    displayName: result.user.display_name,
    tokenType: result.token_type,
    username: result.user.username
  };
}

export function loadStoredSession(): StoredSession | null {
  const raw = window.localStorage.getItem(STORAGE_KEY);
  if (!raw) {
    return null;
  }

  try {
    const parsed: unknown = JSON.parse(raw);
    if (!isStoredSession(parsed)) {
      clearStoredSession();
      return null;
    }

    return parsed;
  } catch {
    clearStoredSession();
    return null;
  }
}

export function saveStoredSession(session: StoredSession) {
  window.localStorage.setItem(STORAGE_KEY, JSON.stringify(session));
}

export function clearStoredSession() {
  window.localStorage.removeItem(STORAGE_KEY);
}

function isStoredSession(value: unknown): value is StoredSession {
  if (typeof value !== "object" || value === null) {
    return false;
  }

  const record = value as Record<string, unknown>;

  return (
    typeof record.accessToken === "string" &&
    record.tokenType === "Bearer" &&
    typeof record.username === "string" &&
    typeof record.displayName === "string"
  );
}
