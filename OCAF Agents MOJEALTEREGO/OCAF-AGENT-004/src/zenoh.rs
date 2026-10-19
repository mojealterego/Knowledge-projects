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
    use crate::agent::{review, PlanInput};
    use ocaf_runtime::executor::{ExecutionContext, ToolError, ToolExecutor};
    use ocaf_runtime::registry::ToolManifest;
    use serde_json::{json, Value};

    pub struct AgentTool;

    pub fn manifest() -> ToolManifest {
        ToolManifest {
            name: "graph.review".to_owned(),
            description: "Local task dependency and cycle review derived from P115".into(),
            input_schema: json!({"type":"object","additionalProperties":false,"properties":{"graph_id":{"type":"string","minLength":1,"maxLength":64},"tasks":{"type":"array","minItems":1,"maxItems":32,"items":{"type":"object","additionalProperties":false,"properties":{"task_id":{"type":"string","minLength":1,"maxLength":64},"dependencies":{"type":"array","maxItems":16,"items":{"type":"string","minLength":1,"maxLength":64}},"stage":{"enum":["plan","build","verify","release"]}},"required":["task_id","dependencies","stage"]}}},"required":["graph_id","tasks"]}),
            output_schema: json!({"type":"object","additionalProperties":false,"properties":{"agent_id":{"const":"AM-OCAF-AI-0004"},"status":{"enum":["blocked","structurally_valid"]},"task_count":{"type":"integer"},"order":{"type":"array","items":{"type":"string"}},"issues":{"type":"array","items":{"type":"string"}},"execution_authorized":{"const":false},"verified":{"const":false}},"required":["agent_id","status","task_count","order","issues","execution_authorized","verified"]}),
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
            let input: PlanInput = serde_json::from_value(args)
                .map_err(|_| ToolError::Validation("invalid request schema".into()))?;
            let result = review(&input).map_err(|e| ToolError::Validation(format!("{e:?}")))?;
            serde_json::to_value(result)
                .map_err(|_| ToolError::Validation("encoding failure".into()))
        }
    }
}
#[cfg(feature = "ocaf-integration")]
pub use ocaf::{manifest, AgentTool};
