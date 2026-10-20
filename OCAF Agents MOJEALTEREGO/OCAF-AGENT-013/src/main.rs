use nexus_consent_agent::agent::{review_link,LinkPlan};
use nexus_consent_agent::security::load_config;
fn main(){
    let conf=load_config(include_str!("../config/agent.toml")).expect("invalid config");
    let proposal=LinkPlan{
        protocol:"zenoh".into(),purpose:"offline_comparison".into(),
        endpoint_owner_authorized:Some(true),counterparty_opted_in:Some(true),
        passive_tracking:Some(false),network_scan:Some(false),
        claims_global_device_location:Some(false),cloud_publication:Some(false),
    };
    let verdict=review_link(&proposal);
    println!("{} | {} | accepted={} | execution_authorized={}",conf.identity.unique_name,conf.identity.serial_number,verdict.accepted,verdict.execution_authorized);
}
