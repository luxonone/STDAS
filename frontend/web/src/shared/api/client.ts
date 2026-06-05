export interface ApiEnvelope<T> {
  code: number;
  message: string;
  data: T;
}

export type Fetcher = (
  input: RequestInfo | URL,
  init?: RequestInit
) => Promise<Response>;

export interface RequestJsonOptions {
  body?: unknown;
  headers?: HeadersInit;
  method?: string;
  signal?: AbortSignal;
  token?: string;
}

export async function requestJson<T>(
  path: string,
  options: RequestJsonOptions = {},
  fetcher: Fetcher = fetch
): Promise<T> {
  const headers = new Headers(options.headers);
  headers.set("Accept", "application/json");

  const init: RequestInit = {
    headers,
    method: options.method ?? "GET",
    signal: options.signal
  };

  if (options.token) {
    headers.set("Authorization", `Bearer ${options.token}`);
  }

  if (options.body !== undefined) {
    headers.set("Content-Type", "application/json");
    init.body = JSON.stringify(options.body);
  }

  const response = await fetcher(path, init);

  const payload: unknown = await response.json();
  if (!isApiEnvelope(payload)) {
    throw new Error("Gateway returned an invalid response envelope");
  }

  if (!response.ok) {
    throw new Error(payload.message || `Request failed with HTTP ${response.status}`);
  }

  if (payload.code !== 0) {
    throw new Error(payload.message || "Gateway returned an error");
  }

  return payload.data as T;
}

function isApiEnvelope(payload: unknown): payload is ApiEnvelope<unknown> {
  if (typeof payload !== "object" || payload === null) {
    return false;
  }

  const record = payload as Record<string, unknown>;

  return (
    typeof record.code === "number" &&
    typeof record.message === "string" &&
    "data" in record
  );
}
