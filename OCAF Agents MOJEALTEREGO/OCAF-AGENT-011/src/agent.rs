//! Offline source provenance records, no OSINT scanning or account attribution.
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

pub const AGENT_NAME: &str = "PROVENANT EVIDENCE";
pub const REGISTRY_NUMBER: &str = "011";
pub const AGENT_ID: &str = "AM-OCAF-AI-0011";
pub const SERIAL_NUMBER: &str = "AM-PRV-EVD-2026-000011";
pub const CATALOG_NUMBER: &str = "AM-OCAF-PRV-011";

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all="lowercase")]
pub enum Origin {
    Simulated,
    Observed,
    Verified,
}
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceInput {
    pub claim: String,
    pub origin: Origin,
    pub scope_id: Option<String>,
    pub source_ref: Option<String>,
    pub observed_at: Option<String>,
    pub verifier_ref: Option<String>,
    pub source_sha256: Option<String>,
}
#[derive(Debug, Clone, Serialize)]
pub struct EvidenceRecord {
    pub agent_id: &'static str,
    pub claim: String,
    pub origin: Origin,
    pub scope_id: Option<String>,
    pub source_ref: Option<String>,
    pub observed_at: Option<String>,
    pub verifier_ref: Option<String>,
    pub source_sha256: Option<String>,
    pub ui_badge: &'static str,
    pub independently_verified: bool,
    pub execution_authorized: bool,
}
#[derive(Debug, PartialEq, Eq)]
pub enum EvidenceError {
    EmptyClaim,
    InvalidDigest,
    MissingProvenance,
    MissingVerifier,
    UntrustedVerifier,
    InvalidHttpStatus,
}
fn nonblank(x: Option<&String>) -> bool {
    x.is_some_and(|v| !v.trim().is_empty() && v.len() <= 256
        && !v.chars().any(char::is_control))
}
fn valid_digest(v: &str) -> bool {
    v.len() == 64 && v.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
pub fn make_record(
    input: &EvidenceInput,
    trusted_verifier_refs: &BTreeSet<String>,
) -> Result<EvidenceRecord, EvidenceError> {
    if input.claim.trim().is_empty() || input.claim.len() > 4096 {
        return Err(EvidenceError::EmptyClaim);
    }
    if input.source_sha256.as_ref().is_some_and(|v| !valid_digest(v)) {
        return Err(EvidenceError::InvalidDigest);
    }
    if input.origin != Origin::Simulated
        && !(nonblank(input.scope_id.as_ref()) && nonblank(input.source_ref.as_ref())
            && nonblank(input.observed_at.as_ref()) && input.source_sha256.is_some())
    {
        return Err(EvidenceError::MissingProvenance);
    }
    if input.origin == Origin::Verified {
        if !nonblank(input.verifier_ref.as_ref()) {
            return Err(EvidenceError::MissingVerifier);
        }
        if !input.verifier_ref.as_ref().is_some_and(|r| trusted_verifier_refs.contains(r)) {
            return Err(EvidenceError::UntrustedVerifier);
        }
    }
    let badge = match input.origin {
        Origin::Simulated => "SYMULOWANE — brak ustaleń faktycznych",
        Origin::Observed => "ZAOBSERWOWANE — niezweryfikowane",
        Origin::Verified => "WERYFIKACJA UDOKUMENTOWANA — sprawdź dowody",
    };
    Ok(EvidenceRecord {
        agent_id: AGENT_ID,
        claim: input.claim.trim().to_owned(),
        origin: input.origin,
        scope_id: input.scope_id.clone(),
        source_ref: input.source_ref.clone(),
        observed_at: input.observed_at.clone(),
        verifier_ref: input.verifier_ref.clone(),
        source_sha256: input.source_sha256.clone(),
        ui_badge: badge,
        independently_verified: input.origin == Origin::Verified,
        execution_authorized: false,
    })
}
pub fn classify_http_username_status(status: i32) -> Result<&'static str, EvidenceError> {
    if !(100..=599).contains(&status) {
        return Err(EvidenceError::InvalidHttpStatus);
    }
    Ok("inconclusive_http_response")
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simulated_cannot_upgrade_to_verified_without_host() {
        let p = EvidenceInput {
            claim: "synthetic graph".into(), origin: Origin::Verified, scope_id:None,
            source_ref:None, observed_at:None, verifier_ref:Some("fake".into()),
            source_sha256:None,
        };
        assert!(make_record(&p,&BTreeSet::new()).is_err());
    }
    #[test]
    fn invalid_hash_not_evidence() {
        assert!(!valid_digest("ABC"));
        assert!(valid_digest(&"a".repeat(64)));
    }
}
