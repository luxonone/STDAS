//! Data pipeline boundary.
//!
//! Owns file registration, raw metadata, parser selection, normalization,
//! `DataVersion` and lineage when trusted data ingestion becomes real
//! implementation work.

pub mod dto;
pub mod handlers;
pub mod routes;
pub mod service;

pub use routes::router;
