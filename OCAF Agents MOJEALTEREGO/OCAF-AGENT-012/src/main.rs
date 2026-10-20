use chrono::{TimeZone,Utc};
use fides_grantis_agent::agent::{review,ProgramClaim};
use fides_grantis_agent::security::load_config;
fn main(){
    let conf=load_config(include_str!("../config/agent.toml")).expect("invalid config");
    let claim=ProgramClaim{
        repository:"mojealterego/Knowledge-projects".into(),action:"research".into(),
        credential_in_payload:Some(false),program:"alibaba_ai_catalyst".into(),
        credits_usd:Some(120000),tokens:Some(2000000000),
        status:None,company_eligibility_checked:None
    };
    let verdict=review(&claim,Utc.with_ymd_and_hms(2026,10,9,12,0,0).unwrap());
    println!("{} | {} | permitted={} | execution_authorized={}",conf.identity.unique_name,conf.identity.serial_number,verdict.permitted,verdict.execution_authorized);
}
