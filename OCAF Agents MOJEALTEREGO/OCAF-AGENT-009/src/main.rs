use cloud_fiscalis_agent::agent::{review_cloud_plan, CloudApproval, CloudPlan};
use cloud_fiscalis_agent::security::load_config;
use rust_decimal::Decimal;
use std::{
    collections::{BTreeMap, BTreeSet},
    str::FromStr,
};

fn main() {
    let cfg = load_config(include_str!("../config/agent.toml")).expect("invalid identity");
    let plan = CloudPlan {
        action: "estimate_cost".into(),
        product: "agent_platform".into(),
        request_id: Some("demo-001".into()),
        project_scope: Some("demo-sandbox".into()),
        maximum_cost_usd: "0.25".into(),
        data_classification: None,
        credentials_in_prompt: Some(false),
        explicit_external_side_effect_approval: Some(false),
    };
    let grant = CloudApproval {
        project_scope: "demo-sandbox".into(),
        allowed_actions: BTreeSet::from(["estimate_cost".into()]),
        allowed_products: BTreeSet::from(["agent_platform".into()]),
        max_cost_usd: Decimal::from_str("1.00").unwrap(),
        human_reviewed: true,
    };
    let result = review_cloud_plan(&plan, &BTreeMap::from([("demo-001".into(), grant)]));
    println!(
        "{} | {} | allowed={} | execution_authorized={}",
        cfg.identity.unique_name,
        cfg.identity.serial_number,
        result.allowed,
        result.execution_authorized
    );
}
