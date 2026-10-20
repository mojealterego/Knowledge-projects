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
    use crate::agent::audit_bytes;
    use ocaf_runtime::executor::{ExecutionContext, ToolError, ToolExecutor};
    use ocaf_runtime::registry::ToolManifest;
    use serde::Deserialize;
    use serde_json::{json, Value};

    pub struct AgentTool;
    #[derive(Debug, Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Input {
        archive_hex: String,
    }

    pub fn manifest() -> ToolManifest {
        ToolManifest {
            name: "archive.audit_zip".into(),
            description: "Read-only ZIP inventory; no names, file extraction, execution or network"
                .into(),
            input_schema: json!({
                "type":"object","additionalProperties":false,
                "properties":{"archive_hex":{"type":"string","minLength":1,"maxLength":2097152}},
                "required":["archive_hex"]
            }),
            output_schema: json!({
                "type":"object",
                "properties":{
                    "agent_id":{"const":"AM-OCAF-AI-0012"},
                    "source_zip_sha256":{"type":"string"},
                    "status":{"const":"INVENTORIED_NOT_SEMANTICALLY_REVIEWED"},
                    "privacy_names_included":{"const":false},
                    "member_count":{"type":"integer"},
                    "total_uncompressed_bytes":{"type":"integer"},
                    "duplicate_content_hashes":{"type":"object"},
                    "files":{"type":"array"},
                    "semantic_review_performed":{"const":false},
                    "archive_extracted":{"const":false}
                },
                "required":["agent_id","source_zip_sha256","status","privacy_names_included",
                    "member_count","total_uncompressed_bytes","duplicate_content_hashes",
                    "files","semantic_review_performed","archive_extracted"]
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
            if !serde_json::to_vec(&args).is_ok_and(|v| v.len() <= 2_100_000) {
                return Err(ToolError::Validation("ZIP request too large".into()));
            }
            let p: Input = serde_json::from_value(args)
                .map_err(|_| ToolError::Validation("invalid ZIP request".into()))?;
            if p.archive_hex.len() > 2_097_152 {
                return Err(ToolError::Validation("ZIP payload too large".into()));
            }
            let bytes = hex::decode(p.archive_hex)
                .map_err(|_| ToolError::Validation("invalid ZIP hex".into()))?;
            let out = audit_bytes(&bytes, false)
                .map_err(|e| ToolError::Validation(format!("ZIP rejected: {e:?}")))?;
            serde_json::to_value(out).map_err(|_| ToolError::Validation("encoding failure".into()))
        }
    }
}
#[cfg(feature = "ocaf-integration")]
pub use ocaf::{manifest, AgentTool};
