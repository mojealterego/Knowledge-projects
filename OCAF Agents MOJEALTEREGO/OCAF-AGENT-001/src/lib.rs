#![forbid(unsafe_code)]
//! Agent 001 for OCAF Runtime. Provider execution and source decryption are not enabled.

pub mod agent;
pub mod lifecycle;
pub mod security;
pub mod zenoh;

pub use agent::{plan, PhotoPlan, PlanError, AGENT_ID, MAX_GOAL_BYTES};
