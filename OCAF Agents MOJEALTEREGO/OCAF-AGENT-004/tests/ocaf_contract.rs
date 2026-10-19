#![cfg(feature = "ocaf-integration")]
use axiom_forge_agent::zenoh::{manifest, AgentTool};
use ocaf_runtime::runtime::{OcafRuntime, RuntimeError, ToolCallRequest};

#[tokio::test]
async fn local_ocaf_executes_bounded_read_only_review() {
    let runtime = OcafRuntime::new(0);
    runtime.register(manifest(), AgentTool).await.unwrap();
    let result = runtime
        .execute(ToolCallRequest {
            call_id: "test-004".into(),
            tool: "graph.review".into(),
            args: serde_json::json!({"graph_id":"g1","tasks":[{"task_id":"test","dependencies":["build"],"stage":"verify"},{"task_id":"build","dependencies":[],"stage":"build"}]}),
            auth: None,
        })
        .await
        .unwrap();
    assert_eq!(result.output["agent_id"], "AM-OCAF-AI-0004");
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
                call_id: "unsigned-004".into(),
                tool: "graph.review".into(),
                args: serde_json::json!({"graph_id":"g1","tasks":[{"task_id":"test","dependencies":["build"],"stage":"verify"},{"task_id":"build","dependencies":[],"stage":"build"}]}),
                auth: None,
            },
            "peer1",
        )
        .await;
    assert!(matches!(result, Err(RuntimeError::AuthRequired(_))));
}
