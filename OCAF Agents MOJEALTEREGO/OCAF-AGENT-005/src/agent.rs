//! Proposal-only Android UI action gate. It neither reads device UI nor acts on it.
use serde::{Deserialize, Serialize};

pub const AGENT_NAME: &str = "ARGUS MOBILE";
pub const REGISTRY_NUMBER: &str = "005";
pub const AGENT_ID: &str = "AM-OCAF-AI-0005";
pub const SERIAL_NUMBER: &str = "AM-MOB-ARG-2026-000005";
pub const CATALOG_NUMBER: &str = "AM-OCAF-MOB-005";

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ActionProposal {
    pub session_id: String,
    pub snapshot_id: String,
    pub node_id: String,
    pub action: String,
    pub operator_grant: bool,
    pub expected_postcondition: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionError {
    InvalidInput,
    UnknownAction,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ActionAssessment {
    pub agent_id: &'static str,
    pub status: &'static str,
    pub execution_authorized: bool,
    pub readback_required: bool,
    pub verified: bool,
    pub reasons: Vec<String>,
}

fn valid_token(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 64
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
}

pub fn assess(input: &ActionProposal) -> Result<ActionAssessment, ActionError> {
    if !valid_token(&input.session_id)
        || !valid_token(&input.snapshot_id)
        || !valid_token(&input.node_id)
        || input.expected_postcondition.trim().is_empty()
        || input.expected_postcondition.len() > 256
        || input.expected_postcondition.chars().any(char::is_control)
    {
        return Err(ActionError::InvalidInput);
    }
    let hard_deny = match input.action.as_str() {
        "DELETE" | "SEND" | "GRANT_PERMISSION" => true,
        "CLICK" | "SCROLL" | "BACK" | "HOME" | "TYPE" => false,
        _ => return Err(ActionError::UnknownAction),
    };
    let status = if hard_deny {
        "blocked"
    } else if !input.operator_grant {
        "requires_grant"
    } else {
        "operator_review"
    };
    let reasons = vec![
        "Operator-provided UI snapshot and permissions are not independently authenticated"
            .to_owned(),
        if hard_deny {
            "High-impact action blocked regardless of asserted operator grant"
        } else if !input.operator_grant {
            "No asserted operator grant for a bounded proposal"
        } else {
            "Candidate only: fresh UI grounding and local policy confirmation required"
        }
        .to_owned(),
        "An authoritative post-action readback is required; no action executed".to_owned(),
    ];
    Ok(ActionAssessment {
        agent_id: AGENT_ID,
        status,
        execution_authorized: false,
        readback_required: true,
        verified: false,
        reasons,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn forged_grants_never_enable_execution() {
        let proposal = ActionProposal {
            session_id: "s".into(),
            snapshot_id: "snap".into(),
            node_id: "node".into(),
            action: "SEND".into(),
            operator_grant: true,
            expected_postcondition: "message sent".into(),
        };
        let result = assess(&proposal).unwrap();
        assert_eq!(result.status, "blocked");
        assert!(!result.execution_authorized);
    }

    #[test]
    fn unknown_and_unsafe_ids_are_rejected() {
        let mut proposal = ActionProposal {
            session_id: "s".into(),
            snapshot_id: "snap".into(),
            node_id: "node".into(),
            action: "CLICK".into(),
            operator_grant: false,
            expected_postcondition: "observed change".into(),
        };
        proposal.session_id = "other/../tenant".into();
        assert_eq!(assess(&proposal), Err(ActionError::InvalidInput));
        proposal.session_id = "s".into();
        proposal.action = "EXEC_SHELL".into();
        assert_eq!(assess(&proposal), Err(ActionError::UnknownAction));
    }
}
