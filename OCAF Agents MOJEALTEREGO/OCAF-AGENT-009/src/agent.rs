//! Offline Rust port of cloud_operation_budget_gate.py (Google Cloud plan gate).
//! Return value is a review decision, NEVER IAM or payment authorization.
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::str::FromStr;

pub const AGENT_NAME: &str = "CLOUD FISCALIS";
pub const REGISTRY_NUMBER: &str = "009";
pub const AGENT_ID: &str = "AM-OCAF-AI-0009";
pub const SERIAL_NUMBER: &str = "AM-CLD-FIS-2026-000009";
pub const CATALOG_NUMBER: &str = "AM-OCAF-CLD-009";

const ALLOWED_PRODUCTS: [&str; 8] = ["agent_platform", "compute_engine", "cloud_storage",
    "vision_ai", "speech_to_text", "translation", "natural_language", "video_intelligence"];
const ALLOWED_ACTIONS: [&str; 8] = ["describe_public_documentation", "read_metadata",
    "estimate_cost", "create_resource", "enable_api", "invoke_model",
    "delete_resource", "make_payment"];
const WRITE_ACTIONS: [&str; 5] = ["create_resource","enable_api","invoke_model",
    "delete_resource","make_payment"];

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CloudPlan {
    pub action: String,
    pub product: String,
    pub request_id: Option<String>,
    pub project_scope: Option<String>,
    pub maximum_cost_usd: String,
    pub data_classification: Option<String>,
    pub credentials_in_prompt: Option<bool>,
    pub explicit_external_side_effect_approval: Option<bool>,
}

/// Approval must be supplied by the trusted policy host, not from model JSON.
#[derive(Debug, Clone)]
pub struct CloudApproval {
    pub project_scope: String,
    pub allowed_actions: BTreeSet<String>,
    pub allowed_products: BTreeSet<String>,
    pub max_cost_usd: Decimal,
    pub human_reviewed: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct CloudVerdict {
    pub agent_id: &'static str,
    pub allowed: bool,
    pub reasons: Vec<String>,
    pub execution_authorized: bool,
    pub charge_incurred: bool,
}

fn valid_request_id(s: &str) -> bool {
    (3..=81).contains(&s.len())
        && s.bytes().next().is_some_and(|b| b.is_ascii_lowercase() || b.is_ascii_digit())
        && s.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit()
            || b == b'.' || b == b'_' || b == b'-')
}
fn cost(input: &str) -> Option<Decimal> {
    let value = Decimal::from_str(input).ok()?;
    if value < Decimal::ZERO || input.len() > 80 {
        return None;
    }
    Some(value)
}
fn push_unique(reasons: &mut Vec<String>, reason: &str) {
    if !reasons.iter().any(|x| x == reason) {
        reasons.push(reason.to_owned());
    }
}
pub fn review_cloud_plan(
    plan: &CloudPlan,
    approved_requests: &BTreeMap<String, CloudApproval>,
) -> CloudVerdict {
    let mut reasons = Vec::new();
    if !ALLOWED_ACTIONS.contains(&plan.action.as_str()) {
        push_unique(&mut reasons, "unknown_action");
    }
    if !ALLOWED_PRODUCTS.contains(&plan.product.as_str()) {
        push_unique(&mut reasons, "unknown_product");
    }
    let price = cost(&plan.maximum_cost_usd);
    if price.is_none() {
        push_unique(&mut reasons, "bad_cost");
    }
    // Hardened against secrets in public-doc requests (upstream checks after early return).
    if plan.credentials_in_prompt == Some(true) {
        push_unique(&mut reasons, "secrets_in_prompt_forbidden");
    }
    if plan.action == "describe_public_documentation" {
        if price != Some(Decimal::ZERO) {
            push_unique(&mut reasons, "public_docs_must_be_free");
        }
        if plan.data_classification.as_deref() != Some("public") {
            push_unique(&mut reasons, "public_docs_require_public_data");
        }
        return CloudVerdict {
            agent_id: AGENT_ID, allowed: reasons.is_empty(), reasons,
            execution_authorized: false, charge_incurred: false,
        };
    }
    let id = plan.request_id.as_deref().unwrap_or("");
    if !valid_request_id(id) {
        push_unique(&mut reasons, "missing_request_id");
    }
    match approved_requests.get(id) {
        None => push_unique(&mut reasons, "missing_independent_approval"),
        Some(grant) if !grant.human_reviewed => {
            push_unique(&mut reasons, "missing_independent_approval");
        }
        Some(grant) => {
            if plan.project_scope.as_deref() != Some(&grant.project_scope) {
                push_unique(&mut reasons, "project_scope_mismatch");
            }
            if !grant.allowed_actions.contains(&plan.action) {
                push_unique(&mut reasons, "operation_not_authorized");
            }
            if !grant.allowed_products.contains(&plan.product) {
                push_unique(&mut reasons, "product_not_authorized");
            }
            if price.is_some_and(|p| p > grant.max_cost_usd) {
                push_unique(&mut reasons, "over_budget");
            }
        }
    }
    if WRITE_ACTIONS.contains(&plan.action.as_str())
        && plan.explicit_external_side_effect_approval != Some(true)
    {
        push_unique(&mut reasons, "side_effect_not_approved");
    }
    if plan.credentials_in_prompt != Some(false) {
        push_unique(&mut reasons, "secrets_in_prompt_forbidden");
    }
    CloudVerdict {
        agent_id: AGENT_ID, allowed: reasons.is_empty(), reasons,
        execution_authorized: false, charge_incurred: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suspicious_id_and_nan_cost_rejected() {
        assert!(!valid_request_id("ticket/../../owner"));
        assert!(!valid_request_id("a"));
        assert!(valid_request_id("ticket-001"));
        assert!(cost("Infinity").is_none());
        assert!(cost("-0.01").is_none());
    }
    #[test]
    fn unauthorized_payment_never_executed() {
        let proposal = CloudPlan {
            action: "make_payment".into(), product: "compute_engine".into(),
            request_id: Some("ticket-001".into()),
            project_scope: Some("some-project".into()),
            maximum_cost_usd:"50".into(),data_classification:None,
            credentials_in_prompt:Some(false),
            explicit_external_side_effect_approval:Some(true),
        };
        let decision = review_cloud_plan(&proposal,&BTreeMap::new());
        assert!(!decision.allowed);
        assert!(!decision.execution_authorized);
        assert!(!decision.charge_incurred);
    }
}
