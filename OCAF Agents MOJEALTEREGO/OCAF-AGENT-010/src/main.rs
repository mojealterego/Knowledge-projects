use veritas_trace_agent::agent::{record_evidence, EvidenceInput};
use veritas_trace_agent::security::load_config;
fn main() {
    let conf = load_config(include_str!("../config/agent.toml")).expect("invalid identity");
    let item = record_evidence(EvidenceInput {
        claim: "Synthetic relationship graph".into(),
        origin: "simulated".into(),
        scope_id: None,
        source_ref: None,
        observed_at: None,
        verifier_ref: None,
        source_sha256: None,
    })
    .expect("bad record");
    println!(
        "{} | {} | {} | independently_verified={}",
        conf.identity.unique_name,
        conf.identity.serial_number,
        item.origin,
        item.independently_verified
    );
}
