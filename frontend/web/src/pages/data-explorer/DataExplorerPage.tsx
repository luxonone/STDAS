import { useEffect, useMemo, useState } from "react";
import {
  readLotList,
  type LotListItem,
  type LotListQuery,
  type LotListResponse
} from "../../shared/api";
import type { StoredSession } from "../../shared/auth/session";

interface DataExplorerPageProps {
  onSignOut: () => void;
  session: StoredSession;
}

interface FilterState {
  cust: string;
  lotNo: string;
  cLotNo: string;
  partNo: string;
  externalPartNo: string;
  step: string;
  flow: string;
  status: string;
  tester: string;
  handler: string;
  program: string;
  endTime: string;
  yieldRange: string;
}

interface ImportDraft {
  cust: string;
  tester: string;
  fileName: string;
  status: "idle" | "ready" | "rejected";
}

interface FilterField {
  key: keyof FilterState;
  label: string;
  placeholder: string;
  options?: string[];
}

const DEFAULT_FILTERS: FilterState = {
  cLotNo: "",
  cust: "",
  endTime: "",
  externalPartNo: "",
  flow: "",
  handler: "",
  lotNo: "",
  partNo: "",
  program: "",
  status: "",
  step: "",
  tester: "",
  yieldRange: ""
};

const DEFAULT_IMPORT_DRAFT: ImportDraft = {
  cust: "",
  fileName: "",
  status: "idle",
  tester: ""
};

const PAGE_SIZE_OPTIONS = [20, 30, 50];

const FILTER_FIELDS: FilterField[] = [
  { key: "cust", label: "Cust.", placeholder: "e.g. AC" },
  { key: "lotNo", label: "LotNo", placeholder: "e.g. LOT25G29" },
  { key: "cLotNo", label: "CLotNo", placeholder: "e.g. CS-129" },
  { key: "partNo", label: "PartNo", placeholder: "e.g. AX112" },
  { key: "externalPartNo", label: "ExternalPartNo", placeholder: "e.g. EXT-A112" },
  { key: "step", label: "Step", placeholder: "FT1", options: ["FT1", "FT2", "BI1", "SLT1"] },
  { key: "flow", label: "Flow", placeholder: "1st Test" },
  { key: "status", label: "Status", placeholder: "e.g. Ready" },
  { key: "tester", label: "Tester", placeholder: "e.g. TS1-01" },
  { key: "handler", label: "Handler/Prober", placeholder: "e.g. HRD-01" },
  { key: "program", label: "Program", placeholder: "e.g. FT_A112" },
  { key: "endTime", label: "EndTime", placeholder: "e.g. Last 24h" },
  { key: "yieldRange", label: "Yield", placeholder: "e.g. 88 - 96" }
];

const COLUMNS: Array<{
  key: keyof LotListItem;
  label: string;
  className?: string;
}> = [
  { key: "cust", label: "Cust.", className: "is-cust" },
  { key: "lot_no", label: "LotNo" },
  { key: "c_lot_no", label: "CLotNo" },
  { key: "part_no", label: "PartNo" },
  { key: "external_part_no", label: "ExternalPartNo" },
  { key: "test_step", label: "TestStep", className: "is-step" },
  { key: "test_flow", label: "TestFlow" },
  { key: "qty", label: "Qty.", className: "is-number" },
  { key: "tested_count", label: "TestedCount", className: "is-number" },
  { key: "yield_rate", label: "Yield", className: "is-number" },
  { key: "tester", label: "Tester" },
  { key: "handler", label: "Handler/Prober" },
  { key: "test_program", label: "TestProgram" },
  { key: "temperature", label: "Temp.", className: "is-temp" },
  { key: "start_time", label: "StartTime" },
  { key: "end_time", label: "EndTime" },
  { key: "status", label: "LotStatus", className: "is-status" }
];

const NAV_ITEMS = [
  { icon: "grid", label: "Data Explorer", active: true },
  { icon: "trace", label: "Analysis Workspace", active: false },
  { icon: "alert", label: "Alerts & Investigation", active: false },
  { icon: "download", label: "Jobs & Exports", active: false },
  { icon: "shield", label: "System Governance", active: false }
];

