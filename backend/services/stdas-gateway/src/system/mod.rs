//! Operational system endpoints.
//!
//! This module owns phase-0 health and preflight routes. It is intentionally
//! separate from `modules/` because it is not a future business service
//! boundary.

mod dto;
mod handlers;
mod models;
mod routes;
mod service;

pub use routes::router;
