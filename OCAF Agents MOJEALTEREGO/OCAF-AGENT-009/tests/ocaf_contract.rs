#![cfg(feature = "ocaf-integration")]
use cloud_fiscalis_agent::agent::{CloudApproval, CloudPlan};
use cloud_fiscalis_agent::zenoh::{manifest, AgentTool};
use ocaf_runtime::runtime::{OcafRuntime, RuntimeError, ToolCallRequest};
use rust_decimal::Decimal;
use std::collections::{BTreeMap, BTreeSet};
use std::str::FromStr;

fn grants() -> BTreeMap<String, CloudApproval> {
    BTreeMap::from([(
        "ticket-001".into(),
        CloudApproval {
            project_scope: "safe-sandbox".into(),
            allowed_actions: BTreeSet::from(["estimate_cost".into()]),
            allowed_products: BTreeSet::from(["agent_platform".into()]),
            max_cost_usd: Decimal::from_str("1.00").unwrap(),
            human_reviewed: true,
        },
    )])
}
fn args() -> serde_json::Value {
    serde_json::json!({"action":"estimate_cost","product":"agent_platform",
        "request_id":"ticket-001","project_scope":"safe-sandbox",
        "maximum_cost_usd":"0.25","credentials_in_prompt":false,
        "explicit_external_side_effect_approval":false})
}
#[tokio::test]
async fn bounded_plan_returns_nonexecuting_approval() {
    let rt = OcafRuntime::new(0);
    rt.register(manifest(), AgentTool::new(grants()))
        .await
        .unwrap();
    let res = rt
        .execute(ToolCallRequest {
            call_id: "cost009".into(),
            tool: "cloud.review_budget".into(),
            args: args(),
            auth: None,
        })
        .await
        .unwrap();
    assert_eq!(res.output["allowed"], true);
    assert_eq!(res.output["execution_authorized"], false);
    assert_eq!(res.output["charge_incurred"], false);
}
#[tokio::test]
async fn model_supplied_approval_registry_cannot_bypass_host() {
    let mut p = args();
    p["approved_requests"] = serde_json::json!({"ticket-001":{"human_reviewed":true}});
    let rt = OcafRuntime::new(0);
    rt.register(manifest(), AgentTool::new(BTreeMap::new()))
        .await
        .unwrap();
    assert!(rt
        .execute(ToolCallRequest {
            call_id: "forged009".into(),
            tool: "cloud.review_budget".into(),
            args: p,
            auth: None
        })
        .await
        .is_err());
}
#[tokio::test]
async fn unauthenticated_remote_peer_denied() {
    let rt = OcafRuntime::new(0)
        .with_tenant_namespace("tenant1")
        .unwrap();
    rt.register(manifest(), AgentTool::new(grants()))
        .await
        .unwrap();
    let result = rt
        .execute_authenticated_peer(
            ToolCallRequest {
                call_id: "unsigned009".into(),
                tool: "cloud.review_budget".into(),
                args: args(),
                auth: None,
            },
            "peer",
        )
        .await;
    assert!(matches!(result, Err(RuntimeError::AuthRequired(_))));
}
