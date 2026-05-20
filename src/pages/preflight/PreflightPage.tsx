import { useCallback, useEffect, useState } from "react";
import { readSystemHealth, type SystemHealth } from "../../shared/api";

type LoadState =
  | { kind: "loading" }
  | { kind: "ready"; health: SystemHealth }
  | { kind: "error"; message: string };

const checks = [
  { label: "Frontend", value: "Vite React TypeScript" },
  { label: "API route", value: "/api/v1/system/health" },
  { label: "Scope", value: "Phase 0 verification only" }
] as const;

export function PreflightPage() {
  const [state, setState] = useState<LoadState>({ kind: "loading" });

  const loadHealth = useCallback(async (signal?: AbortSignal) => {
    setState({ kind: "loading" });

    try {
      const health = await readSystemHealth(fetch, signal);
      setState({ kind: "ready", health });
    } catch (error) {
      const message =
        error instanceof Error ? error.message : "Unable to reach gateway";
      setState({ kind: "error", message });
    }
  }, []);

  useEffect(() => {
    const controller = new AbortController();

    readSystemHealth(fetch, controller.signal)
      .then((health) => {
        setState({ kind: "ready", health });
      })
      .catch((error) => {
        if (controller.signal.aborted) {
          return;
        }

        const message =
          error instanceof Error ? error.message : "Unable to reach gateway";
        setState({ kind: "error", message });
      });

    return () => controller.abort();
  }, []);

  const statusText =
    state.kind === "ready"
      ? "Gateway online"
      : state.kind === "error"
        ? "Gateway unavailable"
        : "Checking gateway";

  return (
    <div className="workbench">
      <aside className="rail" aria-label="Preflight sections">
        <div className="brand">
          <h1 className="brand__name">STDAS</h1>
          <span className="brand__meta">Minimal verification workbench</span>
        </div>
        <ul className="nav-list">
          <li className="nav-item">
            Runtime
            <span className="nav-item__dot nav-item__dot--ready" />
          </li>
          <li className="nav-item">
            Gateway
            <span
              className={
                state.kind === "ready"
                  ? "nav-item__dot nav-item__dot--ready"
                  : "nav-item__dot nav-item__dot--warn"
              }
            />
          </li>
          <li className="nav-item">
            Contracts
            <span className="nav-item__dot" />
          </li>
        </ul>
      </aside>

      <main className="main">
        <header className="topbar">
          <div>
            <p className="eyebrow">Phase 0 preparation</p>
            <h2 className="title">STDAS Preflight</h2>
            <p className="subtitle">
              This screen validates that the documented frontend and backend
              frameworks can compile, run, and communicate before formal
              feature development starts.
            </p>
          </div>
          <button
            className="refresh-button"
            disabled={state.kind === "loading"}
            onClick={() => void loadHealth()}
            type="button"
          >
            Refresh
          </button>
        </header>

        <section className="panel-grid" aria-label="Preflight status panels">
          <article className="panel" data-testid="gateway-panel">
            <p className="panel__label">Gateway status</p>
            <p className="panel__value" data-testid="gateway-status">
              {statusText}
            </p>
            <p className="panel__copy">
              {state.kind === "ready"
                ? `${state.health.service} reported ${state.health.status}.`
                : state.kind === "error"
                  ? state.message
                  : "Waiting for the Axum gateway health response."}
            </p>
            <div className="status-line">
              <span
                className={
                  state.kind === "ready"
                    ? "status-line__dot status-line__dot--ok"
                    : state.kind === "error"
                      ? "status-line__dot status-line__dot--error"
                      : "status-line__dot"
                }
              />
              <span>{state.kind === "ready" ? "Connected" : "Pending"}</span>
            </div>
          </article>

          <article className="panel">
            <p className="panel__label">Frontend runtime</p>
            <p className="panel__value">React + Vite</p>
            <p className="panel__copy">
              The page is structured as an analytical workbench shell with
              shared API access kept under src/shared/api.
            </p>
          </article>

          <article className="panel">
            <p className="panel__label">Backend runtime</p>
            <p className="panel__value">Rust + Axum</p>
            <p className="panel__copy">
              The gateway exposes a stable response envelope under the
              documented /api/v1 prefix.
            </p>
          </article>
        </section>

        <ul className="checklist" aria-label="Verification checklist">
          {checks.map((check) => (
            <li className="checklist__item" key={check.label}>
              <span className="checklist__key">{check.label}</span>
              <span>{check.value}</span>
            </li>
          ))}
        </ul>
      </main>
    </div>
  );
}
