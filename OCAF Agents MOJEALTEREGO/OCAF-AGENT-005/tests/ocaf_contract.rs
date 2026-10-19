#![cfg(feature = "ocaf-integration")]
use ocaf_runtime::runtime::{OcafRuntime, RuntimeError, ToolCallRequest};
use argus_mobile_agent::zenoh::{manifest, AgentTool};

#[tokio::test]
async fn local_ocaf_executes_bounded_read_only_review() {
    let runtime = OcafRuntime::new(0);
    runtime.register(manifest(), AgentTool).await.unwrap();
    let result = runtime
        .execute(ToolCallRequest {
            call_id: "test-005".into(),
            tool: "ui.proposal_assess".into(),
            args: serde_json::json!({"session_id":"session1","snapshot_id":"snapshot1","node_id":"node1","action":"CLICK","operator_grant":true,"expected_postcondition":"visible change"}),
            auth: None,
        })
        .await
        .unwrap();
    assert_eq!(result.output["agent_id"], "AM-OCAF-AI-0005");
    assert_eq!(result.output["verified"], false);
    assert_eq!(result.output["execution_authorized"], false);
}

#[tokio::test]
async fn unsigned_remote_peer_is_rejected() {
    let runtime = OcafRuntime::new(0)
        .with_tenant_namespace("tenant1")
        .unwrap();
    runtime.register(manifest(), AgentTool).await.unwrap();
    let result = runtime
        .execute_authenticated_peer(
            ToolCallRequest {
                call_id: "unsigned-005".into(),
                tool: "ui.proposal_assess".into(),
                args: serde_json::json!({"session_id":"session1","snapshot_id":"snapshot1","node_id":"node1","action":"CLICK","operator_grant":true,"expected_postcondition":"visible change"}),
                auth: None,
            },
            "peer1",
        )
        .await;
    assert!(matches!(result, Err(RuntimeError::AuthRequired(_))));
}
