#![cfg(feature = "ocaf-integration")]
use chrono::{TimeZone, Utc};
use custos_admission_agent::zenoh::{manifest, AgentTool};
use ocaf_runtime::runtime::{OcafRuntime, RuntimeError, ToolCallRequest};
use sha2::{Digest, Sha256};
use std::collections::{BTreeMap, BTreeSet};

fn packet() -> (serde_json::Value, BTreeMap<String, BTreeSet<String>>) {
    let bytes = b"safe offline fixture";
    let digest = format!("{:x}", Sha256::digest(bytes));
    let approvals = BTreeMap::from([(
        digest.clone(),
        BTreeSet::from(["parse_document".to_owned()]),
    )]);
    let data = serde_json::json!({
        "manifest":{
            "artifact_sha256":digest, "capabilities":["parse_document"],
            "sandbox_profile":"isolated_no_network_read_only",
            "timeout_seconds":30,
            "expires_at":"2026-10-22T00:00:00Z",
            "external_side_effects":false
        },
        "artifact_hex":hex::encode(bytes)
    });
    (data, approvals)
}

#[tokio::test]
async fn local_host_supplied_approvals_allow_only_static_admission() {
    let (args, approvals) = packet();
    let runtime = OcafRuntime::new(0);
    let now = Utc.with_ymd_and_hms(2026, 10, 20, 12, 0, 0).unwrap();
    runtime
        .register(manifest(), AgentTool::new(approvals, now))
        .await
        .unwrap();
    let result = runtime
        .execute(ToolCallRequest {
            call_id: "static-review-006".into(),
            tool: "tool.admit".into(),
            args,
            auth: None,
        })
        .await
        .unwrap();
    assert_eq!(result.output["accepted"], true);
    assert_eq!(result.output["execution_authorized"], false);
}

#[tokio::test]
async fn model_cannot_insert_its_own_independent_approvals() {
    let (mut args, _) = packet();
    args["trusted_approvals"] = serde_json::json!({"fake":["parse_document"]});
    let runtime = OcafRuntime::new(0);
    let now = Utc.with_ymd_and_hms(2026, 10, 20, 12, 0, 0).unwrap();
    runtime
        .register(manifest(), AgentTool::new(BTreeMap::new(), now))
        .await
        .unwrap();
    let result = runtime
        .execute(ToolCallRequest {
            call_id: "selfapproved-006".into(),
            tool: "tool.admit".into(),
            args,
            auth: None,
        })
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn remote_unsigned_peer_requires_omas_authentication() {
    let (args, approvals) = packet();
    let runtime = OcafRuntime::new(0)
        .with_tenant_namespace("tenant1")
        .unwrap();
    let now = Utc.with_ymd_and_hms(2026, 10, 20, 12, 0, 0).unwrap();
    runtime
        .register(manifest(), AgentTool::new(approvals, now))
        .await
        .unwrap();
    let result = runtime
        .execute_authenticated_peer(
            ToolCallRequest {
                call_id: "unsigned-006".into(),
                tool: "tool.admit".into(),
                args,
                auth: None,
            },
            "peer1",
        )
        .await;
    assert!(matches!(result, Err(RuntimeError::AuthRequired(_))));
}
