//! Local OCAF ToolExecutor; OMAs/OMAt are separate *external* services.
//! Routes are only validated keys. No authenticated transport is started.
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
    use crate::agent::{admit, ToolManifest};
    use chrono::{DateTime, Utc};
    use ocaf_runtime::executor::{ExecutionContext, ToolError, ToolExecutor};
    use ocaf_runtime::registry::ToolManifest as OcafToolManifest;
    use serde::Deserialize;
    use serde_json::{json, Value};
    use std::collections::{BTreeMap, BTreeSet};

    /// Supplied by the trusted host, not by JSON from the agent/model.
    pub struct AgentTool {
        approvals: BTreeMap<String, BTreeSet<String>>,
        now: DateTime<Utc>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Input {
        manifest: ToolManifest,
        artifact_hex: String,
    }

    impl AgentTool {
        pub fn new(
            trusted_approvals: BTreeMap<String, BTreeSet<String>>,
            now: DateTime<Utc>,
        ) -> Self {
            Self { approvals: trusted_approvals, now }
        }
    }

    pub fn manifest() -> OcafToolManifest {
        OcafToolManifest {
            name: "tool.admit".into(),
            description: "Non-executing gate; approvals must originate from an independent trusted host".into(),
            input_schema: json!({
                "type":"object",
                "additionalProperties": false,
                "properties": {
                    "manifest": {
                        "type":"object",
                        "additionalProperties":false,
                        "properties":{
                            "artifact_sha256":{"type":"string","minLength":64,"maxLength":64},
                            "capabilities":{"type":"array","minItems":1,"maxItems":4,"items":{"type":"string"}},
                            "sandbox_profile":{"type":"string"},
                            "timeout_seconds":{"type":"integer","minimum":1,"maximum":60},
                            "expires_at":{"type":"string"},
                            "external_side_effects":{"type":"boolean"}
                        },
                        "required":["artifact_sha256","capabilities","sandbox_profile","timeout_seconds","expires_at","external_side_effects"]
                    },
                    "artifact_hex":{"type":"string","maxLength":2097152}
                },
                "required":["manifest","artifact_hex"]
            }),
            output_schema: json!({
                "type":"object",
                "properties":{
                    "agent_id":{"const":"AM-OCAF-AI-0006"},
                    "accepted":{"type":"boolean"},
                    "status":{"enum":["admissible","rejected"]},
                    "reasons":{"type":"array","items":{"type":"string"}},
                    "artifact_sha256":{"type":"string"},
                    "execution_authorized":{"const":false},
                    "sandbox_started":{"const":false}
                },
                "required":["agent_id","accepted","status","reasons","artifact_sha256","execution_authorized","sandbox_started"]
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
            if !serde_json::to_vec(&args).is_ok_and(|bytes| bytes.len() <= 2_100_000) {
                return Err(ToolError::Validation("too much input data".into()));
            }
            let input: Input = serde_json::from_value(args)
                .map_err(|_| ToolError::Validation("invalid manifest or untrusted fields".into()))?;
            let bytes = hex::decode(&input.artifact_hex)
                .map_err(|_| ToolError::Validation("artifact_hex is not valid hex".into()))?;
            let result = admit(&input.manifest, &bytes, &self.approvals, self.now);
            serde_json::to_value(result)
                .map_err(|_| ToolError::Validation("encoding failed".into()))
        }
    }
}

#[cfg(feature = "ocaf-integration")]
pub use ocaf::{manifest, AgentTool};
