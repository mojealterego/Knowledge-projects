#![cfg(feature = "ocaf-integration")]
use chrono::{TimeZone,Utc};
use dgm_sentinel_agent::agent::{PromotionTrust,EXPECTED_PHASES};
use dgm_sentinel_agent::zenoh::{manifest,AgentTool};
use ocaf_runtime::runtime::{OcafRuntime,RuntimeError,ToolCallRequest};
use std::collections::{BTreeMap,BTreeSet};

fn inputs()->(serde_json::Value,PromotionTrust){
    let candidate="a".repeat(40);
    let receipts:BTreeMap<String,String>=EXPECTED_PHASES.iter()
        .map(|v|((*v).to_owned(),format!("trusted:{v}"))).collect();
    let proposal=serde_json::json!({
        "candidate_sha":candidate, "base_sha":"b".repeat(40),
        "branch":"integration/review", "phase_receipts":receipts,
        "required_checks":["tests"], "approval_expires_at":"2026-10-22T00:00:00Z",
        "external_side_effects":false, "rollback_ref_verified":true,
        "source_provenance_verified":true
    });
    let trust=PromotionTrust {
        verified_ci_by_candidate:BTreeMap::from([(candidate.clone(),BTreeSet::from(["tests".into()]))]),
        approved_candidates:BTreeSet::from([candidate.clone()]),
        approved_phase_receipts:BTreeMap::from([(candidate.clone(),receipts.values().cloned().collect())]),
        verified_rollback_candidates:BTreeSet::from([candidate.clone()]),
        verified_source_candidates:BTreeSet::from([candidate]),
    };
    (proposal,trust)
}
#[tokio::test]
async fn trusted_host_evidence_allows_review_but_not_merging(){
    let (p,t)=inputs();
    let rt=OcafRuntime::new(0);
    rt.register(manifest(),AgentTool::new(t,Utc.with_ymd_and_hms(2026,10,20,12,0,0).unwrap())).await.unwrap();
    let r=rt.execute(ToolCallRequest {call_id:"dgm008".into(),tool:"dgm.assess_promotion".into(),args:p,auth:None}).await.unwrap();
    assert_eq!(r.output["admitted"],true);
    assert_eq!(r.output["execution_authorized"],false);
    assert_eq!(r.output["branch_mutated"],false);
}
#[tokio::test]
async fn self_asserted_host_trust_in_input_is_rejected(){
    let (mut p, _)=inputs();
    p["approved_candidates"]=serde_json::json!(["a".repeat(40)]);
    let rt=OcafRuntime::new(0);
    rt.register(manifest(),AgentTool::new(PromotionTrust::default(),Utc.with_ymd_and_hms(2026,10,20,12,0,0).unwrap())).await.unwrap();
    assert!(rt.execute(ToolCallRequest {call_id:"forged008".into(),tool:"dgm.assess_promotion".into(),args:p,auth:None}).await.is_err());
}
#[tokio::test]
async fn unauthenticated_peer_is_denied(){
    let (p,t)=inputs();
    let rt=OcafRuntime::new(0).with_tenant_namespace("tenant1").unwrap();
    rt.register(manifest(),AgentTool::new(t,Utc.with_ymd_and_hms(2026,10,20,12,0,0).unwrap())).await.unwrap();
    let out=rt.execute_authenticated_peer(ToolCallRequest {call_id:"unsigned008".into(),tool:"dgm.assess_promotion".into(),args:p,auth:None},"peer").await;
    assert!(matches!(out,Err(RuntimeError::AuthRequired(_))));
}
