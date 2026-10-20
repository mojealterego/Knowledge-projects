//! Offline translation of tools/program_claim_gate.py. Grants are not verified awards.
use chrono::{DateTime,TimeZone,Utc};
use serde::{Deserialize,Serialize};
pub const AGENT_NAME:&str="FIDES GRANTIS";
pub const REGISTRY_NUMBER:&str="012";
pub const AGENT_ID:&str="AM-OCAF-AI-0012";
pub const SERIAL_NUMBER:&str="AM-GRT-FID-2026-000012";
pub const CATALOG_NUMBER:&str="AM-OCAF-GRT-012";
#[derive(Debug,Clone,Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProgramClaim {
    pub repository:String,
    pub action:String,
    pub credential_in_payload:Option<bool>,
    pub program:String,
    pub credits_usd:Option<i64>,
    pub tokens:Option<i64>,
    pub status:Option<String>,
    pub company_eligibility_checked:Option<bool>,
}
#[derive(Debug,Serialize)]
pub struct ProgramVerdict {
    pub agent_id:&'static str,
    pub permitted:bool,
    pub reasons:Vec<String>,
    pub funding_awarded_verified:bool,
    pub external_submission_executed:bool,
    pub execution_authorized:bool,
}
pub fn review(record:&ProgramClaim,now:DateTime<Utc>)->ProgramVerdict {
    let mut reasons=Vec::new();
    if record.repository!="mojealterego/Knowledge-projects" {
        reasons.push("only_single_repository_authorized".into());
    }
    if !matches!(record.action.as_str(),"research"|"draft") {
        reasons.push("external_action_not_authorized".into());
    }
    if record.credential_in_payload!=Some(false) {
        reasons.push("secret_must_not_be_in_payload".into());
    }
    match record.program.as_str() {
        "alibaba_ai_catalyst" => {
            if !record.credits_usd.is_some_and(|v|(0..=120000).contains(&v)) {
                reasons.push("not_a_verified_credit_limit".into());
            }
            if !record.tokens.is_some_and(|v|(0..=2000000000).contains(&v)) {
                reasons.push("not_a_verified_token_limit".into());
            }
            if record.status.as_deref()==Some("awarded") {
                reasons.push("funding_not_confirmed".into());
            }
            if record.action=="draft"&&record.company_eligibility_checked!=Some(true) {
                reasons.push("company_eligibility_unverified".into());
            }
        }
        "general_learning_hacks" => {
            if record.status.as_deref()!=Some("ended") {
                reasons.push("event_is_already_ended".into());
            }
            if record.action=="draft" {
                reasons.push("past_event_not_submittable".into());
            }
            let cutoff=Utc.with_ymd_and_hms(2026,9,20,1,0,0).single().expect("valid cutoff");
            if now<cutoff {
                reasons.push("review_date_precedes_source_cutoff".into());
            }
        }
        _ => reasons.push("unknown_program".into()),
    }
    ProgramVerdict {
        agent_id:AGENT_ID,
        permitted:reasons.is_empty(),
        reasons,
        funding_awarded_verified:false,
        external_submission_executed:false,
        execution_authorized:false,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn repo_violation_and_secret_leak_are_both_reported() {
        let p=ProgramClaim {
            repository:"other/repo".into(),action:"submit".into(),credential_in_payload:Some(true),
            program:"unknown".into(),credits_usd:None,tokens:None,status:None,
            company_eligibility_checked:None,
        };
        let d=review(&p,Utc.with_ymd_and_hms(2026,10,9,12,0,0).unwrap());
        assert!(!d.permitted);
        assert!(d.reasons.contains(&"only_single_repository_authorized".into()));
        assert!(d.reasons.contains(&"secret_must_not_be_in_payload".into()));
    }
    #[test]
    fn false_event_open_status_rejected() {
        let p=ProgramClaim {
            repository:"mojealterego/Knowledge-projects".into(),action:"research".into(),
            credential_in_payload:Some(false),program:"general_learning_hacks".into(),
            credits_usd:None,tokens:None,status:Some("open".into()),
            company_eligibility_checked:None,
        };
        assert!(review(&p,Utc.with_ymd_and_hms(2026,10,9,12,0,0).unwrap()).reasons.contains(&"event_is_already_ended".into()));
    }
}
