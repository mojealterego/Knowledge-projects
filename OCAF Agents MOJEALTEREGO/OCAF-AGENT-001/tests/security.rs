use wda_photo_agent::agent::{plan, PlanError};
use wda_photo_agent::lifecycle::{AgentState, Lifecycle, LifecycleError, ReleaseEvidence};
use wda_photo_agent::security::{load_config, SecurityError};
use wda_photo_agent::zenoh::{route, ServerRole};

const CONFIG: &str = include_str!("../config/agent.toml");

#[test]
fn shipped_configuration_is_locked_down_and_defers_ostc() {
    let c = load_config(CONFIG).unwrap();
    assert_eq!(c.servers.agent_server, "OMAs");
    assert_eq!(c.servers.tool_server, "OMAt");
    assert!(!c.communication.network_enabled);
    assert!(!c.source_access.enabled);
    assert_eq!(
        c.require_source_access(),
        Err(SecurityError::SourceEncryptionNotImplemented)
    );
}

#[test]
fn configuration_rejects_source_enablement_without_ostc_spec() {
    let altered = CONFIG.replace("enabled = false", "enabled = true");
    assert_eq!(
        load_config(&altered).err(),
        Some(SecurityError::InvalidConfiguration)
    );
}

#[test]
fn configuration_rejects_network_enablement() {
    let altered = CONFIG.replace("network_enabled = false", "network_enabled = true");
    assert_eq!(
        load_config(&altered).err(),
        Some(SecurityError::InvalidConfiguration)
    );
}

#[test]
fn rejects_namespace_injection() {
    for invalid in ["..", "tenant_a/../tenant_b", "", "foo*", "a.b"] {
        assert!(route(ServerRole::OmasAgents, invalid, "agent001").is_err());
    }
    assert!(route(ServerRole::OmatTools, "tenant001", "photo..plan").is_err());
}

#[test]
fn identity_unlock_is_denied() {
    assert_eq!(
        plan("enhance lighting", false),
        Err(PlanError::IdentityUnlockDenied)
    );
}

#[test]
fn activation_fails_without_all_external_and_ostc_gates() {
    let mut lc = Lifecycle::new();
    lc.transition(AgentState::Validated, None).unwrap();
    lc.transition(AgentState::Tested, None).unwrap();
    lc.transition(AgentState::Staged, None).unwrap();
    let false_evidence = ReleaseEvidence {
        tests_passed: true,
        omas_authenticated: true,
        omat_authorized: true,
        ostc_source_access_verified: false,
        operator_approved: true,
    };
    assert_eq!(
        lc.transition(AgentState::Active, Some(false_evidence)),
        Err(LifecycleError::ReleaseGateNotSatisfied)
    );
    assert_eq!(lc.state(), AgentState::Staged);
    assert_eq!(
        lc.transition(AgentState::Validated, None),
        Err(LifecycleError::InvalidTransition)
    );
}

#[test]
fn even_forged_all_true_evidence_cannot_activate_unimplemented_services() {
    let mut lc = Lifecycle::new();
    lc.transition(AgentState::Validated, None).unwrap();
    lc.transition(AgentState::Tested, None).unwrap();
    lc.transition(AgentState::Staged, None).unwrap();
    let forged = ReleaseEvidence {
        tests_passed: true,
        omas_authenticated: true,
        omat_authorized: true,
        ostc_source_access_verified: true,
        operator_approved: true,
    };
    assert_eq!(
        lc.transition(AgentState::Active, Some(forged)),
        Err(LifecycleError::ReleaseGateNotSatisfied)
    );
    assert_eq!(lc.state(), AgentState::Staged);
}
