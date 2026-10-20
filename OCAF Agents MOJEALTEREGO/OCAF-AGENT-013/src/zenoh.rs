//! Logical OMAs/OMAt keyspaces only. Neither server nor Zenoh session is started.
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
    use crate::agent::{make_record, EvidenceInput};
    use ocaf_runtime::executor::{ExecutionContext, ToolError, ToolExecutor};
    use ocaf_runtime::registry::ToolManifest;
    use serde_json::{json, Value};
    use std::collections::BTreeSet;

    /// Supplied by operator policy service, not from the agent JSON.
    pub struct AgentTool {
        trusted_verifiers: BTreeSet<String>,
    }
    impl AgentTool {
        pub fn new(trusted_verifiers: BTreeSet<String>) -> Self {
            Self { trusted_verifiers }
        }
    }
    pub fn manifest() -> ToolManifest {
        ToolManifest {
            name: "evidence.review_provenance".into(),
            description: "Non-executing provenance labels and mandatory trusted-verifier scope"
                .into(),
            input_schema: json!({
                "type":"object","additionalProperties":false,
                "properties":{
                    "claim":{"type":"string","minLength":1,"maxLength":4096},
                    "origin":{"enum":["simulated","observed","verified"]},
                    "scope_id":{"type":["string","null"]},
                    "source_ref":{"type":["string","null"]},
                    "observed_at":{"type":["string","null"]},
                    "verifier_ref":{"type":["string","null"]},
                    "source_sha256":{"type":["string","null"]}
                },
                "required":["claim","origin"]
            }),
            output_schema: json!({
                "type":"object","properties":{
                    "agent_id":{"const":"AM-OCAF-AI-0013"},
                    "claim":{"type":"string"},
                    "origin":{"enum":["simulated","observed","verified"]},
                    "ui_badge":{"type":"string"},
                    "independently_verified":{"type":"boolean"},
                    "execution_authorized":{"const":false}
                },
                "required":["agent_id","claim","origin","ui_badge",
                    "independently_verified","execution_authorized"]
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
            if !serde_json::to_vec(&args).is_ok_and(|v| v.len() <= 8192) {
                return Err(ToolError::Validation("record too large".into()));
            }
            let request: EvidenceInput = serde_json::from_value(args)
                .map_err(|_| ToolError::Validation("invalid evidence record".into()))?;
            let result = make_record(&request, &self.trusted_verifiers)
                .map_err(|e| ToolError::Validation(format!("evidence rejected: {e:?}")))?;
            serde_json::to_value(result)
                .map_err(|_| ToolError::Validation("encoding failure".into()))
        }
    }
}
#[cfg(feature = "ocaf-integration")]
pub use ocaf::{manifest, AgentTool};