export function DataExplorerPage({
  onSignOut,
  session
}: DataExplorerPageProps) {
  const [filters, setFilters] = useState<FilterState>(DEFAULT_FILTERS);
  const [activeFilters, setActiveFilters] = useState<FilterState>(DEFAULT_FILTERS);
  const [page, setPage] = useState(1);
  const [pageInput, setPageInput] = useState("1");
  const [pageSize, setPageSize] = useState(20);
  const [selectedLots, setSelectedLots] = useState<Set<string>>(() => new Set());
  const [lotList, setLotList] = useState<LotListResponse | null>(null);
  const [errorMessage, setErrorMessage] = useState<string | null>(null);
  const [isImportOpen, setIsImportOpen] = useState(false);
  const [isNotificationOpen, setIsNotificationOpen] = useState(false);
  const [importDraft, setImportDraft] = useState<ImportDraft>(DEFAULT_IMPORT_DRAFT);

  useEffect(() => {
    const controller = new AbortController();
    const query: LotListQuery = {
      cust: activeFilters.cust,
      lotNo: activeFilters.lotNo,
      page,
      pageSize,
      testStep: activeFilters.step,
      tester: activeFilters.tester
    };

    void readLotList(session.accessToken, query, fetch, controller.signal)
      .then((response) => {
        if (controller.signal.aborted) {
          return;
        }

        setErrorMessage(null);
        setLotList(response);
        setSelectedLots((current) => {
          const visibleIds = new Set(response.items.map((lot) => lot.lot_id));
          return new Set([...current].filter((lotId) => visibleIds.has(lotId)));
        });
      })
      .catch((error: unknown) => {
        if (controller.signal.aborted) {
          return;
        }

        setErrorMessage(error instanceof Error ? error.message : "Lot list request failed");
      });

    return () => controller.abort();
  }, [activeFilters, page, pageSize, session.accessToken]);

  const lots = lotList?.items ?? [];
  const pagination = lotList?.pagination;
  const total = pagination?.total ?? 0;
  const totalPages = pagination?.total_pages ?? 0;
  const selectedCount = selectedLots.size;
  const allVisibleSelected = lots.length > 0 && lots.every((lot) => selectedLots.has(lot.lot_id));
  const rangeLabel = useMemo(() => buildRangeLabel(page, pageSize, total), [page, pageSize, total]);
  const pageButtons = useMemo(() => buildPageButtons(page, totalPages), [page, totalPages]);
  const importRule = useMemo(
    () => resolveImportRule(importDraft.cust, importDraft.tester),
    [importDraft.cust, importDraft.tester]
  );

  function updateFilter(key: keyof FilterState, value: string) {
    setFilters((current) => ({
      ...current,
      [key]: value
    }));
  }

  function searchLots() {
    setErrorMessage(null);
    setActiveFilters(filters);
    setPage(1);
    setPageInput("1");
  }

  function clearFilters() {
    setErrorMessage(null);
    setFilters(DEFAULT_FILTERS);
    setActiveFilters(DEFAULT_FILTERS);
    setPage(1);
    setPageInput("1");
  }

  function toggleVisibleLots(checked: boolean) {
    setSelectedLots((current) => {
      const next = new Set(current);
      for (const lot of lots) {
        if (checked) {
          next.add(lot.lot_id);
        } else {
          next.delete(lot.lot_id);
        }
      }

      return next;
    });
  }

  function toggleLot(lotId: string, checked: boolean) {
    setSelectedLots((current) => {
      const next = new Set(current);
      if (checked) {
        next.add(lotId);
      } else {
        next.delete(lotId);
      }

      return next;
    });
  }

  function changePageSize(value: number) {
    setErrorMessage(null);
    setPageSize(value);
    setPage(1);
    setPageInput("1");
  }

  function changePage(nextPage: number) {
    const maxPage = Math.max(totalPages, 1);
    const normalizedPage = Math.min(Math.max(nextPage, 1), maxPage);
    setErrorMessage(null);
    setPage(normalizedPage);
    setPageInput(String(normalizedPage));
  }

  function applyPageInput() {
    const requestedPage = Number(pageInput);
    if (!Number.isFinite(requestedPage)) {
      setPageInput(String(page));
      return;
    }

    changePage(Math.trunc(requestedPage));
  }

  function updateImportDraft(key: keyof ImportDraft, value: string) {
    setImportDraft((current) => ({
      ...current,
      [key]: value,
      status: key === "fileName" ? current.status : "idle"
    }));
  }

  function handleImportFile(file: File | null) {
    if (!file) {
      setImportDraft((current) => ({
        ...current,
        fileName: "",
        status: "idle"
      }));
      return;
    }

    setImportDraft((current) => ({
      ...current,
      fileName: file.name,
      status: file.name.toLowerCase().endsWith(".7z") && importRule ? "ready" : "rejected"
    }));
  }

  return (
    <main className="data-explorer" aria-label="STDAS Data Explorer">
      <aside className="data-explorer__sidebar" aria-label="Global navigation">
        <div className="data-explorer__brand" aria-label="STDAS version 0.1.0">
          <div className="data-explorer__brand-mark">
            <img src="/login-assets/logos/stdas-icon.png" alt="" />
          </div>
          <div className="data-explorer__brand-copy">
            <div className="data-explorer__brand-row">
              <strong>STDAS</strong>
              <span>Ver 0.1.0</span>
            </div>
            <small>Test Data Analytics System</small>
            <i aria-hidden="true" />
          </div>
        </div>

        <nav className="data-explorer__nav" aria-label="Primary">
          {NAV_ITEMS.map((item) => (
            <button className={item.active ? "is-active" : undefined} key={item.label} type="button">
              <span className={`data-explorer__nav-icon is-${item.icon}`} aria-hidden="true" />
              <span>{item.label}</span>
            </button>
          ))}
        </nav>

        <div className="data-explorer__collapse">
          <button type="button">
            <span aria-hidden="true">{"<<"}</span>
            Collapse
          </button>
        </div>
      </aside>

      <section className="data-explorer__workspace">
        <header className="data-explorer__global-topbar" aria-label="Global context">
          <section className="data-explorer__context-segment" aria-label="Test scope">
            <div>
              <span>Test Scope</span>
              <strong>FT (Final Test)</strong>
            </div>
            <button aria-label="Change test scope filters" type="button">
              <span className="data-explorer__scope-icon" aria-hidden="true" />
            </button>
          </section>

          <div className="data-explorer__topbar-filler" />

          <button
            aria-expanded={isNotificationOpen}
            aria-label="Open notifications"
            className="data-explorer__notification-button"
            onClick={() => setIsNotificationOpen((open) => !open)}
            type="button"
          >
            <span className="data-explorer__bell" aria-hidden="true" />
            <span className="data-explorer__badge">12</span>
          </button>

          <section className="data-explorer__job-segment" aria-label="Job queue">
            <span>Job Queue</span>
            <div>
              <i aria-hidden="true" />
              <strong>3 / 7</strong>
            </div>
          </section>

          <section className="data-explorer__user-segment" aria-label="Current user">
            <div className="data-explorer__avatar" aria-hidden="true">
              TE
            </div>
            <div>
              <strong>{session.displayName || "Test Engineer"}</strong>
              <span>Test Eng. Dept</span>
            </div>
            <button aria-label="Sign out" onClick={onSignOut} title="Sign out" type="button">
              ˅
            </button>
          </section>
        </header>

        {isNotificationOpen ? (
          <aside className="data-explorer__notification-popover" aria-label="Notifications">
            <header>
              <strong>Notifications</strong>
              <button onClick={() => setIsNotificationOpen(false)} type="button">
                Close
              </button>
            </header>
            <ul>
              <li>
                <span>Import parser completed</span>
                <strong>AC-STS8200-FT package ready</strong>
              </li>
              <li>
                <span>Analysis workspace</span>
                <strong>1 selected lot is waiting for review</strong>
              </li>
              <li>
                <span>System</span>
                <strong>3 export jobs remain in queue</strong>
              </li>
            </ul>
          </aside>
        ) : null}

        <section className="data-explorer__content" aria-label="Lot list workspace">
          <h1>Data Explorer / Lot List</h1>

          <section className="data-explorer__filters" aria-label="Lot filters">
            <div className="data-explorer__filter-grid">
              {FILTER_FIELDS.map((field) => (
                <label key={field.key}>
                  <span>{field.label}</span>
                  {field.options ? (
                    <select
                      onChange={(event) => updateFilter(field.key, event.target.value)}
                      value={filters[field.key]}
                    >
                      <option value="">{field.placeholder}</option>
                      {field.options.map((option) => (
                        <option key={option} value={option}>
                          {option}
                        </option>
                      ))}
                    </select>
                  ) : (
                    <input
                      onChange={(event) => updateFilter(field.key, event.target.value)}
                      placeholder={field.placeholder}
                      value={filters[field.key]}
                    />
                  )}
                </label>
              ))}
            </div>

            <div className="data-explorer__filter-actions">
              <button className="is-primary" onClick={searchLots} type="button">
                <span aria-hidden="true">⌕</span>
                Search
              </button>
              <button onClick={clearFilters} type="button">
                <span aria-hidden="true">↻</span>
                Reset
              </button>
              <button type="button">
                <span aria-hidden="true">□</span>
                Save Filter
              </button>
              <button type="button">
                <span aria-hidden="true">▥</span>
                Columns
              </button>
              <button type="button">
                <span aria-hidden="true">⇧</span>
                Export
              </button>
              <button className="is-primary" onClick={() => setIsImportOpen(true)} type="button">
                <span aria-hidden="true">⇧</span>
                Import Data
              </button>
            </div>
          </section>

          <section className="data-explorer__toolbar" aria-label="Lot list controls">
            <div className="data-explorer__toolbar-left">
              <div className="data-explorer__metric">
                <span>Matched</span>
                <strong>{total.toLocaleString()}</strong>
              </div>
              <div className="data-explorer__metric">
                <span>Selected</span>
                <strong>{selectedCount.toLocaleString()} lots</strong>
              </div>
              <button disabled={selectedCount === 0} type="button">
                <span aria-hidden="true">+</span>
                Add to Analysis Workspace
              </button>
            </div>

            <div className="data-explorer__toolbar-right">
              <label className="data-explorer__per-page">
                <span>per page</span>
                <select
                  onChange={(event) => changePageSize(Number(event.target.value))}
                  value={pageSize}
                >
                  {PAGE_SIZE_OPTIONS.map((option) => (
                    <option key={option} value={option}>
                      {option}
                    </option>
                  ))}
                </select>
              </label>
              <strong>{rangeLabel}</strong>
              <label className="data-explorer__page-input">
                <span>Page</span>
                <input
                  inputMode="numeric"
                  onBlur={applyPageInput}
                  onChange={(event) => setPageInput(event.target.value)}
                  onKeyDown={(event) => {
                    if (event.key === "Enter") {
                      applyPageInput();
                    }
                  }}
                  value={pageInput}
                />
              </label>
              <nav aria-label="Pagination">
                {pageButtons.map((button) =>
                  button === "gap" ? (
                    <span key="gap">...</span>
                  ) : (
                    <button
                      className={button === page ? "is-current" : undefined}
                      disabled={totalPages === 0}
                      key={button}
                      onClick={() => changePage(button)}
                      type="button"
                    >
                      {button}
                    </button>
                  )
                )}
                <button disabled={!pagination?.has_next_page} onClick={() => changePage(page + 1)} type="button">
                  ›
                </button>
              </nav>
            </div>
          </section>

          <section className="data-explorer__table-shell" aria-label="Lot list table">
            {errorMessage ? <div className="data-explorer__error">{errorMessage}</div> : null}
            <div className="data-explorer__table-scroll">
              <table>
                <thead>
                  <tr>
                    <th className="is-select">
                      <input
                        aria-label="Select visible lots"
                        checked={allVisibleSelected}
                        disabled={lots.length === 0}
                        onChange={(event) => toggleVisibleLots(event.target.checked)}
                        type="checkbox"
                      />
                    </th>
                    {COLUMNS.map((column) => (
                      <th className={column.className} key={column.key}>
                        {column.label}
                      </th>
                    ))}
                  </tr>
                </thead>
                <tbody>
                  {lots.map((lot) => (
                    <tr key={lot.lot_id}>
                      <td className="is-select">
                        <input
                          aria-label={`Select ${lot.lot_no}`}
                          checked={selectedLots.has(lot.lot_id)}
                          onChange={(event) => toggleLot(lot.lot_id, event.target.checked)}
                          type="checkbox"
                        />
                      </td>
                      {COLUMNS.map((column) => (
                        <td className={column.className} key={column.key}>
                          {renderCell(lot, column.key)}
                        </td>
                      ))}
                    </tr>
                  ))}
                </tbody>
              </table>

              {lots.length === 0 && lotList === null ? (
                <div className="data-explorer__empty">Loading lots...</div>
              ) : null}
            </div>
            <span className="data-explorer__table-continuation" aria-hidden="true" />
          </section>
        </section>
      </section>

      {isImportOpen ? (
        <>
          <button
            aria-label="Close import data panel"
            className="data-explorer__import-backdrop"
            onClick={() => setIsImportOpen(false)}
            type="button"
          />
          <aside className="data-explorer__import-panel" aria-label="Import data panel">
            <header>
              <div>
                <p>Import Data</p>
                <h2>Parser intake</h2>
              </div>
              <button onClick={() => setIsImportOpen(false)} type="button">
                Close
              </button>
            </header>

            <section className="data-explorer__import-fields">
              <label>
                <span>Cust.</span>
                <input
                  onChange={(event) => updateImportDraft("cust", event.target.value)}
                  placeholder="AC"
                  value={importDraft.cust}
                />
              </label>
              <label>
                <span>Tester</span>
                <input
                  onChange={(event) => updateImportDraft("tester", event.target.value)}
                  placeholder="STS8200"
                  value={importDraft.tester}
                />
              </label>
              <label>
                <span>TestScope</span>
                <input readOnly value="FT (Final Test)" />
              </label>
            </section>

            <section className="data-explorer__import-rule">
              <span>Import parser rule</span>
              <strong>{importRule ? importRule.name : "Waiting for Cust. and Tester"}</strong>
              <p>
                {importRule
                  ? importRule.description
                  : "Parser restrictions are resolved before file selection. Current scope only accepts FT data."}
              </p>
            </section>

            <label className="data-explorer__file-picker">
              <span>Compressed data package</span>
              <input
                accept={importRule?.accept ?? ".7z"}
                disabled={!importRule}
                onChange={(event) => handleImportFile(event.target.files?.item(0) ?? null)}
                type="file"
              />
            </label>

            <div className={`data-explorer__import-status is-${importDraft.status}`}>
              {importStatusText(importDraft)}
            </div>
          </aside>
        </>
      ) : null}
    </main>
  );
}

