//! Shared primitives for stable cross-module foundations.
//!
//! Keep this boundary small. Business use cases, SQL access, parser logic and
//! module-private DTOs belong in their owning modules, not here.

mod api;

pub use api::{ApiErrorResponse, ApiResponse};
