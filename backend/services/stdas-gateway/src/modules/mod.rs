//! Business module boundaries inside the single `stdas-gateway` runtime.
//!
//! STDAS currently uses a modular monolith: one Axum process with clear domain
//! boundaries. These modules mirror future service boundaries, but they are not
//! independent runtime services yet.

pub mod analytics;
pub mod customer;
pub mod data_pipeline;
pub mod evidence;
pub mod identity;
pub mod integration;
pub mod workflow;
