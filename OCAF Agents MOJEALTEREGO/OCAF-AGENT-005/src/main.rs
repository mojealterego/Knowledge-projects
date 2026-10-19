//! Demonstration only, no mobile UI or device connections.
use argus_mobile_agent::agent::{assess, ActionProposal};
use argus_mobile_agent::security::load_config;

fn main() {
    let config = load_config(include_str!("../config/agent.toml")).expect("invalid identity");
    let proposal = ActionProposal {
        session_id: "demo".into(),
        snapshot_id: "snapshot".into(),
        node_id: "node".into(),
        action: "CLICK".into(),
        operator_grant: true,
        expected_postcondition: "new screen visible".into(),
    };
    let output = assess(&proposal).expect("invalid action proposal");
    println!(
        "{} | {} | {}",
        config.identity.unique_name, config.identity.serial_number, output.status
    );
    for reason in output.reasons {
        println!("- {reason}");
    }
}
