#![cfg(feature = "ocaf-integration")]

use ocaf_runtime::runtime::{OcafRuntime, RuntimeError, ToolCallRequest};
use serde_json::json;
use wda_photo_agent::zenoh::{manifest, route, ServerRole, WdaPlanExecutor};

#[tokio::test]
async fn read_only_plan_registers_in_ocaf_runtime() {
    let runtime = OcafRuntime::new(0);
    runtime.register(manifest(), WdaPlanExecutor).await.unwrap();
    let reply = runtime
        .execute(ToolCallRequest {
            call_id: "agent001-local".to_owned(),
            tool: "photo.plan".to_owned(),
            args: json!({"goal":"adjust exposure","identity_lock":true}),
            auth: None,
        })
        .await
        .unwrap();
    assert_eq!(reply.output["agent_id"], "AM-OCAF-AI-0001");
    assert_eq!(reply.output["status"], "plan_only");
    assert!(reply.output["workflow"]
        .as_array()
        .is_some_and(|v| v.len() >= 3));
}

#[tokio::test]
async fn omas_and_omat_have_distinct_namespaces() {
    assert_eq!(
        route(ServerRole::OmasAgents, "tenant001", "OCAF-AGENT-001").unwrap(),
        "omas/v1/tenant001/agents/OCAF-AGENT-001"
    );
    assert_eq!(
        route(ServerRole::OmatTools, "tenant001", "photo.plan").unwrap(),
        "omat/v1/tenant001/tools/photo.plan"
    );
}

#[tokio::test]
async fn pinned_tenant_requires_signed_omas_instead_of_plain_peer_execution() {
    let runtime = OcafRuntime::new(0)
        .with_tenant_namespace("tenant001")
        .unwrap();
    runtime.register(manifest(), WdaPlanExecutor).await.unwrap();
    let result = runtime
        .execute_authenticated_peer(
            ToolCallRequest {
                call_id: "denied-unsigned-001".to_owned(),
                tool: "photo.plan".to_owned(),
                args: json!({"goal":"edit","identity_lock":true}),
                auth: None,
            },
            "peer001",
        )
        .await;
    assert!(matches!(result, Err(RuntimeError::AuthRequired(_))));
}
