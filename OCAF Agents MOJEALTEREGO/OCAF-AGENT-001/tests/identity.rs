//! Registry identity is distinct from agent display name, serial and certificate.

use std::collections::HashSet;
use wda_photo_agent::agent::{
    AGENT_ID, AGENT_NAME, CATALOG_NUMBER, REGISTRY_NUMBER, SERIAL_NUMBER,
};
use wda_photo_agent::security::load_config;

const CONFIG: &str = include_str!("../config/agent.toml");
const MANIFEST: &str = include_str!("../config/agent-manifest.json");
const REGISTRY: &str = include_str!("../../AGENT_IDENTITY_REGISTRY.json");

#[test]
fn immutable_name_and_serial_match_runtime_configuration() {
    let c = load_config(CONFIG).unwrap();
    assert_eq!(AGENT_NAME, "WDA AURELION");
    assert_eq!(REGISTRY_NUMBER, "001");
    assert_eq!(AGENT_ID, "AM-OCAF-AI-0001");
    assert_eq!(SERIAL_NUMBER, "AM-WDA-AUR-2026-000001-DE338AC20B42");
    assert_eq!(CATALOG_NUMBER, "AM-OCAF-WDA-001");
    assert_eq!(c.identity.unique_name, AGENT_NAME);
    assert_eq!(c.identity.serial_number, SERIAL_NUMBER);
    assert_eq!(c.identity.catalog_number, CATALOG_NUMBER);
    assert_eq!(c.identity.registry_number, REGISTRY_NUMBER);
}

#[test]
fn serial_name_and_registry_number_are_unique_in_registry() {
    let value: serde_json::Value = serde_json::from_str(REGISTRY).unwrap();
    let agents = value["agents"].as_array().unwrap();
    let mut registry = HashSet::new();
    let mut names = HashSet::new();
    let mut serials = HashSet::new();
    let mut agent_ids = HashSet::new();
    for agent in agents {
        let number = agent["registry_number"].as_str().unwrap();
        let name = agent["unique_name"].as_str().unwrap();
        let serial = agent["serial_number"].as_str().unwrap();
        let id = agent["agent_id"].as_str().unwrap();
        assert!(
            registry.insert(number.to_uppercase()),
            "duplicate registry number"
        );
        assert!(names.insert(name.to_uppercase()), "duplicate unique name");
        assert!(
            serials.insert(serial.to_uppercase()),
            "duplicate serial number"
        );
        assert!(agent_ids.insert(id.to_uppercase()), "duplicate agent ID");
    }
    let assigned = agents
        .iter()
        .find(|a| a["registry_number"] == REGISTRY_NUMBER)
        .expect("Agent 001 must be present");
    assert_eq!(assigned["unique_name"], AGENT_NAME);
    assert_eq!(assigned["serial_number"], SERIAL_NUMBER);
    assert_eq!(assigned["agent_id"], AGENT_ID);
    assert_eq!(assigned["catalog_number"], CATALOG_NUMBER);
    assert_eq!(assigned["certificate_issued"], false);
    assert_eq!(assigned["production_approved"], false);
}

#[test]
fn manifest_identifiers_and_states_match_canonical_registry() {
    let doc: serde_json::Value = serde_json::from_str(MANIFEST).unwrap();
    assert_eq!(doc["unique_name"], AGENT_NAME);
    assert_eq!(doc["display_name"], AGENT_NAME);
    assert_eq!(doc["serial_number"], SERIAL_NUMBER);
    assert_eq!(doc["agent_id"], AGENT_ID);
    assert_eq!(doc["registry_number"], REGISTRY_NUMBER);
    assert_eq!(doc["catalog_number"], CATALOG_NUMBER);
    assert_eq!(doc["source_access"]["enabled"], false);
    assert_eq!(doc["certificate_issued"], false);
    assert_eq!(doc["production_approved"], false);
}
