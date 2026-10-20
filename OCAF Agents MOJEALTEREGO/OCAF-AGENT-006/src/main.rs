use custos_admission_agent::agent::{admit,ToolManifest,SANDBOX_PROFILE};
use custos_admission_agent::security::load_config;
use chrono::{TimeZone,Utc};
use sha2::{Digest,Sha256};
use std::collections::{BTreeMap,BTreeSet};

fn main() {
    let c=load_config(include_str!("../config/agent.toml")).expect("bad identity");
    let bytes=b"demo-only artifact";
    let digest=format!("{:x}",Sha256::digest(bytes));
    let m=ToolManifest{artifact_sha256:digest.clone(),capabilities:vec!["parse_document".into()],sandbox_profile:SANDBOX_PROFILE.into(),timeout_seconds:30,expires_at:"2026-10-22T00:00:00Z".into(),external_side_effects:false};
    let approvals=BTreeMap::from([(digest,BTreeSet::from(["parse_document".to_owned()]))]);
    let out=admit(&m,bytes,&approvals,Utc.with_ymd_and_hms(2026,10,20,10,0,0).unwrap());
    println!("{} | {} | {} | execution_authorized={}",c.identity.unique_name,c.identity.serial_number,out.status,out.execution_authorized);
}
