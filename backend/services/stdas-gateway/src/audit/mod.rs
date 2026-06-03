//! Cross-cutting audit boundary.
//!
//! Audit records who or what changed business state. It is separate from
//! telemetry, which observes runtime behavior, and separate from data lineage,
//! which belongs to `modules::data_pipeline`.
