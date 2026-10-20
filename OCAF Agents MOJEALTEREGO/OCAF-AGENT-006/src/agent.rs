//! Non-executing Rust port of tools/agent_tool_admission_gate.py.
//! Artifact hash alone never authenticates a reviewer or provides a sandbox.
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

pub const AGENT_NAME: &str = "CUSTOS ADMISSION";
pub const REGISTRY_NUMBER: &str = "006";
pub const AGENT_ID: &str = "AM-OCAF-AI-0006";
pub const SERIAL_NUMBER: &str = "AM-SEC-CUS-2026-000006";
pub const CATALOG_NUMBER: &str = "AM-OCAF-SEC-006";
pub const SANDBOX_PROFILE: &str = "isolated_no_network_read_only";

const ALLOWED: [&str; 4] = [
    "read_repo_metadata",
    "parse_document",
    "format_text",
    "unit_test_fixture",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ToolManifest {
    pub artifact_sha256: String,
    pub capabilities: Vec<String>,
    pub sandbox_profile: String,
    pub timeout_seconds: u32,
    pub expires_at: String,
    pub external_side_effects: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct AdmissionDecision {
    pub agent_id: &'static str,
    pub accepted: bool,
    pub status: &'static str,
    pub reasons: Vec<String>,
    pub artifact_sha256: String,
    pub execution_authorized: bool,
    pub sandbox_started: bool,
}

pub fn admit(
    manifest: &ToolManifest,
    artifact: &[u8],
    trusted_approvals: &BTreeMap<String, BTreeSet<String>>,
    now: DateTime<Utc>,
) -> AdmissionDecision {
    let digest = format!("{:x}", Sha256::digest(artifact));
    let mut reasons = Vec::new();
    if artifact.is_empty() {
        reasons.push("empty_artifact".into());
    }
    if artifact.len() > 1_048_576 {
        reasons.push("artifact_too_large".into());
    }
    if manifest.artifact_sha256 != digest {
        reasons.push("artifact_hash_mismatch".into());
    }
    let caps: BTreeSet<&str> = manifest.capabilities.iter().map(String::as_str).collect();
    if manifest.capabilities.is_empty() {
        reasons.push("invalid_capabilities".into());
    }
    if caps.len() != manifest.capabilities.len() {
        reasons.push("duplicate_capability".into());
    }
    if !caps.iter().all(|v| ALLOWED.contains(v)) {
        reasons.push("privileged_or_unknown_capability".into());
    }
    match trusted_approvals.get(&digest) {
        None => reasons.push("no_independent_approval".into()),
        Some(reviewed) => {
            if !caps.iter().all(|v| reviewed.contains(*v)) {
                reasons.push("outside_approved_capability_scope".into());
            }
        }
    }
    if manifest.sandbox_profile != SANDBOX_PROFILE {
        reasons.push("missing_isolated_read_only_sandbox".into());
    }
    if !(1..=60).contains(&manifest.timeout_seconds) {
        reasons.push("invalid_timeout".into());
    }
    match manifest.expires_at.strip_suffix('Z') {
        Some(_) => match DateTime::parse_from_rfc3339(&manifest.expires_at) {
            Ok(expires) => {
                if expires <= now {
                    reasons.push("approval_expired".into());
                }
            }
            Err(_) => reasons.push("invalid_expiration".into()),
        },
        None => reasons.push("invalid_expiration".into()),
    }
    if manifest.external_side_effects {
        reasons.push("side_effects_not_disabled".into());
    }
    AdmissionDecision {
        agent_id: AGENT_ID,
        accepted: reasons.is_empty(),
        status: if reasons.is_empty() { "admissible" } else { "rejected" },
        reasons,
        artifact_sha256: digest,
        execution_authorized: false,
        sandbox_started: false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    fn manifest_and_approvals() -> (ToolManifest, Vec<u8>, BTreeMap<String, BTreeSet<String>>) {
        let bytes = b"nonexecuting test artifact".to_vec();
        let digest = format!("{:x}", Sha256::digest(&bytes));
        let m = ToolManifest {
            artifact_sha256: digest.clone(),
            capabilities: vec!["parse_document".into()],
            sandbox_profile: SANDBOX_PROFILE.into(),
            timeout_seconds: 30,
            expires_at: "2026-10-22T00:00:00Z".into(),
            external_side_effects: false,
        };
        let approvals = BTreeMap::from([(
            digest,
            BTreeSet::from(["parse_document".to_owned()]),
        )]);
        (m, bytes, approvals)
    }

    #[test]
    fn parity_approved_but_never_executable() {
        let (m, bytes, approvals) = manifest_and_approvals();
        let result = admit(
            &m, &bytes, &approvals, Utc.with_ymd_and_hms(2026,10,20,12,0,0).unwrap(),
        );
        assert!(result.accepted);
        assert!(!result.execution_authorized);
    }

    #[test]
    fn no_scope_or_expired_is_denied() {
        let (mut m, bytes, approvals) = manifest_and_approvals();
        let now = Utc.with_ymd_and_hms(2026,10,20,12,0,0).unwrap();
        assert!(admit(&m, &bytes, &BTreeMap::new(), now).reasons.contains(&"no_independent_approval".into()));
        m.expires_at = "2026-10-01T00:00:00Z".into();
        assert!(admit(&m, &bytes, &approvals, now).reasons.contains(&"approval_expired".into()));
    }

    #[test]
    fn duplicate_and_side_effects_are_denied() {
        let (mut m, bytes, approvals) = manifest_and_approvals();
        m.capabilities.push("parse_document".into());
        m.external_side_effects = true;
        let now = Utc.with_ymd_and_hms(2026,10,20,12,0,0).unwrap();
        let d = admit(&m, &bytes, &approvals, now);
        assert!(d.reasons.contains(&"duplicate_capability".into()));
        assert!(d.reasons.contains(&"side_effects_not_disabled".into()));
    }
}
