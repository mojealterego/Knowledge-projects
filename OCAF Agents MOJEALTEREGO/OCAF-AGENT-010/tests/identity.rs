use veritas_trace_agent::agent::{
    AGENT_ID, AGENT_NAME, CATALOG_NUMBER, REGISTRY_NUMBER, SERIAL_NUMBER,
};
use veritas_trace_agent::security::load_config;
use std::collections::HashSet;

const CONFIG: &str = include_str!("../config/agent.toml");
const MANIFEST: &str = include_str!("../config/agent-manifest.json");
const REGISTRY: &str = include_str!("../../AGENT_IDENTITY_REGISTRY.json");

#[test]
fn unique_identity_and_serial_are_independent() {
    let config = load_config(CONFIG).unwrap();
    assert_eq!(config.identity.registry_number, REGISTRY_NUMBER);
    assert_eq!(config.identity.unique_name, AGENT_NAME);
    assert_eq!(config.identity.serial_number, SERIAL_NUMBER);
    assert_eq!(config.identity.agent_id, AGENT_ID);
    assert_eq!(config.identity.catalog_number, CATALOG_NUMBER);
    assert_ne!(SERIAL_NUMBER, REGISTRY_NUMBER);
}

#[test]
fn registry_uniqueness_and_manifest_are_consistent() {
    let registry: serde_json::Value = serde_json::from_str(REGISTRY).unwrap();
    let manifest: serde_json::Value = serde_json::from_str(MANIFEST).unwrap();
    let entries = registry["agents"].as_array().unwrap();
    let mut names = HashSet::new();
    let mut serials = HashSet::new();
    let mut numbers = HashSet::new();
    let mut agent_ids = HashSet::new();
    let mut catalogs = HashSet::new();
    for e in entries {
        assert!(names.insert(e["unique_name"].as_str().unwrap().to_uppercase()));
        assert!(serials.insert(e["serial_number"].as_str().unwrap().to_uppercase()));
        assert!(numbers.insert(e["registry_number"].as_str().unwrap().to_uppercase()));
        assert!(agent_ids.insert(e["agent_id"].as_str().unwrap().to_uppercase()));
        assert!(catalogs.insert(e["catalog_number"].as_str().unwrap().to_uppercase()));
    }
    let registered = entries
        .iter()
        .find(|entry| entry["registry_number"] == REGISTRY_NUMBER)
        .unwrap();
    assert_eq!(registered["agent_id"], AGENT_ID);
    assert_eq!(registered["unique_name"], AGENT_NAME);
    assert_eq!(registered["serial_number"], SERIAL_NUMBER);
    assert_eq!(manifest["agent_id"], AGENT_ID);
    assert_eq!(manifest["unique_name"], AGENT_NAME);
    assert_eq!(manifest["serial_number"], SERIAL_NUMBER);
    assert_eq!(manifest["certificate_issued"], false);
    assert_eq!(manifest["production_approved"], false);
}

#[test]
fn replaced_serial_or_name_are_rejected_by_config() {
    let altered_serial = CONFIG.replace(SERIAL_NUMBER, "FORGED");
    let altered_name = CONFIG.replace(AGENT_NAME, "FORGED");
    assert!(load_config(&altered_serial).is_err());
    assert!(load_config(&altered_name).is_err());
}
