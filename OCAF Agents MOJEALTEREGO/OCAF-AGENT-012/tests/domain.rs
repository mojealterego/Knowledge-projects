use chrono::{TimeZone, Utc};
use fides_grantis_agent::agent::{review, ProgramClaim};
fn base() -> ProgramClaim {
    ProgramClaim {
        repository:"mojealterego/Knowledge-projects".into(),action:"research".into(),
        credential_in_payload:Some(false),program:"alibaba_ai_catalyst".into(),
        credits_usd:Some(120000),tokens:Some(2000000000),status:None,
        company_eligibility_checked:None,
    }
}
fn now() -> chrono::DateTime<Utc> { Utc.with_ymd_and_hms(2026,10,9,12,0,0).unwrap() }
#[test]
fn research_claim_may_be_reviewed_but_is_not_proof_of_award() {
    let d=review(&base(),now());
    assert!(d.permitted);
    assert!(!d.funding_awarded_verified);
    assert!(!d.execution_authorized);
}
#[test]
fn awarding_claims_and_submission_are_rejected() {
    let mut p=base();
    p.status=Some("awarded".into());
    assert!(review(&p,now()).reasons.contains(&"funding_not_confirmed".to_owned()));
    p.action="submit".into();
    assert!(review(&p,now()).reasons.contains(&"external_action_not_authorized".to_owned()));
}
#[test]
fn cost_limits_and_eligibility_are_fail_closed() {
    let mut p=base(); p.credits_usd=Some(120001);
    assert!(review(&p,now()).reasons.contains(&"not_a_verified_credit_limit".to_owned()));
    p.credits_usd=Some(120000);p.action="draft".into();
    assert!(review(&p,now()).reasons.contains(&"company_eligibility_unverified".to_owned()));
}
#[test]
fn historical_event_is_not_open_for_new_submission() {
    let mut p=base();p.program="general_learning_hacks".into();p.action="draft".into();p.status=Some("ended".into());
    assert!(review(&p,now()).reasons.contains(&"past_event_not_submittable".to_owned()));
}
