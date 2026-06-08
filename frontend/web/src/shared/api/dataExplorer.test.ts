import { describe, expect, it } from "vitest";
import { readLotList } from "./dataExplorer";
import type { Fetcher } from "./client";

describe("data explorer api", () => {
  it("reads lot list through the authenticated gateway envelope", async () => {
    const fetcher: Fetcher = async (input, init) => {
      expect(String(input)).toBe(
        "/api/v1/data/lots?cust=AC&lot_no=AC25G2907&test_step=FT1&tester=STS8200&page=1&page_size=20"
      );

      const headers = new Headers(init?.headers);
      expect(headers.get("Authorization")).toBe("Bearer stdas-api-client-test-token");

      return new Response(
        JSON.stringify({
          code: 0,
          data: {
            items: [
              {
                c_lot_no: "C250609-AC01",
                cust: "AC",
                end_time: "2026-06-09 09:48",
                external_part_no: "AC-9280-FG",
                handler: "Epson NX-16",
                lot_id: "LOT-AC25G2907",
                lot_no: "AC25G2907",
                part_no: "A9280-FT",
                qty: 12000,
                start_time: "2026-06-09 08:12",
                status: "Ready",
                temperature: "25C",
                test_flow: "FLOW-AC-FT-A1",
                test_program: "AC9280_FT1_V3.18",
                test_scope: "FT (Final Test)",
                test_step: "FT1",
                tested_count: 11982,
                tester: "STS8200-FT-03",
                yield_rate: 98.64
              }
            ],
            pagination: {
              has_next_page: false,
              page: 1,
              page_size: 20,
              total: 1,
              total_pages: 1
            },
            query: {
              cust: "AC",
              lot_no: "AC25G2907",
              test_scope: "FT (Final Test)",
              test_step: "FT1",
              tester: "STS8200"
            },
            summary: {
              available_lots: 6,
              dataset_state: "filtered",
              matched_customers: ["AC"],
              query_snapshot_id: "QS-FT-20260609-001"
            }
          },
          message: "success"
        }),
        {
          headers: {
            "Content-Type": "application/json"
          },
          status: 200
        }
      );
    };

    await expect(
      readLotList(
        "stdas-api-client-test-token",
        {
          cust: " AC ",
          lotNo: "AC25G2907",
          page: 1,
          pageSize: 20,
          testStep: "FT1",
          tester: "STS8200"
        },
        fetcher
      )
    ).resolves.toMatchObject({
      items: [
        {
          lot_no: "AC25G2907",
          test_scope: "FT (Final Test)",
          test_step: "FT1"
        }
      ],
      pagination: {
        total: 1
      }
    });
  });
});
