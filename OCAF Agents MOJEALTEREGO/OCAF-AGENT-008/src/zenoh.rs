//! Key-expression validation only; OMAs and OMAt are separate external servers.
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
    use crate::agent::{assess_promotion, PromotionProposal, PromotionTrust};
    use chrono::{DateTime, Utc};
    use ocaf_runtime::executor::{ExecutionContext, ToolError, ToolExecutor};
    use ocaf_runtime::registry::ToolManifest;
    use serde_json::{json, Value};
    pub struct AgentTool {
        trust: PromotionTrust,
        now: DateTime<Utc>,
    }
    impl AgentTool {
        pub fn new(trust: PromotionTrust, now: DateTime<Utc>) -> Self {
            Self { trust, now }
        }
    }
    pub fn manifest() -> ToolManifest {
        ToolManifest {
            name: "dgm.assess_promotion".into(),
            description: "Offline DGM promotion policy gate. External approval injected by trusted host".into(),
            input_schema: json!({
                "type": "object", "additionalProperties": false,
                "properties": {
                    "candidate_sha": {"type":"string","minLength":40,"maxLength":40},
                    "base_sha": {"type":"string","minLength":40,"maxLength":40},
                    "branch": {"type":"string","minLength":1,"maxLength":140},
                    "phase_receipts": {"type":"object","minProperties":4,"maxProperties":4},
                    "required_checks": {"type":"array","minItems":1,"maxItems":32,"items":{"type":"string"}},
                    "approval_expires_at": {"type":"string"},
                    "external_side_effects": {"type":"boolean"},
                    "rollback_ref_verified": {"type":"boolean"},
                    "source_provenance_verified": {"type":"boolean"}
                },
                "required":["candidate_sha","base_sha","branch","phase_receipts","required_checks",
                    "approval_expires_at","external_side_effects","rollback_ref_verified","source_provenance_verified"]
            }),
            output_schema: json!({
                "type":"object",
                "properties":{
                    "agent_id":{"const":"AM-OCAF-AI-0008"},
                    "admitted":{"type":"boolean"},
                    "reasons":{"type":"array","items":{"type":"string"}},
                    "execution_authorized":{"const":false},
                    "branch_mutated":{"const":false}
                },
                "required":["agent_id","admitted","reasons","execution_authorized","branch_mutated"]
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
            if !serde_json::to_vec(&args).is_ok_and(|v| v.len() <= 16_384) {
                return Err(ToolError::Validation("oversize promotion proposal".into()));
            }
            let proposal: PromotionProposal = serde_json::from_value(args)
                .map_err(|_| ToolError::Validation("invalid promotion proposal".into()))?;
            let decision = assess_promotion(&proposal, &self.trust, self.now);
            serde_json::to_value(decision)
                .map_err(|_| ToolError::Validation("encoding failed".into()))
        }
    }
}
#[cfg(feature = "ocaf-integration")]
pub use ocaf::{manifest, AgentTool};
