use axiom_forge_agent::lifecycle::{transition, LifecycleError, Stage};
use axiom_forge_agent::security::{load_config, SecurityError};
use axiom_forge_agent::zenoh::{route, ServerRole};

const CONFIG: &str = include_str!("../config/agent.toml");

#[test]
fn network_and_ostc_source_access_remain_disabled() {
    let config = load_config(CONFIG).unwrap();
    assert!(!config.communication.network_enabled);
    assert!(!config.source_access.enabled);
    assert_eq!(
        config.source_access(),
        Err(SecurityError::SourceEncryptionNotImplemented)
    );
    assert_eq!(
        load_config(&CONFIG.replace("network_enabled = false", "network_enabled = true")).err(),
        Some(SecurityError::InvalidConfiguration)
    );
    assert_eq!(
        load_config(&CONFIG.replace("enabled = false", "enabled = true")).err(),
        Some(SecurityError::InvalidConfiguration)
    );
}

#[test]
fn serial_and_name_tampering_are_rejected() {
    assert!(load_config(&CONFIG.replace("AM-ENG-AXF-2026-000004", "FORGED")).is_err());
    assert!(load_config(&CONFIG.replace("AXIOM FORGE", "FORGED")).is_err());
}

#[test]
fn malicious_tenant_key_is_rejected_and_services_are_separate() {
    assert!(route(ServerRole::OmasAgents, "tenant/../other", "agent_004").is_err());
    assert!(route(ServerRole::OmatTools, "tenant", "x..y").is_err());
    let agents = route(ServerRole::OmasAgents, "tenant", "agent_004").unwrap();
    let tools = route(ServerRole::OmatTools, "tenant", "graph.review").unwrap();
    assert!(agents.starts_with("omas/"));
    assert!(tools.starts_with("omat/"));
}

#[test]
fn self_asserted_evidence_cannot_promote_to_active() {
    assert_eq!(
        transition(Stage::Staged, Stage::Active),
        Err(LifecycleError::DeploymentNotAuthorized)
    );
}