function buildRangeLabel(page: number, pageSize: number, total: number) {
  if (total === 0) {
    return "0-0 of 0";
  }

  const start = (page - 1) * pageSize + 1;
  const end = Math.min(page * pageSize, total);
  return `${start}-${end} of ${total.toLocaleString()}`;
}

function buildPageButtons(page: number, totalPages: number) {
  if (totalPages <= 0) {
    return [1, 2, 3, 4, 5] as Array<number | "gap">;
  }

  if (totalPages <= 6) {
    return Array.from({ length: totalPages }, (_, index) => index + 1);
  }

  const buttons = new Set([1, page - 1, page, page + 1, totalPages]);
  const normalized = [...buttons]
    .filter((button) => button >= 1 && button <= totalPages)
    .sort((left, right) => left - right);
  const result: Array<number | "gap"> = [];

  normalized.forEach((button, index) => {
    if (index > 0 && button - normalized[index - 1] > 1) {
      result.push("gap");
    }
    result.push(button);
  });

  return result;
}

function renderCell(lot: LotListItem, key: keyof LotListItem) {
  if (key === "yield_rate") {
    return <span className={yieldClassName(lot.yield_rate)}>{lot.yield_rate.toFixed(2)}</span>;
  }

  if (key === "status") {
    return <span className={statusClassName(lot.status)}>{lot.status}</span>;
  }

  const value = lot[key];
  return typeof value === "number" ? value.toLocaleString() : value;
}

