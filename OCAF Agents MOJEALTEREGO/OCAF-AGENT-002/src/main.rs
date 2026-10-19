//! Demonstration CLI: does not connect to a cluster.
use vigilis_sre_agent::agent::{triage, Incident};
use vigilis_sre_agent::security::load_config;

fn main() {
    let conf = load_config(include_str!("../config/agent.toml")).expect("invalid identity");
    let signal = std::env::args().nth(1).unwrap_or_else(|| "CrashLoopBackOff".into());
    let incident = Incident {
        incident_id: "DEMO-1".into(),
        tenant_id: "demo".into(),
        signal,
        observations: vec!["demo-only operator assertion, not observed from cluster".into()],
        risk_tier: "R0".into(),
    };
    let out = triage(&incident).expect("invalid incident");
    println!("{} | {} | {}", conf.identity.unique_name, conf.identity.serial_number, out.status);
    for step in out.recommendations {
        println!("- {step}");
    }
}
