import { describe, expect, it } from "vitest";
import { requestJson, type Fetcher } from "./client";

describe("api client", () => {
  it("reports unavailable gateway when fetch fails before a response is available", async () => {
    const fetcher: Fetcher = async () => {
      throw new Error("connection refused");
    };

    await expect(requestJson("/api/v1/auth/login", {}, fetcher)).rejects.toThrow(
      "Gateway is unavailable: connection refused"
    );
  });

  it("reports empty gateway responses before parsing JSON", async () => {
    const fetcher: Fetcher = async () => new Response("", { status: 502 });

    await expect(requestJson("/api/v1/auth/login", {}, fetcher)).rejects.toThrow(
      "Gateway returned an empty response with HTTP 502. Make sure stdas-gateway is running."
    );
  });

  it("reports non-json gateway responses before envelope validation", async () => {
    const fetcher: Fetcher = async () =>
      new Response("<html>proxy error</html>", { status: 500 });

    await expect(requestJson("/api/v1/auth/login", {}, fetcher)).rejects.toThrow(
      "Gateway returned a non-JSON response with HTTP 500. Make sure stdas-gateway is running."
    );
  });
});
