use std::collections::BTreeSet;

use super::dto::{
    AppliedLotListQuery, LotListItem, LotListPagination, LotListQuery, LotListResponse,
    LotListSummary,
};

const TEST_SCOPE: &str = "FT (Final Test)";
const DEFAULT_PAGE: u32 = 1;
const DEFAULT_PAGE_SIZE: u32 = 20;
const MAX_PAGE_SIZE: u32 = 50;

pub fn list_lots(query: &LotListQuery) -> LotListResponse {
    let page = query.page.unwrap_or(DEFAULT_PAGE).max(1);
    let page_size = query
        .page_size
        .unwrap_or(DEFAULT_PAGE_SIZE)
        .clamp(1, MAX_PAGE_SIZE);
    let has_active_filter = has_value(&query.cust)
        || has_value(&query.lot_no)
        || has_value(&query.test_step)
        || has_value(&query.tester);

    let filtered = if has_active_filter {
        sample_lots()
            .iter()
            .copied()
            .filter(|lot| matches_query(*lot, query))
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };

    let total = u32::try_from(filtered.len()).unwrap_or(u32::MAX);
    let total_pages = if total == 0 {
        0
    } else {
        total.div_ceil(page_size)
    };
    let start = usize::try_from((page - 1).saturating_mul(page_size)).unwrap_or(usize::MAX);
    let take = usize::try_from(page_size).unwrap_or(usize::MAX);
    let items = filtered
        .iter()
        .skip(start)
        .take(take)
        .copied()
        .collect::<Vec<_>>();

    let matched_customers = filtered
        .iter()
        .map(|lot| lot.cust.to_owned())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    LotListResponse {
        query: AppliedLotListQuery {
            cust: normalized_value(&query.cust),
            lot_no: normalized_value(&query.lot_no),
            test_step: normalized_value(&query.test_step),
            tester: normalized_value(&query.tester),
            test_scope: TEST_SCOPE,
        },
        summary: LotListSummary {
            dataset_state: if has_active_filter {
                "filtered"
            } else {
                "waiting_for_filters"
            },
            query_snapshot_id: has_active_filter.then(|| "QS-FT-20260609-001".to_owned()),
            matched_customers,
            available_lots: u32::try_from(sample_lots().len()).unwrap_or(u32::MAX),
        },
        pagination: LotListPagination {
            page,
            page_size,
            total,
            total_pages,
            has_next_page: total_pages > page,
        },
        items,
    }
}

fn has_value(value: &Option<String>) -> bool {
    value.as_ref().is_some_and(|value| !value.trim().is_empty())
}

