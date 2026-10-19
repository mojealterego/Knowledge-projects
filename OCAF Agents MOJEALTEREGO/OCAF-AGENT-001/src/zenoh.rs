//! Zenoh routing contracts for two *separate* external server roles:
//! OMAs = Agent Server; OMAt = Tool Server.
//!
//! No Zenoh socket is opened here. Actual transport is provisioned through an
//! authenticated and authorized OCAF/OMAs gateway. Never interpret a route or
//! untrusted ExecutionContext.agent_id as a proof of identity.

use crate::security::{safe_namespace_part, safe_tool_name, SecurityError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServerRole {
    OmasAgents,
    OmatTools,
}

/// Produces a bounded transport resource key; not an authorization decision.
pub fn route(
    role: ServerRole,
    tenant: &str,
    resource_name: &str,
) -> Result<String, SecurityError> {
    safe_namespace_part(tenant)?;
    let key = match role {
        ServerRole::OmasAgents => {
            safe_namespace_part(resource_name)?;
            format!("omas/v1/{tenant}/agents/{resource_name}")
        }
        ServerRole::OmatTools => {
            safe_tool_name(resource_name)?;
            format!("omat/v1/{tenant}/tools/{resource_name}")
        }
    };
    Ok(key)
}

#[cfg(feature = "ocaf-integration")]
mod bridge {
    use crate::agent::plan;
    use ocaf_runtime::executor::{ExecutionContext, ToolError, ToolExecutor};
    use ocaf_runtime::registry::ToolManifest;
    use serde_json::{json, Value};

    pub struct WdaPlanExecutor;

    pub fn manifest() -> ToolManifest {
        ToolManifest {
            name: "photo.plan".to_owned(),
            description: "Identity-preserving read-only photographic plan".to_owned(),
            input_schema: json!({
                "type": "object",
                "additionalProperties": false,
                "properties": {
                    "goal": {"type": "string", "minLength": 1, "maxLength": 1024},
                    "identity_lock": {"const": true}
                },
                "required": ["goal", "identity_lock"]
            }),
            output_schema: json!({
                "type": "object",
                "additionalProperties": false,
                "properties": {
                    "agent_id": {"const": "AM-OCAF-AI-0001"},
                    "status": {"const": "plan_only"},
                    "identity_lock": {"const": true},
                    "workflow": {"type": "array", "items": {"type": "string"}, "minItems": 3}
                },
                "required": ["agent_id", "status", "identity_lock", "workflow"]
            }),
            cost_sats: 0,
            permissions: vec![],
            timeout_ms: 5_000,
            enabled: true,
        }
    }

    #[async_trait::async_trait]
    impl ToolExecutor for WdaPlanExecutor {
        async fn execute(&self, args: Value, _ctx: &ExecutionContext) -> Result<Value, ToolError> {
            let goal = args
                .get("goal")
                .and_then(Value::as_str)
                .ok_or_else(|| ToolError::Validation("goal must be a string".into()))?;
            let identity_lock = args
                .get("identity_lock")
                .and_then(Value::as_bool)
                .ok_or_else(|| ToolError::Validation("identity_lock must be true".into()))?;

            let output = plan(goal, identity_lock)
                .map_err(|error| ToolError::Validation(format!("{error:?}")))?;

            Ok(json!({
                "agent_id": output.agent_id,
                "status": output.status,
                "identity_lock": output.identity_lock,
                "workflow": output.workflow,
            }))
        }
    }
}

#[cfg(feature = "ocaf-integration")]
pub use bridge::{manifest, WdaPlanExecutor};
