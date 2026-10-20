#![cfg(feature="ocaf-integration")]
use chrono::{TimeZone,Utc};
use fides_grantis_agent::zenoh::{manifest,AgentTool};
use ocaf_runtime::runtime::{OcafRuntime,RuntimeError,ToolCallRequest};
fn sample()->serde_json::Value{
    serde_json::json!({"repository":"mojealterego/Knowledge-projects","action":"research",
        "credential_in_payload":false,"program":"alibaba_ai_catalyst",
        "credits_usd":120000,"tokens":2000000000})
}
#[tokio::test]
async fn local_review_never_certifies_funding(){
    let rt=OcafRuntime::new(0);
    rt.register(manifest(),AgentTool::new(Utc.with_ymd_and_hms(2026,10,9,12,0,0).unwrap())).await.unwrap();
    let r=rt.execute(ToolCallRequest{call_id:"t012".into(),tool:"program.claim_review".into(),args:sample(),auth:None}).await.unwrap();
    assert_eq!(r.output["permitted"],true);
    assert_eq!(r.output["funding_awarded_verified"],false);
    assert_eq!(r.output["external_submission_executed"],false);
}
#[tokio::test]
async fn model_cannot_override_review_clock(){
    let rt=OcafRuntime::new(0);
    rt.register(manifest(),AgentTool::new(Utc.with_ymd_and_hms(2026,10,9,12,0,0).unwrap())).await.unwrap();
    let mut args=sample();args["now"]=serde_json::json!("2099-01-01T00:00:00Z");
    assert!(rt.execute(ToolCallRequest{call_id:"forged012".into(),tool:"program.claim_review".into(),args,auth:None}).await.is_err());
}
#[tokio::test]
async fn unsigned_peer_denied(){
    let rt=OcafRuntime::new(0).with_tenant_namespace("tenant1").unwrap();
    rt.register(manifest(),AgentTool::new(Utc.with_ymd_and_hms(2026,10,9,12,0,0).unwrap())).await.unwrap();
    let r=rt.execute_authenticated_peer(ToolCallRequest{
        call_id:"unsigned012".into(),tool:"program.claim_review".into(),args:sample(),auth:None
    },"peer").await;
    assert!(matches!(r,Err(RuntimeError::AuthRequired(_))));
}
