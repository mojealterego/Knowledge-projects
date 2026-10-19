#![cfg(feature = "ocaf-integration")]
use vigilis_sre_agent::zenoh::{manifest, AgentTool};
use ocaf_runtime::runtime::{OcafRuntime, RuntimeError, ToolCallRequest};

#[tokio::test]
async fn local_ocaf_runtime_executes_the_bounded_read_only_tool() {
    let runtime = OcafRuntime::new(0);
    runtime.register(manifest(), AgentTool).await.unwrap();
    let result = runtime
        .execute(ToolCallRequest {
            call_id: "trial-002".into(),
            tool: "incident.triage".into(),
            args: serde_json::json!({"incident_id":"I1","tenant_id":"team1","signal":"CrashLoopBackOff","observations":["restarts"],"risk_tier":"R3"}),
            auth: None,
        })
        .await
        .unwrap();
    assert_eq!(result.output["agent_id"], "AM-OCAF-AI-0002");
    assert_eq!(result.output["verified"], false);
}

#[tokio::test]
async fn signed_omas_policy_rejects_unsigned_remote_peer() {
    let runtime = OcafRuntime::new(0)
        .with_tenant_namespace("tenant1")
        .unwrap();
    runtime.register(manifest(), AgentTool).await.unwrap();
    let result = runtime
        .execute_authenticated_peer(
            ToolCallRequest {
                call_id: "no-signature-002".into(),
                tool: "incident.triage".into(),
                args: serde_json::json!({"incident_id":"I1","tenant_id":"team1","signal":"CrashLoopBackOff","observations":["restarts"],"risk_tier":"R3"}),
                auth: None,
            },
            "peer1",
        )
        .await;
    assert!(matches!(result, Err(RuntimeError::AuthRequired(_))));
}
