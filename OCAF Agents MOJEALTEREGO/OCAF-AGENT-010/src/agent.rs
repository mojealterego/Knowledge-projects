//! Offline provenance bookkeeping, port of tools/osint_provenance_gate.py.
//! "verified" is a *declared* classification, not an independent attestation.
use serde::{Deserialize, Serialize};

pub const AGENT_NAME: &str = "VERITAS TRACE";
pub const REGISTRY_NUMBER: &str = "010";
pub const AGENT_ID: &str = "AM-OCAF-AI-0010";
pub const SERIAL_NUMBER: &str = "AM-OSN-VRT-2026-000010";
pub const CATALOG_NUMBER: &str = "AM-OCAF-OSN-010";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvidenceError {
    EmptyClaim,
    UnknownOrigin,
    MissingProvenance,
    MissingVerifier,
    InvalidDigest,
    InvalidMetadata,
    InvalidHttpStatus,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceInput {
    pub claim: String,
    pub origin: String,
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
    pub origin: String,
    pub scope_id: Option<String>,
    pub source_ref: Option<String>,
    pub observed_at: Option<String>,
    pub verifier_ref: Option<String>,
    pub source_sha256: Option<String>,
    pub ui_badge: &'static str,
    pub independently_verified: bool,
    pub source_contacted: bool,
}

fn valid_optional(value: &Option<String>, max_len: usize) -> bool {
    value.as_ref().is_none_or(|x| {
        !x.trim().is_empty() && x.len() <= max_len && !x.chars().any(char::is_control)
    })
}
fn valid_digest(s: &str) -> bool {
    s.len() == 64
        && s.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
pub fn record_evidence(input: EvidenceInput) -> Result<EvidenceRecord, EvidenceError> {
    if input.claim.trim().is_empty()
        || input.claim.len() > 2048
        || input.claim.chars().any(char::is_control)
    {
        return Err(EvidenceError::EmptyClaim);
    }
    let badge = match input.origin.as_str() {
        "simulated" => "SYMULOWANE — brak ustaleń faktycznych",
        "observed" => "ZAOBSERWOWANE — niezweryfikowane",
        "verified" => "WERYFIKACJA UDOKUMENTOWANA — sprawdź dowody",
        _ => return Err(EvidenceError::UnknownOrigin),
    };
    if !valid_optional(&input.scope_id, 128)
        || !valid_optional(&input.source_ref, 256)
        || !valid_optional(&input.observed_at, 128)
        || !valid_optional(&input.verifier_ref, 256)
    {
        return Err(EvidenceError::InvalidMetadata);
    }
    if input
        .source_sha256
        .as_deref()
        .is_some_and(|s| !valid_digest(s))
    {
        return Err(EvidenceError::InvalidDigest);
    }
    if input.origin != "simulated"
        && [
            input.scope_id.as_deref(),
            input.source_ref.as_deref(),
            input.observed_at.as_deref(),
            input.source_sha256.as_deref(),
        ]
        .iter()
        .any(Option::is_none)
    {
        return Err(EvidenceError::MissingProvenance);
    }
    if input.origin == "verified" && input.verifier_ref.is_none() {
        return Err(EvidenceError::MissingVerifier);
    }
    Ok(EvidenceRecord {
        agent_id: AGENT_ID,
        claim: input.claim.trim().to_owned(),
        origin: input.origin,
        scope_id: input.scope_id,
        source_ref: input.source_ref,
        observed_at: input.observed_at,
        verifier_ref: input.verifier_ref,
        source_sha256: input.source_sha256,
        ui_badge: badge,
        independently_verified: false,
        source_contacted: false,
    })
}

pub fn classify_http_username_status(status: u16) -> Result<&'static str, EvidenceError> {
    if !(100..=599).contains(&status) {
        return Err(EvidenceError::InvalidHttpStatus);
    }
    Ok("inconclusive_http_response")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unknown_origin_and_invalid_digest_denied() {
        let input = EvidenceInput {
            claim: "x".into(),
            origin: "real".into(),
            scope_id: None,
            source_ref: None,
            observed_at: None,
            verifier_ref: None,
            source_sha256: None,
        };
        assert_eq!(
            record_evidence(input.clone()).err(),
            Some(EvidenceError::UnknownOrigin)
        );
        let mut other = input;
        other.origin = "simulated".into();
        other.source_sha256 = Some("BAD".into());
        assert_eq!(
            record_evidence(other).err(),
            Some(EvidenceError::InvalidDigest)
        );
    }
    #[test]
    fn verifier_reference_is_required_but_not_independent_proof() {
        let input = EvidenceInput {
            claim: "example".into(),
            origin: "verified".into(),
            scope_id: Some("c".into()),
            source_ref: Some("r".into()),
            observed_at: Some("2026-10-20T10:00:00Z".into()),
            verifier_ref: None,
            source_sha256: Some("a".repeat(64)),
        };
        assert_eq!(
            record_evidence(input).err(),
            Some(EvidenceError::MissingVerifier)
        );
    }
}
