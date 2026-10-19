//! OMAs agent server vs OMAt tool server. These are key-expression contracts,
//! NOT operational network sessions, credentials or production services.
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

// Local pinned OCAF ToolExecutor adapter; does not start external services.
#[cfg(feature = "ocaf-integration")]
mod ocaf {
    use crate::agent::{assess, ActionProposal};
    use ocaf_runtime::executor::{ExecutionContext, ToolError, ToolExecutor};
    use ocaf_runtime::registry::ToolManifest;
    use serde_json::{json, Value};

    pub struct AgentTool;

    pub fn manifest() -> ToolManifest {
        ToolManifest {
            name: "ui.proposal_assess".to_owned(),
            description: "Offline Android UI action policy proposal review derived from P119"
                .into(),
            input_schema: json!({"type":"object","additionalProperties":false,"properties":{"session_id":{"type":"string","minLength":1,"maxLength":64},"snapshot_id":{"type":"string","minLength":1,"maxLength":64},"node_id":{"type":"string","minLength":1,"maxLength":64},"action":{"enum":["CLICK","SCROLL","BACK","HOME","TYPE","DELETE","SEND","GRANT_PERMISSION"]},"operator_grant":{"type":"boolean"},"expected_postcondition":{"type":"string","minLength":1,"maxLength":256}},"required":["session_id","snapshot_id","node_id","action","operator_grant","expected_postcondition"]}),
            output_schema: json!({"type":"object","additionalProperties":false,"properties":{"agent_id":{"const":"AM-OCAF-AI-0005"},"status":{"enum":["blocked","requires_grant","operator_review"]},"execution_authorized":{"const":false},"readback_required":{"const":true},"verified":{"const":false},"reasons":{"type":"array","minItems":3,"items":{"type":"string"}}},"required":["agent_id","status","execution_authorized","readback_required","verified","reasons"]}),
            cost_sats: 0,
            permissions: vec![],
            timeout_ms: 5_000,
            enabled: true,
        }
    }

    #[async_trait::async_trait]
    impl ToolExecutor for AgentTool {
        async fn execute(&self, args: Value, _ctx: &ExecutionContext) -> Result<Value, ToolError> {
            if !serde_json::to_vec(&args).is_ok_and(|bytes| bytes.len() <= 8192) {
                return Err(ToolError::Validation("input exceeds 8192 bytes".into()));
            }
            let input: ActionProposal = serde_json::from_value(args)
                .map_err(|_| ToolError::Validation("invalid request schema".into()))?;
            let result = assess(&input).map_err(|e| ToolError::Validation(format!("{e:?}")))?;
            serde_json::to_value(result)
                .map_err(|_| ToolError::Validation("encoding failure".into()))
        }
    }
}
#[cfg(feature = "ocaf-integration")]
pub use ocaf::{manifest, AgentTool};
