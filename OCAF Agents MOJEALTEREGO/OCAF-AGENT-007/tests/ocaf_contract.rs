#![cfg(feature = "ocaf-integration")]
use ocaf_runtime::runtime::{OcafRuntime, RuntimeError, ToolCallRequest};
use vigia_loop_agent::agent::Limits;
use vigia_loop_agent::zenoh::{manifest, AgentTool};

fn args() -> serde_json::Value {
    serde_json::json!({"tool":"read_file","args_sha256":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","estimated_tokens":1})
}
#[tokio::test]
async fn host_owns_guard_and_repeated_calls_are_denied() {
    let runtime = OcafRuntime::new(0);
    runtime
        .register(
            manifest(),
            AgentTool::new(Limits {
                max_calls: 10,
                max_tokens: 100,
                max_identical_no_progress: 2,
            })
            .unwrap(),
        )
        .await
        .unwrap();
    for i in 0..3 {
        let result = runtime
            .execute(ToolCallRequest {
                call_id: format!("loop-{i}"),
                tool: "tool.loop_gate".into(),
                args: args(),
                auth: None,
            })
            .await
            .unwrap();
        if i < 2 {
            assert_eq!(result.output["allowed"], true);
        } else {
            assert_eq!(result.output["reason"], "identical_call_without_progress");
        }
        assert_eq!(result.output["execution_authorized"], false);
    }
}
#[tokio::test]
async fn agent_supplied_progress_version_is_rejected() {
    let runtime = OcafRuntime::new(0);
    runtime
        .register(
            manifest(),
            AgentTool::new(Limits {
                max_calls: 10,
                max_tokens: 100,
                max_identical_no_progress: 2,
            })
            .unwrap(),
        )
        .await
        .unwrap();
    let mut input = args();
    input["trusted_progress_version"] = serde_json::json!(100);
    assert!(runtime
        .execute(ToolCallRequest {
            call_id: "forged-progress".into(),
            tool: "tool.loop_gate".into(),
            args: input,
            auth: None
        })
        .await
        .is_err());
}
#[tokio::test]
async fn unverified_remote_peer_denied() {
    let runtime = OcafRuntime::new(0)
        .with_tenant_namespace("tenant1")
        .unwrap();
    runtime
        .register(
            manifest(),
            AgentTool::new(Limits {
                max_calls: 10,
                max_tokens: 100,
                max_identical_no_progress: 2,
            })
            .unwrap(),
        )
        .await
        .unwrap();
    let r = runtime
        .execute_authenticated_peer(
            ToolCallRequest {
                call_id: "unsigned-loop".into(),
                tool: "tool.loop_gate".into(),
                args: args(),
                auth: None,
            },
            "peer1",
        )
        .await;
    assert!(matches!(r, Err(RuntimeError::AuthRequired(_))));
}
