//! Logical route contract. OMAs and OMAt servers are not deployed.
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
    use crate::agent::{review_github_operation, GitOperation};
    use ocaf_runtime::executor::{ExecutionContext, ToolError, ToolExecutor};
    use ocaf_runtime::registry::ToolManifest;
    use serde_json::{json, Value};
    pub struct AgentTool;
    pub fn manifest() -> ToolManifest {
        ToolManifest {
            name: "github.scope_review".into(),
            description: "Read-only local policy assessment; no authoritative external actions"
                .into(),
            input_schema: json!({"type":"object","additionalProperties":false,"properties":{"repository_full_name":{"type":"string"},"operation":{"type":"string"},"target_branch":{"type":["string","null"]},"pull_base":{"type":["string","null"]},"pull_head":{"type":["string","null"]}},"required":["repository_full_name","operation"]}),
            output_schema: json!({"type":"object","additionalProperties":false,"properties":{"agent_id":{"const":"AM-OCAF-AI-0011"},"permitted":{"type":"boolean"},"reasons":{"type":"array","items":{"type":"string"}},"execution_authorized":{"const":false},"api_called":{"const":false}},"required":["agent_id","permitted","reasons","execution_authorized","api_called"]}),
            cost_sats: 0,
            permissions: vec![],
            timeout_ms: 5000,
            enabled: true,
        }
    }
    #[async_trait::async_trait]
    impl ToolExecutor for AgentTool {
        async fn execute(&self, args: Value, _ctx: &ExecutionContext) -> Result<Value, ToolError> {
            if !serde_json::to_vec(&args).is_ok_and(|v| v.len() <= 8192) {
                return Err(ToolError::Validation("oversize untrusted request".into()));
            }
            let input: GitOperation = serde_json::from_value(args)
                .map_err(|_| ToolError::Validation("invalid structured request".into()))?;
            let result = review_github_operation(&input);
            serde_json::to_value(result)
                .map_err(|_| ToolError::Validation("encoding failed".into()))
        }
    }
}
#[cfg(feature = "ocaf-integration")]
pub use ocaf::{manifest, AgentTool};
