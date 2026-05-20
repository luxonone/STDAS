export interface ApiEnvelope<T> {
  code: number;
  message: string;
  data: T;
}

export type Fetcher = (
  input: RequestInfo | URL,
  init?: RequestInit
) => Promise<Response>;

export async function requestJson<T>(
  path: string,
  fetcher: Fetcher = fetch,
  signal?: AbortSignal
): Promise<T> {
  const response = await fetcher(path, {
    headers: {
      Accept: "application/json"
    },
    signal
  });

  if (!response.ok) {
    throw new Error(`Request failed with HTTP ${response.status}`);
  }

  const payload: unknown = await response.json();
  if (!isApiEnvelope(payload)) {
    throw new Error("Gateway returned an invalid response envelope");
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