function yieldClassName(yieldRate: number) {
  if (yieldRate >= 95) {
    return "data-explorer__yield is-good";
  }

  if (yieldRate >= 90) {
    return "data-explorer__yield is-watch";
  }

  return "data-explorer__yield is-risk";
}

function statusClassName(status: string) {
  const normalized = status.toLowerCase();
  if (normalized.includes("ready") || normalized.includes("grant")) {
    return "data-explorer__status is-ready";
  }

  if (normalized.includes("partial")) {
    return "data-explorer__status is-partial";
  }

  if (normalized.includes("hold")) {
    return "data-explorer__status is-hold";
  }

  return "data-explorer__status is-neutral";
}

function resolveImportRule(cust: string, tester: string) {
  if (cust.trim().toUpperCase() === "AC" && tester.trim().toUpperCase() === "STS8200") {
    return {
      accept: ".7z",
      description:
        "AC / STS8200 parser accepts FT scope only and restricts package selection to 7z archives.",
      name: "AC-STS8200-FT"
    };
  }

  return null;
}

function importStatusText(draft: ImportDraft) {
  if (draft.status === "ready") {
    return `${draft.fileName} is ready for parser queue.`;
  }

  if (draft.status === "rejected") {
    return `${draft.fileName} was rejected by the selected parser rule.`;
  }

  return "Choose a matching package after parser rule is available.";
}
