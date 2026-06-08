import { requestJson, type Fetcher } from "./client";

export interface LotListQuery {
  cust?: string;
  lotNo?: string;
  testStep?: string;
  tester?: string;
  page?: number;
  pageSize?: number;
}

export interface LotListResponse {
  query: AppliedLotListQuery;
  summary: LotListSummary;
  pagination: LotListPagination;
  items: LotListItem[];
}

export interface AppliedLotListQuery {
  cust: string | null;
  lot_no: string | null;
  test_step: string | null;
  tester: string | null;
  test_scope: string;
}

export interface LotListSummary {
  dataset_state: "waiting_for_filters" | "filtered";
  query_snapshot_id: string | null;
  matched_customers: string[];
  available_lots: number;
}

export interface LotListPagination {
  page: number;
  page_size: number;
  total: number;
  total_pages: number;
  has_next_page: boolean;
}

export interface LotListItem {
  lot_id: string;
  cust: string;
  lot_no: string;
  c_lot_no: string;
  part_no: string;
  external_part_no: string;
  test_step: string;
  test_flow: string;
  test_scope: string;
  qty: number;
  tested_count: number;
  yield_rate: number;
  tester: string;
  handler: string;
  test_program: string;
  temperature: string;
  start_time: string;
  end_time: string;
  status: string;
}

export function readLotList(
  token: string,
  query: LotListQuery = {},
  fetcher?: Fetcher,
  signal?: AbortSignal
): Promise<LotListResponse> {
  const params = new URLSearchParams();
  appendTextParam(params, "cust", query.cust);
  appendTextParam(params, "lot_no", query.lotNo);
  appendTextParam(params, "test_step", query.testStep);
  appendTextParam(params, "tester", query.tester);
  appendNumberParam(params, "page", query.page);
  appendNumberParam(params, "page_size", query.pageSize);

  const queryString = params.toString();
  const path = queryString ? `/api/v1/data/lots?${queryString}` : "/api/v1/data/lots";

  return requestJson<LotListResponse>(
    path,
    {
      signal,
      token
    },
    fetcher ?? fetch
  );
}

function appendTextParam(params: URLSearchParams, key: string, value?: string) {
  const normalized = value?.trim();
  if (normalized) {
    params.set(key, normalized);
  }
}

function appendNumberParam(params: URLSearchParams, key: string, value?: number) {
  if (typeof value === "number" && Number.isFinite(value)) {
    params.set(key, String(value));
  }
}
