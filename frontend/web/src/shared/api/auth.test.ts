import { describe, expect, it } from "vitest";
import { login, readCurrentUser } from "./auth";
import type { Fetcher } from "./client";

describe("auth api", () => {
  it("logs in through the gateway auth envelope", async () => {
    const fetcher: Fetcher = async (_input, init) => {
      expect(init?.method).toBe("POST");
      expect(init?.body).toBe(
        JSON.stringify({
          password: "admin@123",
          username: "admin"
        })
      );

      return new Response(
        JSON.stringify({
          code: 0,
          data: {
            access_token: "stdas-api-client-test-token",
            expires_in_seconds: 28800,
            token_type: "Bearer",
            user: {
              display_name: "STDAS Administrator",
              is_system_manager: true,
              person_code: "admin",
              site_id: "STDAS",
              user_id: "73d29518-9b9d-45c8-a84a-c8df19d9bbd7",
              username: "admin"
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
      login(
        {
          password: "admin@123",
          username: "admin"
        },
        fetcher
      )
    ).resolves.toEqual({
      access_token: "stdas-api-client-test-token",
      expires_in_seconds: 28800,
      token_type: "Bearer",
      user: {
        display_name: "STDAS Administrator",
        is_system_manager: true,
        person_code: "admin",
        site_id: "STDAS",
        user_id: "73d29518-9b9d-45c8-a84a-c8df19d9bbd7",
        username: "admin"
      }
    });
  });

  it("passes bearer token when reading the current user", async () => {
    const fetcher: Fetcher = async (_input, init) => {
      const headers = new Headers(init?.headers);
      expect(headers.get("Authorization")).toBe("Bearer stdas-api-client-test-token");

      return new Response(
        JSON.stringify({
          code: 0,
          data: {
            display_name: "STDAS Administrator",
            is_system_manager: true,
            person_code: "admin",
            site_id: "STDAS",
            user_id: "73d29518-9b9d-45c8-a84a-c8df19d9bbd7",
            username: "admin"
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

    await expect(readCurrentUser("stdas-api-client-test-token", fetcher)).resolves.toEqual({
      display_name: "STDAS Administrator",
      is_system_manager: true,
      person_code: "admin",
      site_id: "STDAS",
      user_id: "73d29518-9b9d-45c8-a84a-c8df19d9bbd7",
      username: "admin"
    });
  });

  it("surfaces gateway auth errors", async () => {
    const fetcher: Fetcher = async () =>
      new Response(
        JSON.stringify({
          code: 40101,
          data: null,
          message: "invalid username, password, or token"
        }),
        {
          headers: {
            "Content-Type": "application/json"
          },
          status: 401
        }
      );

    await expect(
      login(
        {
          password: "wrong",
          username: "admin"
        },
        fetcher
      )
    ).rejects.toThrow("invalid username, password, or token");
  });
});
