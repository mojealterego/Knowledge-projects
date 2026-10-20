#![cfg(feature = "ocaf-integration")]
use ocaf_runtime::runtime::{OcafRuntime, RuntimeError, ToolCallRequest};
use veritas_trace_agent::zenoh::{manifest, AgentTool};

fn demo()->serde_json::Value {
    serde_json::json!({"claim":"synthetic observation","origin":"simulated"})
}
#[tokio::test]
async fn local_credential_free_simulation_is_always_marked_unverified() {
    let rt=OcafRuntime::new(0);
    rt.register(manifest(),AgentTool).await.unwrap();
    let res=rt.execute(ToolCallRequest{call_id:"test010".into(),
        tool:"osint.provenance_record".into(),args:demo(),auth:None}).await.unwrap();
    assert_eq!(res.output["origin"],"simulated");
    assert_eq!(res.output["independently_verified"],false);
    assert_eq!(res.output["source_contacted"],false);
}
#[tokio::test]
async fn model_supplied_evidence_attestation_is_denied() {
    let rt=OcafRuntime::new(0);
    rt.register(manifest(),AgentTool).await.unwrap();
    let mut p=demo();p["externally_verified"]=serde_json::json!(true);
    assert!(rt.execute(ToolCallRequest{call_id:"spoof010".into(),
        tool:"osint.provenance_record".into(),args:p,auth:None}).await.is_err());
}
#[tokio::test]
async fn unauthenticated_peer_is_denied() {
    let rt=OcafRuntime::new(0).with_tenant_namespace("tenant1").unwrap();
    rt.register(manifest(),AgentTool).await.unwrap();
    let result=rt.execute_authenticated_peer(ToolCallRequest{
        call_id:"peer010".into(),tool:"osint.provenance_record".into(),
        args:demo(),auth:None},"peer1").await;
    assert!(matches!(result,Err(RuntimeError::AuthRequired(_))));
}
