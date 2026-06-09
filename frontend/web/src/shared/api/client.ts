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

  let response: Response;
  try {
    response = await fetcher(path, init);
  } catch (error) {
    throw new Error(
      error instanceof Error
        ? `Gateway is unavailable: ${error.message}`
        : "Gateway is unavailable"
    );
  }

  const payload: unknown = await readJsonEnvelope(response);
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

async function readJsonEnvelope(response: Response): Promise<unknown> {
  const text = await response.text();
  if (!text.trim()) {
    throw new Error(
      `Gateway returned an empty response with HTTP ${response.status}. Make sure stdas-gateway is running.`
    );
  }

  try {
    return JSON.parse(text) as unknown;
  } catch {
    throw new Error(
      `Gateway returned a non-JSON response with HTTP ${response.status}. Make sure stdas-gateway is running.`
    );
  }
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
