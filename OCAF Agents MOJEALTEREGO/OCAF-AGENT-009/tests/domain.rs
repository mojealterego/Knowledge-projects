use cloud_fiscalis_agent::agent::{review_cloud_plan, CloudApproval, CloudPlan};
use rust_decimal::Decimal;
use std::collections::{BTreeMap, BTreeSet};
use std::str::FromStr;

fn good() -> (CloudPlan, BTreeMap<String, CloudApproval>) {
    let plan = CloudPlan {
        action: "invoke_model".into(),
        product: "agent_platform".into(),
        request_id: Some("ticket-001".into()),
        project_scope: Some("approved-sandbox".into()),
        maximum_cost_usd: "0.40".into(),
        data_classification: None,
        credentials_in_prompt: Some(false),
        explicit_external_side_effect_approval: Some(true),
    };
    let grant = CloudApproval {
        project_scope: "approved-sandbox".into(),
        allowed_actions: BTreeSet::from(["estimate_cost".into(), "invoke_model".into()]),
        allowed_products: BTreeSet::from(["agent_platform".into()]),
        max_cost_usd: Decimal::from_str("1.50").unwrap(),
        human_reviewed: true,
    };
    (plan, BTreeMap::from([("ticket-001".into(), grant)]))
}
#[test]
fn legitimate_bounded_cloud_plan_is_only_a_nonexecuting_approval() {
    let (p, grants) = good();
    let verdict = review_cloud_plan(&p, &grants);
    assert!(verdict.allowed, "{:?}", verdict.reasons);
    assert!(!verdict.execution_authorized);
}
#[test]
fn fraudulently_asserted_approval_does_not_authorize_a_cloud_payment() {
    let (mut p, grants) = good();
    p.action = "make_payment".into();
    assert!(review_cloud_plan(&p, &grants)
        .reasons
        .contains(&"operation_not_authorized".to_owned()));
}
#[test]
fn quota_bypass_attempts_and_nan_are_denied() {
    let (mut p, grants) = good();
    p.maximum_cost_usd = "Infinity".into();
    assert!(review_cloud_plan(&p, &grants)
        .reasons
        .contains(&"bad_cost".to_owned()));
    p.maximum_cost_usd = "1.51".into();
    assert!(review_cloud_plan(&p, &grants)
        .reasons
        .contains(&"over_budget".to_owned()));
}
#[test]
fn public_documentation_is_zero_cost_and_public_only() {
    let (mut p, _) = good();
    p.action = "describe_public_documentation".into();
    p.product = "vision_ai".into();
    p.maximum_cost_usd = "0".into();
    p.data_classification = Some("public".into());
    assert!(review_cloud_plan(&p, &BTreeMap::new()).allowed);
    p.maximum_cost_usd = "0.01".into();
    assert!(review_cloud_plan(&p, &BTreeMap::new())
        .reasons
        .contains(&"public_docs_must_be_free".to_owned()));
}
