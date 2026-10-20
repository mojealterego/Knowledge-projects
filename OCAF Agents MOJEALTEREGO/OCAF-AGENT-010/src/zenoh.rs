//! Logical route contract. OMAs and OMAt servers are not deployed.
use crate::security::{safe_segment,safe_tool_name,SecurityError};
#[derive(Debug, Copy, Clone)]
pub enum ServerRole { OmasAgents, OmatTools }
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
#[cfg(feature="ocaf-integration")]
mod ocaf {
    use crate::agent::{record_evidence,EvidenceInput};
    use ocaf_runtime::executor::{ExecutionContext,ToolError,ToolExecutor};
    use ocaf_runtime::registry::ToolManifest;
    use serde_json::{json,Value};
    pub struct AgentTool;
    pub fn manifest() -> ToolManifest {
        ToolManifest {
            name:"osint.provenance_record".into(),
            description:"Read-only local policy assessment; no authoritative external actions".into(),
            input_schema: json!({"type":"object","additionalProperties":false,"properties":{"claim":{"type":"string","minLength":1,"maxLength":2048},"origin":{"enum":["simulated","observed","verified"]},"scope_id":{"type":["string","null"]},"source_ref":{"type":["string","null"]},"observed_at":{"type":["string","null"]},"verifier_ref":{"type":["string","null"]},"source_sha256":{"type":["string","null"]}},"required":["claim","origin"]}),
            output_schema: json!({"type":"object","additionalProperties":false,"properties":{"agent_id":{"const":"AM-OCAF-AI-0010"},"claim":{"type":"string"},"origin":{"type":"string"},"scope_id":{"type":["string","null"]},"source_ref":{"type":["string","null"]},"observed_at":{"type":["string","null"]},"verifier_ref":{"type":["string","null"]},"source_sha256":{"type":["string","null"]},"ui_badge":{"type":"string"},"independently_verified":{"const":false},"source_contacted":{"const":false}},"required":["agent_id","claim","origin","ui_badge","independently_verified","source_contacted"]}),
            cost_sats:0,
            permissions:vec![],
            timeout_ms:5000,
            enabled:true,
        }
    }
    #[async_trait::async_trait]
    impl ToolExecutor for AgentTool {
        async fn execute(&self, args:Value, _ctx:&ExecutionContext) -> Result<Value,ToolError> {
            if !serde_json::to_vec(&args).is_ok_and(|v|v.len() <= 8192) {
                return Err(ToolError::Validation("oversize untrusted request".into()));
            }
            let input:EvidenceInput=serde_json::from_value(args).map_err(|_|
                ToolError::Validation("invalid structured request".into()))?;
            let result=record_evidence(input)
                .map_err(|e|ToolError::Validation(format!("{e:?}")))?;
            serde_json::to_value(result).map_err(|_|
                ToolError::Validation("encoding failed".into()))
        }
    }
}
#[cfg(feature="ocaf-integration")]
pub use ocaf::{manifest,AgentTool};
