use vigilis_sre_agent::agent::{AGENT_ID, AGENT_NAME, REGISTRY_NUMBER, SERIAL_NUMBER};
use vigilis_sre_agent::security::load_config;

#[test]
fn unique_identity_and_serial_are_independent() {
    let config = load_config(include_str!("../config/agent.toml")).unwrap();
    assert_eq!(config.identity.registry_number, REGISTRY_NUMBER);
    assert_eq!(config.identity.unique_name, AGENT_NAME);
    assert_eq!(config.identity.serial_number, SERIAL_NUMBER);
    assert_eq!(config.identity.agent_id, AGENT_ID);
    assert_ne!(SERIAL_NUMBER, REGISTRY_NUMBER);
}
