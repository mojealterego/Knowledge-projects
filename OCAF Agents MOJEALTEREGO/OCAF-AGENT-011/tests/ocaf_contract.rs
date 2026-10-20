#![cfg(feature = "ocaf-integration")]
use ocaf_runtime::runtime::{OcafRuntime, RuntimeError, ToolCallRequest};
use provenance_evidence_agent::zenoh::{manifest, AgentTool};
use std::collections::BTreeSet;
fn args() -> serde_json::Value {
    serde_json::json!({
        "claim":"DNS record returned address","origin":"verified","scope_id":"case123",
        "source_ref":"res:primary","observed_at":"2026-10-20T11:00:00Z",
        "source_sha256":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "verifier_ref":"review:human456"
    })
}
#[tokio::test]
async fn host_signed_verifier_registry_controls_qualification() {
    let rt = OcafRuntime::new(0);
    rt.register(
        manifest(),
        AgentTool::new(BTreeSet::from(["review:human456".into()])),
    )
    .await
    .unwrap();
    let res = rt
        .execute(ToolCallRequest {
            call_id: "prov011".into(),
            tool: "evidence.review_provenance".into(),
            args: args(),
            auth: None,
        })
        .await
        .unwrap();
    assert_eq!(res.output["independently_verified"], true);
    assert_eq!(res.output["execution_authorized"], false);
}
#[tokio::test]
async fn model_cannot_supply_own_verifier_trust() {
    let rt = OcafRuntime::new(0);
    rt.register(manifest(), AgentTool::new(BTreeSet::new()))
        .await
        .unwrap();
    let mut a = args();
    a["trusted_verifiers"] = serde_json::json!(["review:human456"]);
    assert!(rt
        .execute(ToolCallRequest {
            call_id: "forge011".into(),
            tool: "evidence.review_provenance".into(),
            args: a,
            auth: None
        })
        .await
        .is_err());
}
#[tokio::test]
async fn unsigned_peer_denied() {
    let rt = OcafRuntime::new(0)
        .with_tenant_namespace("tenant1")
        .unwrap();
    rt.register(manifest(), AgentTool::new(BTreeSet::new()))
        .await
        .unwrap();
    let result = rt
        .execute_authenticated_peer(
            ToolCallRequest {
                call_id: "unsigned011".into(),
                tool: "evidence.review_provenance".into(),
                args: args(),
                auth: None,
            },
            "peer",
        )
        .await;
    assert!(matches!(result, Err(RuntimeError::AuthRequired(_))));
}
