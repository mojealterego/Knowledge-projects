#![cfg(feature = "ocaf-integration")]

use ocaf_runtime::runtime::{OcafRuntime, RuntimeError, ToolCallRequest};
use serde_json::json;
use wda_photo_agent::ocaf::{manifest, WdaPlanExecutor};

#[tokio::test]
async fn can_register_and_execute_read_only_tool() {
    let runtime = OcafRuntime::new(0);
    runtime.register(manifest(), WdaPlanExecutor).await.unwrap();
    let result = runtime
        .execute(ToolCallRequest {
            call_id: "plan-001".to_owned(),
            tool: "photo.plan".to_owned(),
            args: json!({"goal": "adjust exposure", "identity_lock": true}),
            auth: None,
        })
        .await
        .unwrap();
    assert_eq!(result.output["agent_id"], "AM-OCAF-AI-0001");
    assert_eq!(result.output["status"], "plan_only");
}

#[tokio::test]
async fn input_schema_rejects_unknown_fields_and_unlock() {
    let runtime = OcafRuntime::new(0);
    runtime.register(manifest(), WdaPlanExecutor).await.unwrap();
    for args in [
        json!({"goal": "edit", "identity_lock": false}),
        json!({"goal": "edit", "identity_lock": true, "unexpected": "mutate"}),
    ] {
        let response = runtime
            .execute(ToolCallRequest {
                call_id: "denied-001".to_owned(),
                tool: "photo.plan".to_owned(),
                args,
                auth: None,
            })
            .await;
        assert!(matches!(response, Err(RuntimeError::InvalidInput(_))));
    }
}

#[tokio::test]
async fn pinned_tenant_rejects_unsigned_peer_execution() {
    let runtime = OcafRuntime::new(0)
        .with_tenant_namespace("tenant001")
        .unwrap();
    runtime.register(manifest(), WdaPlanExecutor).await.unwrap();
    let result = runtime
        .execute_authenticated_peer(
            ToolCallRequest {
                call_id: "rpc-denied-001".to_owned(),
                tool: "photo.plan".to_owned(),
                args: json!({"goal": "edit", "identity_lock": true}),
                auth: None,
            },
            "peer001",
        )
        .await;
    assert!(matches!(result, Err(RuntimeError::AuthRequired(_))));
}
