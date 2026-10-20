use provenance_evidence_agent::agent::{make_record,EvidenceInput,Origin};
use provenance_evidence_agent::security::load_config;
use std::collections::BTreeSet;
fn main() {
    let c=load_config(include_str!("../config/agent.toml")).expect("invalid agent identity");
    let i=EvidenceInput {
        claim:"demonstration, not independently observed".into(),origin:Origin::Simulated,
        scope_id:None,source_ref:None,observed_at:None,verifier_ref:None,source_sha256:None,
    };
    let r=make_record(&i,&BTreeSet::new()).unwrap();
    println!("{} | {} | {} | verified={}",
        c.identity.unique_name,c.identity.serial_number,r.ui_badge,r.independently_verified);
}
