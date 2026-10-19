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

// Local adapter for pinned OCAF Runtime ToolExecutor.
// OMAs/OMAt are external roles; transport is OFF by default.
#[cfg(feature = "ocaf-integration")]
mod ocaf {
    use crate::agent::{triage, Incident};
    use ocaf_runtime::executor::{ExecutionContext, ToolError, ToolExecutor};
    use ocaf_runtime::registry::ToolManifest;
    use serde_json::{json, Value};

    pub struct AgentTool;

    pub fn manifest() -> ToolManifest {
        ToolManifest {
            name: "incident.triage".to_owned(),
            description: "Read-only, unverified offline assessment (no external side effects)"
                .into(),
            input_schema: json!({
"type":"object","additionalProperties":false,
"properties":{
"incident_id":{"type":"string","minLength":1,"maxLength":64},
"tenant_id":{"type":"string","minLength":1,"maxLength":64},
"signal":{"enum":["CrashLoopBackOff","NodeNotReady","PodPending","ServiceUnavailable"]},
"observations":{"type":"array","minItems":1,"maxItems":8,"items":{"type":"string","minLength":1,"maxLength":256}},
"risk_tier":{"enum":["R0","R1","R2","R3","R4"]}
},"required":["incident_id","tenant_id","signal","observations","risk_tier"]}),
            output_schema: json!({
"type":"object","additionalProperties":false,
"properties":{
"agent_id":{"const":"AM-OCAF-AI-0002"},"status":{"const":"read_only"},
"signal":{"type":"string"},"risk_tier":{"type":"string"},
"evidence_count":{"type":"integer"},"allow_mutation":{"const":false},
"verified":{"const":false},"recommendations":{"type":"array","minItems":4,"items":{"type":"string"}}
},"required":["agent_id","status","signal","risk_tier","evidence_count","allow_mutation","verified","recommendations"]}),
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
                return Err(ToolError::Validation(
                    "input exceeds 8192 serialized bytes".into(),
                ));
            }
            let input: Incident = serde_json::from_value(args)
                .map_err(|_| ToolError::Validation("invalid structured input".into()))?;
            let result = triage(&input).map_err(|e| ToolError::Validation(format!("{e:?}")))?;
            serde_json::to_value(result)
                .map_err(|_| ToolError::Validation("encoding failed".into()))
        }
    }
}
#[cfg(feature = "ocaf-integration")]
pub use ocaf::{manifest, AgentTool};