fn normalized_value(value: &Option<String>) -> Option<String> {
    value
        .as_ref()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

fn matches_query(lot: LotListItem, query: &LotListQuery) -> bool {
    matches_exact(&query.cust, lot.cust)
        && matches_contains(&query.lot_no, lot.lot_no)
        && matches_exact(&query.test_step, lot.test_step)
        && matches_contains(&query.tester, lot.tester)
}

fn matches_exact(filter: &Option<String>, value: &str) -> bool {
    let Some(filter) = normalized_value(filter) else {
        return true;
    };

    value.eq_ignore_ascii_case(&filter)
}

fn matches_contains(filter: &Option<String>, value: &str) -> bool {
    let Some(filter) = normalized_value(filter) else {
        return true;
    };

    value
        .to_ascii_uppercase()
        .contains(&filter.to_ascii_uppercase())
}

fn sample_lots() -> &'static [LotListItem] {
    &[
        LotListItem {
            lot_id: "LOT-AC25G2907",
            cust: "AC",
            lot_no: "AC25G2907",
            c_lot_no: "C250609-AC01",
            part_no: "A9280-FT",
            external_part_no: "AC-9280-FG",
            test_step: "FT1",
            test_flow: "FLOW-AC-FT-A1",
            test_scope: TEST_SCOPE,
            qty: 12000,
            tested_count: 11982,
            yield_rate: 98.64,
            tester: "STS8200-FT-03",
            handler: "Epson NX-16",
            test_program: "AC9280_FT1_V3.18",
            temperature: "25C",
            start_time: "2026-06-09 08:12",
            end_time: "2026-06-09 09:48",
            status: "Ready",
        },
        LotListItem {
            lot_id: "LOT-AC25G2911",
            cust: "AC",
            lot_no: "AC25G2911",
            c_lot_no: "C250609-AC02",
            part_no: "A9280-FT",
            external_part_no: "AC-9280-FG",
            test_step: "FT1",
            test_flow: "FLOW-AC-FT-A1",
            test_scope: TEST_SCOPE,
            qty: 11800,
            tested_count: 11796,
            yield_rate: 98.51,
            tester: "STS8200-FT-04",
            handler: "Epson NX-16",
            test_program: "AC9280_FT1_V3.18",
            temperature: "25C",
            start_time: "2026-06-09 09:55",
            end_time: "2026-06-09 11:31",
            status: "Ready",
        },
        LotListItem {
            lot_id: "LOT-AC25G2922",
            cust: "AC",
            lot_no: "AC25G2922",
            c_lot_no: "C250609-AC03",
            part_no: "A9280-FT",
            external_part_no: "AC-9280-FG",
            test_step: "FT2",
            test_flow: "FLOW-AC-FT-A2",
            test_scope: TEST_SCOPE,
            qty: 12500,
            tested_count: 12488,
            yield_rate: 97.92,
            tester: "STS8200-FT-03",
            handler: "Epson NX-16",
            test_program: "AC9280_FT2_V2.09",
            temperature: "85C",
            start_time: "2026-06-09 11:42",
            end_time: "2026-06-09 13:18",
            status: "Ready",
        },
        LotListItem {
            lot_id: "LOT-AC25G2938",
            cust: "AC",
            lot_no: "AC25G2938",
            c_lot_no: "C250609-AC04",
            part_no: "A7412-FT",
            external_part_no: "AC-7412-FG",
            test_step: "BI1",
            test_flow: "FLOW-AC-BI-B1",
            test_scope: TEST_SCOPE,
            qty: 8600,
            tested_count: 8584,
            yield_rate: 96.88,
            tester: "STS8200-BI-02",
            handler: "Cohu MATRiX",
            test_program: "AC7412_BI1_V1.27",
            temperature: "125C",
            start_time: "2026-06-09 13:42",
            end_time: "2026-06-09 15:06",
            status: "Ready",
        },
        LotListItem {
            lot_id: "LOT-AC25G2944",
            cust: "AC",
            lot_no: "AC25G2944",
            c_lot_no: "C250609-AC05",
            part_no: "A7412-FT",
            external_part_no: "AC-7412-FG",
            test_step: "SLT1",
            test_flow: "FLOW-AC-SLT-S1",
            test_scope: TEST_SCOPE,
            qty: 7900,
            tested_count: 7890,
            yield_rate: 97.35,
            tester: "SLT-RACK-11",
            handler: "SLT Tray Loader",
            test_program: "AC7412_SLT1_V1.04",
            temperature: "45C",
            start_time: "2026-06-09 15:22",
            end_time: "2026-06-09 16:57",
            status: "Ready",
        },
        LotListItem {
            lot_id: "LOT-BU25F1880",
            cust: "BU",
            lot_no: "BU25F1880",
            c_lot_no: "C250609-BU01",
            part_no: "B3104-FT",
            external_part_no: "BU-3104-FG",
            test_step: "FT1",
            test_flow: "FLOW-BU-FT-A1",
            test_scope: TEST_SCOPE,
            qty: 10120,
            tested_count: 10098,
            yield_rate: 99.02,
            tester: "J750-FT-01",
            handler: "HonTech HT-88",
            test_program: "BU3104_FT1_V4.02",
            temperature: "25C",
            start_time: "2026-06-09 07:58",
            end_time: "2026-06-09 09:10",
            status: "Ready",
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::{list_lots, LotListQuery};

    #[test]
    fn default_query_waits_for_filters() {
        let response = list_lots(&LotListQuery::default());

        assert_eq!(response.summary.dataset_state, "waiting_for_filters");
        assert_eq!(response.pagination.total, 0);
        assert!(response.items.is_empty());
    }

    #[test]
    fn customer_filter_returns_only_final_test_lots() {
        let response = list_lots(&LotListQuery {
            cust: Some("AC".to_owned()),
            page_size: Some(50),
            ..LotListQuery::default()
        });

        assert_eq!(response.summary.dataset_state, "filtered");
        assert_eq!(response.pagination.total, 5);
        assert!(response.items.iter().all(|lot| lot.cust == "AC"));
        assert!(response
            .items
            .iter()
            .all(|lot| lot.test_scope == "FT (Final Test)"));
        assert!(response.items.iter().all(|lot| lot.test_step != "CP1"));
    }
}
