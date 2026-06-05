//! Identity boundary.
//!
//! Owns user, role, session, permission and `CustomerScope` concerns when those
//! features become real implementation work.

mod dto;
mod handlers;
mod models;
mod routes;
mod service;

pub(crate) use routes::router;
