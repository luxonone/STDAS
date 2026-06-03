import { requestJson, type Fetcher } from "./client";

export interface SystemHealth {
  service: string;
  status: string;
}

export function readSystemHealth(
  fetcher?: Fetcher,
  signal?: AbortSignal
): Promise<SystemHealth> {
  return requestJson<SystemHealth>(
    "/api/v1/system/health",
    fetcher ?? fetch,
    signal
  );
}

