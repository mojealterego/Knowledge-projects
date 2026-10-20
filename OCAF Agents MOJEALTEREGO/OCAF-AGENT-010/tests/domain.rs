use veritas_trace_agent::agent::{
    classify_http_username_status, record_evidence, EvidenceError, EvidenceInput,
};
fn observed() -> EvidenceInput {
    EvidenceInput {
        claim: "A DNS observation reported".into(),
        origin: "observed".into(),
        scope_id: Some("scope-1".into()),
        source_ref: Some("res:primary".into()),
        observed_at: Some("2026-10-20T08:00:00Z".into()),
        verifier_ref: None,
        source_sha256: Some("a".repeat(64)),
    }
}
#[test]
fn simulated_claim_is_never_independently_verified() {
    let mut x = observed();
    x.origin = "simulated".into();
    x.scope_id = None;
    x.source_ref = None;
    x.observed_at = None;
    x.source_sha256 = None;
    let out = record_evidence(x).unwrap();
    assert_eq!(out.origin, "simulated");
    assert!(!out.independently_verified);
    assert!(out.ui_badge.contains("SYMULOWANE"));
}
#[test]
fn non_simulated_claim_requires_provenance() {
    let mut x = observed();
    x.scope_id = None;
    assert_eq!(
        record_evidence(x).err(),
        Some(EvidenceError::MissingProvenance)
    );
}
#[test]
fn independently_unchecked_verifier_ref_is_not_a_trusted_attestation() {
    let mut x = observed();
    x.origin = "verified".into();
    x.verifier_ref = Some("review:human-456".into());
    let out = record_evidence(x).unwrap();
    assert_eq!(out.origin, "verified");
    assert!(!out.independently_verified);
    assert!(out.ui_badge.contains("WERYFIKACJA"));
}
#[test]
fn http_200_does_not_prove_online_account() {
    assert_eq!(
        classify_http_username_status(200).unwrap(),
        "inconclusive_http_response"
    );
    assert!(classify_http_username_status(99).is_err());
}
