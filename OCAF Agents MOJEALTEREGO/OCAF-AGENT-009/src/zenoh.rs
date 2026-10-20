//! Key-expression validation only; OMAs and OMAt are separate external servers.
use crate::security::{safe_segment, safe_tool_name, SecurityError};

#[derive(Debug, Copy, Clone)]
pub enum ServerRole {
    OmasAgents,
    OmatTools,
}
pub fn route(role: ServerRole, tenant: &str, name: &str) -> Result<String, SecurityError> {
    safe_segment(tenant)?;
    match role {
        ServerRole::OmasAgents => {
            safe_segment(name)?;
            Ok(format!("omas/v1/{tenant}/agents/{name}"))
        }
        ServerRole::OmatTools => {
            safe_tool_name(name)?;
            Ok(format!("omat/v1/{tenant}/tools/{name}"))
        }
    }
}

#[cfg(feature = "ocaf-integration")]
mod ocaf {
    use crate::agent::{review_cloud_plan, CloudApproval, CloudPlan};
    use ocaf_runtime::executor::{ExecutionContext, ToolError, ToolExecutor};
    use ocaf_runtime::registry::ToolManifest;
    use serde_json::{json, Value};
    use std::collections::BTreeMap;

    pub struct AgentTool {
        grants: BTreeMap<String, CloudApproval>,
    }
    impl AgentTool {
        pub fn new(grants: BTreeMap<String, CloudApproval>) -> Self {
            Self { grants }
        }
    }
    pub fn manifest() -> ToolManifest {
        ToolManifest {
            name: "cloud.review_budget".into(),
            description: "Offline budget policy; never an IAM, billing or API client".into(),
            input_schema: json!({
                "type":"object","additionalProperties":false,
                "properties":{
                    "action":{"type":"string"},
                    "product":{"type":"string"},
                    "request_id":{"type":["string","null"]},
                    "project_scope":{"type":["string","null"]},
                    "maximum_cost_usd":{"type":"string"},
                    "data_classification":{"type":["string","null"]},
                    "credentials_in_prompt":{"type":["boolean","null"]},
                    "explicit_external_side_effect_approval":{"type":["boolean","null"]}
                },
                "required":["action","product","maximum_cost_usd"]
            }),
            output_schema: json!({
                "type":"object",
                "properties":{
                    "agent_id":{"const":"AM-OCAF-AI-0009"},
                    "allowed":{"type":"boolean"},
                    "reasons":{"type":"array","items":{"type":"string"}},
                    "execution_authorized":{"const":false},
                    "charge_incurred":{"const":false}
                },
                "required":["agent_id","allowed","reasons","execution_authorized","charge_incurred"]
            }),
            cost_sats: 0,
            permissions: vec![],
            timeout_ms: 5000,
            enabled: true,
        }
    }
    #[async_trait::async_trait]
    impl ToolExecutor for AgentTool {
        async fn execute(&self, args: Value, _ctx: &ExecutionContext) -> Result<Value, ToolError> {
            if !serde_json::to_vec(&args).is_ok_and(|v| v.len() <= 4096) {
                return Err(ToolError::Validation("oversize cloud plan".into()));
            }
            let plan: CloudPlan = serde_json::from_value(args)
                .map_err(|_| ToolError::Validation("invalid cloud plan".into()))?;
            serde_json::to_value(review_cloud_plan(&plan, &self.grants))
                .map_err(|_| ToolError::Validation("encoding failed".into()))
        }
    }
}
#[cfg(feature = "ocaf-integration")]
pub use ocaf::{manifest, AgentTool};
