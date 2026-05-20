import { describe, expect, it } from "vitest";
import { readSystemHealth } from "./system";
import type { Fetcher } from "./client";

describe("readSystemHealth", () => {
  it("reads the gateway health data from the standard envelope", async () => {
    const fetcher: Fetcher = async () =>
      new Response(
        JSON.stringify({
          code: 0,
          message: "success",
          data: {
            service: "stdas-gateway",
            status: "ok"
          }
        }),
        {
          headers: {
            "Content-Type": "application/json"
          },
          status: 200
        }
      );

    await expect(readSystemHealth(fetcher)).resolves.toEqual({
      service: "stdas-gateway",
      status: "ok"
    });
  });
});

