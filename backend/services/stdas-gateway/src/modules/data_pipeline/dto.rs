use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Default)]
pub struct LotListQuery {
    pub cust: Option<String>,
    pub lot_no: Option<String>,
    pub test_step: Option<String>,
    pub tester: Option<String>,
    pub page: Option<u32>,
    pub page_size: Option<u32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LotListResponse {
    pub query: AppliedLotListQuery,
    pub summary: LotListSummary,
    pub pagination: LotListPagination,
    pub items: Vec<LotListItem>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AppliedLotListQuery {
    pub cust: Option<String>,
    pub lot_no: Option<String>,
    pub test_step: Option<String>,
    pub tester: Option<String>,
    pub test_scope: &'static str,
}

#[derive(Debug, Clone, Serialize)]
pub struct LotListSummary {
    pub dataset_state: &'static str,
    pub query_snapshot_id: Option<String>,
    pub matched_customers: Vec<String>,
    pub available_lots: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct LotListPagination {
    pub page: u32,
    pub page_size: u32,
    pub total: u32,
    pub total_pages: u32,
    pub has_next_page: bool,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct LotListItem {
    pub lot_id: &'static str,
    pub cust: &'static str,
    pub lot_no: &'static str,
    pub c_lot_no: &'static str,
    pub part_no: &'static str,
    pub external_part_no: &'static str,
    pub test_step: &'static str,
    pub test_flow: &'static str,
    pub test_scope: &'static str,
    pub qty: u32,
    pub tested_count: u32,
    pub yield_rate: f32,
    pub tester: &'static str,
    pub handler: &'static str,
    pub test_program: &'static str,
    pub temperature: &'static str,
    pub start_time: &'static str,
    pub end_time: &'static str,
    pub status: &'static str,
}
