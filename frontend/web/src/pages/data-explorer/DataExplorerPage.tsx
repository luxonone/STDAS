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
  testStep: string;
  tester: string;
}

interface ImportDraft {
  cust: string;
  tester: string;
  fileName: string;
  status: "idle" | "ready" | "rejected";
}

const DEFAULT_FILTERS: FilterState = {
  cust: "",
  lotNo: "",
  testStep: "",
  tester: ""
};

const DEFAULT_IMPORT_DRAFT: ImportDraft = {
  cust: "",
  fileName: "",
  status: "idle",
  tester: ""
};

const PAGE_SIZE_OPTIONS = [20, 30, 50];

const COLUMNS: Array<{
  key: keyof LotListItem;
  label: string;
  className?: string;
}> = [
  { key: "cust", label: "Cust.", className: "is-tight" },
  { key: "lot_no", label: "LotNo" },
  { key: "c_lot_no", label: "CLotNo" },
  { key: "part_no", label: "PartNo" },
  { key: "external_part_no", label: "ExternalPartNo" },
  { key: "test_step", label: "TestStep", className: "is-tight" },
  { key: "test_flow", label: "TestFlow" },
  { key: "qty", label: "Qty.", className: "is-number" },
  { key: "tested_count", label: "TestedCount", className: "is-number" },
  { key: "yield_rate", label: "Yield", className: "is-number" },
  { key: "tester", label: "Tester" },
  { key: "handler", label: "Handler" },
  { key: "test_program", label: "TestProgram" },
  { key: "temperature", label: "Temp.", className: "is-tight" },
  { key: "start_time", label: "StartTime" },
  { key: "end_time", label: "EndTime" },
  { key: "status", label: "Status", className: "is-tight" }
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
  const [importDraft, setImportDraft] = useState<ImportDraft>(DEFAULT_IMPORT_DRAFT);

  useEffect(() => {
    const controller = new AbortController();
    const query: LotListQuery = {
      cust: activeFilters.cust,
      lotNo: activeFilters.lotNo,
      page,
      pageSize,
      testStep: activeFilters.testStep,
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
  const allVisibleSelected = lots.length > 0 && lots.every((lot) => selectedLots.has(lot.lot_id));
  const activeFilterCount = Object.values(activeFilters).filter((value) => value.trim()).length;
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

  function applyPageInput() {
    const requestedPage = Number(pageInput);
    if (!Number.isFinite(requestedPage)) {
      setPageInput(String(page));
      return;
    }

    const maxPage = pagination?.total_pages ? pagination.total_pages : 1;
    const nextPage = Math.min(Math.max(Math.trunc(requestedPage), 1), maxPage);
    setErrorMessage(null);
    setPage(nextPage);
    setPageInput(String(nextPage));
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
        <div className="data-explorer__brand">
          <img src="/login-assets/logos/stdas-icon.png" alt="" />
          <div>
            <strong>STDAS</strong>
            <span>Ver 0.1.0</span>
          </div>
        </div>

        <nav className="data-explorer__nav" aria-label="Primary">
          <button className="is-active" type="button">
            Data Explorer
          </button>
          <button type="button">Analysis Workspace</button>
          <button type="button">Alerts &amp; Investigation</button>
          <button type="button">System Admin</button>
        </nav>
      </aside>

      <section className="data-explorer__main">
        <header className="data-explorer__topbar">
          <div>
            <p>Data Explorer / Lot List</p>
            <h1>Lot List</h1>
          </div>

          <div className="data-explorer__top-actions">
            <button
              aria-label="Open notifications"
              className="data-explorer__icon-button"
              type="button"
            >
              3
            </button>
            <span>{session.displayName}</span>
            <button onClick={onSignOut} type="button">
              Sign out
            </button>
          </div>
        </header>

        <section className="data-explorer__content">
          <section className="data-explorer__filters" aria-label="Lot filters">
            <div className="data-explorer__filter-grid">
              <label>
                <span>Cust.</span>
                <input
                  onChange={(event) => updateFilter("cust", event.target.value)}
                  placeholder="AC"
                  value={filters.cust}
                />
              </label>
              <label>
                <span>LotNo</span>
                <input
                  onChange={(event) => updateFilter("lotNo", event.target.value)}
                  placeholder="AC25G2907"
                  value={filters.lotNo}
                />
              </label>
              <label>
                <span>TestStep</span>
                <select
                  onChange={(event) => updateFilter("testStep", event.target.value)}
                  value={filters.testStep}
                >
                  <option value="">FT1 / FT2 / BI1 / SLT1</option>
                  <option value="FT1">FT1</option>
                  <option value="FT2">FT2</option>
                  <option value="BI1">BI1</option>
                  <option value="SLT1">SLT1</option>
                </select>
              </label>
              <label>
                <span>Tester</span>
                <input
                  onChange={(event) => updateFilter("tester", event.target.value)}
                  placeholder="STS8200"
                  value={filters.tester}
                />
              </label>
            </div>

            <div className="data-explorer__filter-actions">
              <div>
                <span>TestScope</span>
                <strong>FT (Final Test)</strong>
              </div>
              <button onClick={clearFilters} type="button">
                Clear
              </button>
              <button onClick={searchLots} type="button">
                Search
              </button>
              <button onClick={() => setIsImportOpen(true)} type="button">
                Import Data
              </button>
            </div>
          </section>

          <section className="data-explorer__toolbar" aria-label="Lot list controls">
            <div>
              <strong>
                {pagination ? pagination.total.toLocaleString() : 0} matched lots
              </strong>
              <span>{activeFilterCount} filters active</span>
            </div>

            <div className="data-explorer__toolbar-right">
              <label>
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
              <label>
                <span>page</span>
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
              <button
                disabled={selectedLots.size === 0}
                onClick={() => undefined}
                type="button"
              >
                Add Selected Lots to Analysis Workspace
              </button>
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
                          {formatCell(lot, column.key)}
                        </td>
                      ))}
                    </tr>
                  ))}
                </tbody>
              </table>

              {lots.length === 0 ? (
                <div className="data-explorer__empty">
                  {lotList === null ? "Loading lots..." : "Set filters and search to load lot data."}
                </div>
              ) : null}
            </div>
          </section>
        </section>
      </section>

      {isImportOpen ? (
        <aside className="data-explorer__import-panel" aria-label="Import data panel">
          <header>
            <div>
              <p>Data import</p>
              <h2>Parser intake</h2>
            </div>
            <button onClick={() => setIsImportOpen(false)} type="button">
              Close
            </button>
          </header>

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

          <section className="data-explorer__import-rule">
            <span>Parser rule</span>
            <strong>{importRule ? importRule.name : "No parser selected"}</strong>
            <p>{importRule ? importRule.description : "Select Cust. and Tester first."}</p>
          </section>

          <label className="data-explorer__file-picker">
            <span>Package</span>
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
      ) : null}
    </main>
  );
}

function formatCell(lot: LotListItem, key: keyof LotListItem) {
  if (key === "yield_rate") {
    return `${lot.yield_rate.toFixed(2)}%`;
  }

  const value = lot[key];
  return typeof value === "number" ? value.toLocaleString() : value;
}

function resolveImportRule(cust: string, tester: string) {
  if (cust.trim().toUpperCase() === "AC" && tester.trim().toUpperCase() === "STS8200") {
    return {
      accept: ".7z",
      description: "AC / STS8200 / FT packages must be staged as 7z archives.",
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
