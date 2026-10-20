//! Local OMAs/OMAt route key validation. No Zenoh sockets or remote RPC.
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
    use crate::agent::{review_link, LinkPlan};
    use ocaf_runtime::executor::{ExecutionContext, ToolError, ToolExecutor};
    use ocaf_runtime::registry::ToolManifest;
    use serde_json::{json, Value};
    pub struct AgentTool;
    pub fn manifest() -> ToolManifest {
        ToolManifest {
            name: "communication.review_link".into(),
            description:
                "Offline consent and protocol planning; no device pairing, GPS or scanning".into(),
            input_schema: json!({
                "type":"object","additionalProperties":false,
                "properties":{
                    "protocol":{"type":"string"},"purpose":{"type":"string"},
                    "endpoint_owner_authorized":{"type":["boolean","null"]},
                    "counterparty_opted_in":{"type":["boolean","null"]},
                    "passive_tracking":{"type":["boolean","null"]},
                    "network_scan":{"type":["boolean","null"]},
                    "claims_global_device_location":{"type":["boolean","null"]},
                    "cloud_publication":{"type":["boolean","null"]}
                },
                "required":["protocol","purpose"]
            }),
            output_schema: json!({
                "type":"object","properties":{
                    "agent_id":{"const":"AM-OCAF-AI-0013"},
                    "accepted":{"type":"boolean"},
                    "reasons":{"type":"array","items":{"type":"string"}},
                    "has_physical_location_evidence":{"const":false},
                    "execution_authorized":{"const":false},
                    "device_contacted":{"const":false}
                },
                "required":["agent_id","accepted","reasons","has_physical_location_evidence","execution_authorized","device_contacted"]
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
                return Err(ToolError::Validation("oversize link plan".into()));
            }
            let input: LinkPlan = serde_json::from_value(args)
                .map_err(|_| ToolError::Validation("invalid link plan".into()))?;
            serde_json::to_value(review_link(&input))
                .map_err(|_| ToolError::Validation("failed to encode verdict".into()))
        }
    }
}
#[cfg(feature = "ocaf-integration")]
pub use ocaf::{manifest, AgentTool};
