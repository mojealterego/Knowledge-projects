use chrono::{TimeZone, Utc};
use dgm_sentinel_agent::agent::{assess_promotion, PromotionProposal, PromotionTrust};
use std::collections::{BTreeMap, BTreeSet};

fn good() -> (PromotionProposal, PromotionTrust) {
    let candidate = "a".repeat(40);
    let mut receipts = BTreeMap::new();
    for phase in ["ModelScout", "TechRecon", "Strategist", "DGM_Core"] {
        receipts.insert(phase.to_owned(), format!("trusted:{phase}"));
    }
    let proposal = PromotionProposal {
        candidate_sha: candidate.clone(),
        base_sha: "b".repeat(40),
        branch: "integration/test-candidate".into(),
        phase_receipts: receipts,
        required_checks: vec!["tests".into(), "security".into()],
        approval_expires_at: "2026-10-22T00:00:00Z".into(),
        external_side_effects: false,
        rollback_ref_verified: true,
        source_provenance_verified: true,
    };
    let trust = PromotionTrust {
        verified_ci_by_candidate: BTreeMap::from([(
            candidate.clone(), BTreeSet::from(["tests".into(), "security".into()])
        )]),
        approved_candidates: BTreeSet::from([candidate.clone()]),
        approved_phase_receipts: BTreeMap::from([(candidate.clone(),
            proposal.phase_receipts.values().cloned().collect())]),
        verified_rollback_candidates: BTreeSet::from([candidate.clone()]),
        verified_source_candidates: BTreeSet::from([candidate]),
    };
    (proposal, trust)
}
fn now() -> chrono::DateTime<Utc> {
    Utc.with_ymd_and_hms(2026, 10, 20, 12, 0, 0).unwrap()
}

#[test]
fn authentic_independent_evidence_can_admit_but_never_execute() {
    let (p, trust) = good();
    let result = assess_promotion(&p, &trust, now());
    assert!(result.admitted, "{:?}", result.reasons);
    assert!(!result.execution_authorized);
}
#[test]
fn agent_claims_of_rollback_source_and_phase_are_not_self_authorizing() {
    let (p, mut trust) = good();
    trust.verified_rollback_candidates.clear();
    trust.verified_source_candidates.clear();
    trust.approved_phase_receipts.clear();
    let verdict = assess_promotion(&p, &trust, now());
    assert!(!verdict.admitted);
    assert!(verdict.reasons.contains(&"missing_rollback_evidence".to_owned()));
    assert!(verdict.reasons.contains(&"missing_source_provenance".to_owned()));
    assert!(verdict.reasons.contains(&"missing_four_phase_receipts".to_owned()));
}
#[test]
fn default_branch_cannot_be_promoted_or_merged() {
    let (mut p, trust) = good();
    p.branch = "main".into();
    assert!(assess_promotion(&p, &trust, now()).reasons.contains(&"not_an_isolated_integration_branch".to_owned()));
}
#[test]
fn untrusted_check_and_approval_fail_closed() {
    let (p, mut trust) = good();
    trust.verified_ci_by_candidate.clear();
    trust.approved_candidates.clear();
    let verdict = assess_promotion(&p, &trust, now());
    assert!(verdict.reasons.contains(&"missing_verified_ci_check".to_owned()));
    assert!(verdict.reasons.contains(&"not_independently_approved".to_owned()));
}
