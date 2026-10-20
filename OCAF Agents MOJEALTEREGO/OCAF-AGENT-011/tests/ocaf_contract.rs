#![cfg(feature = "ocaf-integration")]
use limes_git_agent::zenoh::{manifest, AgentTool};
use ocaf_runtime::runtime::{OcafRuntime, RuntimeError, ToolCallRequest};

fn demo() -> serde_json::Value {
    serde_json::json!({"repository_full_name":"mojealterego/Knowledge-projects",
        "operation":"update_ref","target_branch":"main"})
}
#[tokio::test]
async fn direct_write_to_main_is_denied_by_local_preflight() {
    let rt = OcafRuntime::new(0);
    rt.register(manifest(), AgentTool).await.unwrap();
    let res = rt
        .execute(ToolCallRequest {
            call_id: "test011".into(),
            tool: "github.scope_review".into(),
            args: demo(),
            auth: None,
        })
        .await
        .unwrap();
    assert_eq!(res.output["permitted"], false);
    assert_eq!(res.output["execution_authorized"], false);
    assert_eq!(res.output["api_called"], false);
}
#[tokio::test]
async fn model_supplied_grant_is_not_an_input_property() {
    let rt = OcafRuntime::new(0);
    rt.register(manifest(), AgentTool).await.unwrap();
    let mut p = demo();
    p["is_admin"] = serde_json::json!(true);
    assert!(rt
        .execute(ToolCallRequest {
            call_id: "spoof011".into(),
            tool: "github.scope_review".into(),
            args: p,
            auth: None
        })
        .await
        .is_err());
}
#[tokio::test]
async fn unsigned_remote_peer_is_denied() {
    let rt = OcafRuntime::new(0)
        .with_tenant_namespace("tenant1")
        .unwrap();
    rt.register(manifest(), AgentTool).await.unwrap();
    let result = rt
        .execute_authenticated_peer(
            ToolCallRequest {
                call_id: "peer011".into(),
                tool: "github.scope_review".into(),
                args: demo(),
                auth: None,
            },
            "peer",
        )
        .await;
    assert!(matches!(result, Err(RuntimeError::AuthRequired(_))));
}
