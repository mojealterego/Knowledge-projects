//! OMAs (agent server) and OMAt (tool server) use separate logical keyspaces.
//! Transport remains disabled. Stateful guard is installed only in local OCAF.
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
    use crate::agent::{Guard, GuardError, Limits, Observation};
    use ocaf_runtime::executor::{ExecutionContext, ToolError, ToolExecutor};
    use ocaf_runtime::registry::ToolManifest;
    use serde_json::{json, Value};
    use std::sync::Mutex;

    /// The host owns and injects progress, models cannot increase it in tool JSON.
    pub struct AgentTool {
        guard: Mutex<Guard>,
    }
    impl AgentTool {
        pub fn new(limits: Limits) -> Result<Self, GuardError> {
            Ok(Self {
                guard: Mutex::new(Guard::new(limits)?),
            })
        }

        /// Only call after an independent, trusted executor postcondition readback.
        pub fn advance_verified_progress(&self, version: u64) -> Result<(), GuardError> {
            let mut state = self
                .guard
                .lock()
                .map_err(|_| GuardError::InvalidProgressVersion)?;
            state.advance_verified_progress(version)
        }
    }

    pub fn manifest() -> ToolManifest {
        ToolManifest {
            name: "tool.loop_gate".into(),
            description: "Local pre-execution loop and token budget gate, no side effects".into(),
            input_schema: json!({
                "type":"object",
                "additionalProperties":false,
                "properties":{
                    "tool":{"type":"string","minLength":1,"maxLength":128},
                    "args_sha256":{"type":"string","minLength":64,"maxLength":64},
                    "estimated_tokens":{"type":"integer","minimum":0}
                },
                "required":["tool","args_sha256","estimated_tokens"]
            }),
            output_schema: json!({
                "type":"object",
                "properties":{
                    "agent_id":{"const":"AM-OCAF-AI-0007"},
                    "allowed":{"type":"boolean"},
                    "reason":{"type":"string"},
                    "total_calls":{"type":"integer"},
                    "total_tokens":{"type":"integer"},
                    "identical_no_progress":{"type":"integer"},
                    "execution_authorized":{"const":false}
                },
                "required":["agent_id","allowed","reason","total_calls","total_tokens","identical_no_progress","execution_authorized"]
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
            if !serde_json::to_vec(&args).is_ok_and(|v| v.len() <= 4096) {
                return Err(ToolError::Validation("input size too large".into()));
            }
            let request: Observation = serde_json::from_value(args)
                .map_err(|_| ToolError::Validation("invalid loop observation".into()))?;
            let mut guard = self
                .guard
                .lock()
                .map_err(|_| ToolError::Validation("guard lock is poisoned".into()))?;
            serde_json::to_value(guard.observe(request))
                .map_err(|_| ToolError::Validation("encoding failed".into()))
        }
    }
}
#[cfg(feature = "ocaf-integration")]
pub use ocaf::{manifest, AgentTool};
