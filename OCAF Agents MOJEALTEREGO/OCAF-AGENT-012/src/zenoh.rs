//! Local OMAs/OMAt route key validation. No Zenoh sockets or remote RPC.
use crate::security::{safe_segment,safe_tool_name,SecurityError};
#[derive(Debug,Copy,Clone)]
pub enum ServerRole{OmasAgents,OmatTools}
pub fn route(role:ServerRole,tenant:&str,name:&str)->Result<String,SecurityError>{
    safe_segment(tenant)?;
    match role{
        ServerRole::OmasAgents=>{safe_segment(name)?;Ok(format!("omas/v1/{tenant}/agents/{name}"))}
        ServerRole::OmatTools=>{safe_tool_name(name)?;Ok(format!("omat/v1/{tenant}/tools/{name}"))}
    }
}

#[cfg(feature="ocaf-integration")]
mod ocaf{
    use crate::agent::{review,ProgramClaim};
    use chrono::{DateTime,Utc};
    use ocaf_runtime::executor::{ExecutionContext,ToolError,ToolExecutor};
    use ocaf_runtime::registry::ToolManifest;
    use serde_json::{json,Value};
    /// Trusted caller owns the review date; model JSON cannot specify time.
    pub struct AgentTool{ now:DateTime<Utc> }
    impl AgentTool{pub fn new(now:DateTime<Utc>)->Self{Self{now}}}
    pub fn manifest()->ToolManifest{
        ToolManifest{
            name:"program.claim_review".into(),
            description:"Offline public program documentation review, no award assertion or submission".into(),
            input_schema:json!({
                "type":"object","additionalProperties":false,
                "properties":{
                    "repository":{"type":"string"},"action":{"type":"string"},
                    "credential_in_payload":{"type":["boolean","null"]},
                    "program":{"type":"string"},"credits_usd":{"type":["integer","null"]},
                    "tokens":{"type":["integer","null"]},"status":{"type":["string","null"]},
                    "company_eligibility_checked":{"type":["boolean","null"]}
                },
                "required":["repository","action","program"]
            }),
            output_schema:json!({
                "type":"object","properties":{
                    "agent_id":{"const":"AM-OCAF-AI-0012"},"permitted":{"type":"boolean"},
                    "reasons":{"type":"array","items":{"type":"string"}},
                    "funding_awarded_verified":{"const":false},
                    "external_submission_executed":{"const":false},
                    "execution_authorized":{"const":false}
                },
                "required":["agent_id","permitted","reasons","funding_awarded_verified","external_submission_executed","execution_authorized"]
            }),
            cost_sats:0,permissions:vec![],timeout_ms:5000,enabled:true,
        }
    }
    #[async_trait::async_trait]
    impl ToolExecutor for AgentTool{
        async fn execute(&self,args:Value,_ctx:&ExecutionContext)->Result<Value,ToolError>{
            if !serde_json::to_vec(&args).is_ok_and(|v|v.len()<=4096){
                return Err(ToolError::Validation("oversize program record".into()));
            }
            let input:ProgramClaim=serde_json::from_value(args)
                .map_err(|_|ToolError::Validation("invalid record".into()))?;
            serde_json::to_value(review(&input,self.now))
                .map_err(|_|ToolError::Validation("failed to encode verdict".into()))
        }
    }
}
#[cfg(feature="ocaf-integration")]
pub use ocaf::{manifest,AgentTool};
