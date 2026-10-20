use std::collections::BTreeSet;
use provenance_evidence_agent::agent::{make_record, classify_http_username_status, EvidenceInput, Origin, EvidenceError};

fn input(origin: Origin) -> EvidenceInput {
    EvidenceInput {
        claim:"DNS record returned an address".into(),
        origin,
        scope_id:Some("case123".into()),
        source_ref:Some("res:primary".into()),
        observed_at:Some("2026-10-20T10:00:00Z".into()),
        verifier_ref:Some("review:human456".into()),
        source_sha256:Some("a".repeat(64)),
    }
}
#[test]
fn simulated_is_unequivocally_labeled_and_not_verified() {
    let mut i=input(Origin::Simulated);
    i.verifier_ref=None;
    let out=make_record(&i,&BTreeSet::new()).unwrap();
    assert!(out.ui_badge.starts_with("SYMULOWANE"));
    assert!(!out.independently_verified);
    assert_eq!(out.origin,Origin::Simulated);
}
#[test]
fn observed_requires_trace_and_still_never_claims_verification() {
    let mut i=input(Origin::Observed);
    i.source_ref=None;
    assert_eq!(make_record(&i,&BTreeSet::new()).err(),Some(EvidenceError::MissingProvenance));
    i.source_ref=Some("res:primary".into());
    let out=make_record(&i,&BTreeSet::new()).unwrap();
    assert!(!out.independently_verified);
}
#[test]
fn verified_requires_external_trusted_host_verifier() {
    let i=input(Origin::Verified);
    assert_eq!(make_record(&i,&BTreeSet::new()).err(),Some(EvidenceError::UntrustedVerifier));
    let trusted=BTreeSet::from(["review:human456".to_owned()]);
    let record=make_record(&i,&trusted).unwrap();
    assert!(record.independently_verified);
    assert!(record.ui_badge.starts_with("WERYFIKACJA"));
}
#[test]
fn bogus_source_hash_rejected() {
    let mut i=input(Origin::Observed);
    i.source_sha256=Some("NOT SHA256".into());
    assert_eq!(make_record(&i,&BTreeSet::new()).err(),Some(EvidenceError::InvalidDigest));
}
#[test]
fn http_code_does_not_assert_online_account_exists() {
    for code in [200,301,403,404,429,500] {
        assert_eq!(classify_http_username_status(code).unwrap(),"inconclusive_http_response");
    }
    assert!(classify_http_username_status(600).is_err());
}
