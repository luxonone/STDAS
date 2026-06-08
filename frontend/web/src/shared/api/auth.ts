import { requestJson, type Fetcher } from "./client";

export interface LoginCredentials {
  username: string;
  password: string;
}

export interface AuthUser {
  user_id: string;
  username: string;
  display_name: string;
  person_code: string;
  site_id: string;
  is_system_manager: boolean;
}

export interface LoginResult {
  access_token: string;
  token_type: "Bearer";
  expires_in_seconds: number;
  user: AuthUser;
}

export function login(
  credentials: LoginCredentials,
  fetcher?: Fetcher,
  signal?: AbortSignal
): Promise<LoginResult> {
  return requestJson<LoginResult>(
    "/api/v1/auth/login",
    {
      body: credentials,
      method: "POST",
      signal
    },
    fetcher ?? fetch
  );
}

export function readCurrentUser(
  token: string,
  fetcher?: Fetcher,
  signal?: AbortSignal
): Promise<AuthUser> {
  return requestJson<AuthUser>(
    "/api/v1/auth/me",
    {
      signal,
      token
    },
    fetcher ?? fetch
  );
}
