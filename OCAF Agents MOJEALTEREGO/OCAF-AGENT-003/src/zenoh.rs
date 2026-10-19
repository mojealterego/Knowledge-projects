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

//! Local adapter for pinned OCAF Runtime ToolExecutor.
//! "OMAs" and "OMAt" are external logical roles; transport is OFF by default.
#[cfg(feature = "ocaf-integration")]
mod ocaf {
    use crate::agent::{assess, ResearchInput};
    use ocaf_runtime::executor::{ExecutionContext, ToolError, ToolExecutor};
    use ocaf_runtime::registry::ToolManifest;
    use serde_json::{json, Value};

    pub struct AgentTool;

    pub fn manifest() -> ToolManifest {
        ToolManifest {
            name: "research.assess".to_owned(),
            description: "Read-only, unverified offline assessment (no external side effects)".into(),
            input_schema: json!({
"type":"object","additionalProperties":false,
"properties":{
"claim":{"type":"string","minLength":1,"maxLength":1024},
"evidence":{"type":"array","minItems":2,"maxItems":8,
"items":{"type":"object","additionalProperties":false,
"properties":{
"source_id":{"type":"string","minLength":1,"maxLength":64},
"stance":{"enum":["supports","contradicts","unclear"]},
"excerpt":{"type":"string","minLength":1,"maxLength":256}
},"required":["source_id","stance","excerpt"]}}
},"required":["claim","evidence"]}),
            output_schema: json!({
"type":"object","additionalProperties":false,
"properties":{
"agent_id":{"const":"AM-OCAF-AI-0003"},"status":{"enum":["disputed","unverified"]},
"source_count":{"type":"integer"},"has_contradiction":{"type":"boolean"},
"verified":{"const":false},"conclusions":{"type":"array","minItems":3,"items":{"type":"string"}}
},"required":["agent_id","status","source_count","has_contradiction","verified","conclusions"]}),
            cost_sats: 0,
            permissions: vec![],
            timeout_ms: 5_000,
            enabled: true,
        }
    }

    #[async_trait::async_trait]
    impl ToolExecutor for AgentTool {
        async fn execute(&self, args: Value, _ctx: &ExecutionContext) -> Result<Value, ToolError> {
            if !serde_json::to_vec(&args).is_ok_and(|encoded| encoded.len() <= 8192) {
                return Err(ToolError::Validation("input exceeds 8192 serialized bytes".into()));
            }
            let input: ResearchInput = serde_json::from_value(args)
                .map_err(|_| ToolError::Validation("invalid structured input".into()))?;
            let result = assess(&input).map_err(|e| ToolError::Validation(format!("{e:?}")))?;
            serde_json::to_value(result).map_err(|_| ToolError::Validation("encoding failed".into()))
        }
    }
}
#[cfg(feature = "ocaf-integration")]
pub use ocaf::{manifest, AgentTool};
