//! Offline port of tools/dgm_cycle_gate.py, strengthened with host-only attestations.
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const AGENT_NAME: &str = "DGM SENTINEL";
pub const REGISTRY_NUMBER: &str = "008";
pub const AGENT_ID: &str = "AM-OCAF-AI-0008";
pub const SERIAL_NUMBER: &str = "AM-DGM-SNT-2026-000008";
pub const CATALOG_NUMBER: &str = "AM-OCAF-DGM-008";
pub const EXPECTED_PHASES: [&str; 4] = ["ModelScout", "TechRecon", "Strategist", "DGM_Core"];

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PromotionProposal {
    pub candidate_sha: String,
    pub base_sha: String,
    pub branch: String,
    pub phase_receipts: BTreeMap<String, String>,
    pub required_checks: Vec<String>,
    pub approval_expires_at: String,
    pub external_side_effects: bool,
    pub rollback_ref_verified: bool,
    pub source_provenance_verified: bool,
}

/// Supplied by a separate trusted operator service, never from model tool JSON.
#[derive(Debug, Clone, Default)]
pub struct PromotionTrust {
    pub verified_ci_by_candidate: BTreeMap<String, BTreeSet<String>>,
    pub approved_candidates: BTreeSet<String>,
    pub approved_phase_receipts: BTreeMap<String, BTreeSet<String>>,
    pub verified_rollback_candidates: BTreeSet<String>,
    pub verified_source_candidates: BTreeSet<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PromotionDecision {
    pub agent_id: &'static str,
    pub admitted: bool,
    pub reasons: Vec<String>,
    pub execution_authorized: bool,
    pub branch_mutated: bool,
}

fn git_sha(v: &str) -> bool {
    v.len() == 40
        && v.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
fn isolated_branch(v: &str) -> bool {
    let Some(tail) = v.strip_prefix("integration/") else {
        return false;
    };
    !tail.is_empty()
        && tail.len() <= 128
        && !tail.contains("..")
        && !tail.starts_with('/')
        && !tail.ends_with('/')
        && tail
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_' || b == b'/' || b == b'.')
}

pub fn assess_promotion(
    proposal: &PromotionProposal,
    trust: &PromotionTrust,
    now: DateTime<Utc>,
) -> PromotionDecision {
    let mut reasons = Vec::new();
    if !git_sha(&proposal.candidate_sha) {
        reasons.push("invalid_candidate_sha".to_owned());
    }
    if !git_sha(&proposal.base_sha) || proposal.base_sha == proposal.candidate_sha {
        reasons.push("invalid_base_sha".to_owned());
    }
    if !isolated_branch(&proposal.branch) {
        reasons.push("not_an_isolated_integration_branch".to_owned());
    }
    let exact_phases = proposal.phase_receipts.len() == EXPECTED_PHASES.len()
        && EXPECTED_PHASES.iter().all(|phase| {
            proposal
                .phase_receipts
                .get(*phase)
                .is_some_and(|receipt| !receipt.trim().is_empty() && receipt.len() <= 256)
        });
    let approved_receipts = trust.approved_phase_receipts.get(&proposal.candidate_sha);
    let valid_receipts = exact_phases
        && approved_receipts.is_some_and(|trusted| {
            proposal
                .phase_receipts
                .values()
                .all(|receipt| trusted.contains(receipt))
        });
    if !valid_receipts {
        reasons.push("missing_four_phase_receipts".to_owned());
    }
    if !trust.approved_candidates.contains(&proposal.candidate_sha) {
        reasons.push("not_independently_approved".to_owned());
    }
    if proposal.required_checks.is_empty()
        || proposal.required_checks.len() > 32
        || !proposal
            .required_checks
            .iter()
            .all(|c| !c.trim().is_empty() && c.len() <= 128 && !c.chars().any(char::is_control))
    {
        reasons.push("invalid_required_checks".to_owned());
    } else {
        let ci = trust.verified_ci_by_candidate.get(&proposal.candidate_sha);
        if ci.is_none_or(|checks| !proposal.required_checks.iter().all(|r| checks.contains(r))) {
            reasons.push("missing_verified_ci_check".to_owned());
        }
    }
    if !proposal.approval_expires_at.ends_with('Z') {
        reasons.push("invalid_approval_expiration".to_owned());
    } else {
        match DateTime::parse_from_rfc3339(&proposal.approval_expires_at) {
            Ok(exp) if exp <= now => reasons.push("approval_expired".to_owned()),
            Err(_) => reasons.push("invalid_approval_expiration".to_owned()),
            Ok(_) => {}
        }
    }
    if proposal.external_side_effects {
        reasons.push("external_side_effects_forbidden".to_owned());
    }
    if !proposal.rollback_ref_verified
        || !trust
            .verified_rollback_candidates
            .contains(&proposal.candidate_sha)
    {
        reasons.push("missing_rollback_evidence".to_owned());
    }
    if !proposal.source_provenance_verified
        || !trust
            .verified_source_candidates
            .contains(&proposal.candidate_sha)
    {
        reasons.push("missing_source_provenance".to_owned());
    }
    PromotionDecision {
        agent_id: AGENT_ID,
        admitted: reasons.is_empty(),
        reasons,
        execution_authorized: false,
        branch_mutated: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn invalid_sha_and_missing_phase_fail_closed() {
        let p = PromotionProposal {
            candidate_sha: "bad".into(),
            base_sha: "bad".into(),
            branch: "main".into(),
            phase_receipts: BTreeMap::new(),
            required_checks: vec![],
            approval_expires_at: "tomorrow".into(),
            external_side_effects: true,
            rollback_ref_verified: false,
            source_provenance_verified: false,
        };
        let out = assess_promotion(
            &p,
            &PromotionTrust::default(),
            Utc.with_ymd_and_hms(2026, 10, 20, 12, 0, 0).unwrap(),
        );
        assert!(!out.admitted);
        assert!(!out.execution_authorized);
        assert!(out.reasons.contains(&"invalid_candidate_sha".into()));
        assert!(out.reasons.contains(&"missing_four_phase_receipts".into()));
    }
    #[test]
    fn malformed_integration_branch_denied() {
        for v in [
            "main",
            "integration/",
            "integration/../../main",
            "integration/folder with spaces",
            "integration/something\\other",
        ] {
            assert!(!isolated_branch(v));
        }
    }
}
