# Changelog

This file records project-visible changes made during the Phase 0 preflight
validation work. It is intentionally concise: implementation details belong in
the code and design rationale belongs in `docs`.

## Commit Numbering

Commit numbers are written in the commit subject because Git commit hashes are
content-addressed and should not be manually shaped.

- `000`: repository baseline or import point.
- `C###`: code or configuration change.
- `D###`: documentation-only change.

Code/configuration commits and documentation commits must stay separate. This
keeps rollback practical: reverting generated code should not automatically
discard accurate documentation, and updating documentation should not hide code
changes in the same commit.

Existing commits before this rule are kept as-is to avoid rewriting local
history. The commit `001 review: align preflight project with best practices`
is treated as `C001` for changelog purposes.

## Unreleased

### D002 - Documentation

- Added this changelog.
- Documented the local commit numbering convention for code/configuration and
  documentation-only commits.
- Linked the changelog from the project README and documentation index.

### C001 - Code and Configuration Review

- Tightened the minimal preflight slice without broad refactoring.
- Made the React root lookup explicit instead of relying on a non-null
  assertion.
- Made the preflight page request flow more abort-aware and easier to maintain.
- Added a backend contract test for `GET /api/v1/system/preflight`.
- Moved frontend build tooling packages to development dependencies.
- Ignored local agent guidance/state and external reference material.

### 000 - Baseline

- Captured the minimal STDAS Phase 0 preflight validation project before the
  best-practice review pass.
