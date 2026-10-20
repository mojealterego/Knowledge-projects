#![cfg(feature = "ocaf-integration")]
use nexus_consent_agent::zenoh::{manifest, AgentTool};
use ocaf_runtime::runtime::{OcafRuntime, RuntimeError, ToolCallRequest};
fn sample() -> serde_json::Value {
    serde_json::json!({"protocol":"bluetooth_le","purpose":"owned_device_transfer",
        "endpoint_owner_authorized":true,"counterparty_opted_in":true,
        "passive_tracking":false,"network_scan":false,"claims_global_device_location":false,
        "cloud_publication":false})
}
#[tokio::test]
async fn local_review_does_not_touch_devices() {
    let rt = OcafRuntime::new(0);
    rt.register(manifest(), AgentTool).await.unwrap();
    let r = rt
        .execute(ToolCallRequest {
            call_id: "t013".into(),
            tool: "communication.review_link".into(),
            args: sample(),
            auth: None,
        })
        .await
        .unwrap();
    assert_eq!(r.output["accepted"], true);
    assert_eq!(r.output["device_contacted"], false);
    assert_eq!(r.output["execution_authorized"], false);
}
#[tokio::test]
async fn injected_location_proof_is_rejected() {
    let rt = OcafRuntime::new(0);
    rt.register(manifest(), AgentTool).await.unwrap();
    let mut args = sample();
    args["verified_lat"] = serde_json::json!(12.5);
    assert!(rt
        .execute(ToolCallRequest {
            call_id: "spoof013".into(),
            tool: "communication.review_link".into(),
            args,
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
    rt.register(manifest(), AgentTool).await.unwrap();
    let r = rt
        .execute_authenticated_peer(
            ToolCallRequest {
                call_id: "unsigned013".into(),
                tool: "communication.review_link".into(),
                args: sample(),
                auth: None,
            },
            "peer",
        )
        .await;
    assert!(matches!(r, Err(RuntimeError::AuthRequired(_))));
}
