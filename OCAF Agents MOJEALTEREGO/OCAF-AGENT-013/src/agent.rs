//! Pure-data translation of communication_link_policy.py. No real radio, GPS or Zenoh session.
use serde::{Deserialize, Serialize};
pub const AGENT_NAME: &str = "NEXUS CONSENT";
pub const REGISTRY_NUMBER: &str = "013";
pub const AGENT_ID: &str = "AM-OCAF-AI-0013";
pub const SERIAL_NUMBER: &str = "AM-COM-NEX-2026-000013";
pub const CATALOG_NUMBER: &str = "AM-OCAF-COM-013";

#[derive(Debug, Clone, Copy)]
pub struct ProtocolInfo {
    pub transport: &'static str,
    pub proximity: &'static str,
    pub pairing: &'static str,
}
pub fn protocol_info(name: &str) -> Option<ProtocolInfo> {
    let (transport, proximity, pairing) = match name {
        "irda" => ("optical", "line_of_sight", "explicit"),
        "bluetooth_le" => ("radio", "nearby", "explicit"),
        "nfc" => ("radio", "very_near", "explicit"),
        "uwb" => ("radio", "nearby", "explicit"),
        "wifi_direct" => ("radio", "local", "explicit"),
        "lorawan" => ("radio", "long_range", "provisioned"),
        "li_fi" => ("optical", "line_of_sight", "explicit"),
        "zenoh" => ("software_messaging", "network_dependent", "authenticated"),
        _ => return None,
    };
    Some(ProtocolInfo {
        transport,
        proximity,
        pairing,
    })
}
#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LinkPlan {
    pub protocol: String,
    pub purpose: String,
    pub endpoint_owner_authorized: Option<bool>,
    pub counterparty_opted_in: Option<bool>,
    pub passive_tracking: Option<bool>,
    pub network_scan: Option<bool>,
    pub claims_global_device_location: Option<bool>,
    pub cloud_publication: Option<bool>,
}
#[derive(Debug, Serialize)]
pub struct LinkVerdict {
    pub agent_id: &'static str,
    pub accepted: bool,
    pub reasons: Vec<String>,
    pub has_physical_location_evidence: bool,
    pub execution_authorized: bool,
    pub device_contacted: bool,
}
pub fn review_link(plan: &LinkPlan) -> LinkVerdict {
    let mut reasons = Vec::new();
    if protocol_info(&plan.protocol).is_none() {
        reasons.push("unknown_protocol".into());
    }
    if !matches!(
        plan.purpose.as_str(),
        "owned_device_transfer" | "authorized_testbed" | "offline_comparison"
    ) {
        reasons.push("unapproved_purpose".into());
    }
    if plan.endpoint_owner_authorized != Some(true) {
        reasons.push("endpoint_owner_not_authorized".into());
    }
    if plan.counterparty_opted_in != Some(true) {
        reasons.push("counterparty_consent_missing".into());
    }
    if plan.passive_tracking != Some(false) {
        reasons.push("passive_tracking_forbidden".into());
    }
    if plan.network_scan != Some(false) {
        reasons.push("network_scanning_forbidden".into());
    }
    if plan.claims_global_device_location != Some(false) {
        reasons.push("protocol_does_not_prove_global_location".into());
    }
    if plan.cloud_publication != Some(false) {
        reasons.push("external_data_transfer_requires_separate_approval".into());
    }
    LinkVerdict {
        agent_id: AGENT_ID,
        accepted: reasons.is_empty(),
        reasons,
        has_physical_location_evidence: false,
        execution_authorized: false,
        device_contacted: false,
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unsupported_protocol_denied() {
        assert!(protocol_info("unknown").is_none());
        assert_eq!(protocol_info("nfc").unwrap().proximity, "very_near");
    }
    #[test]
    fn absent_consent_denied() {
        let input = LinkPlan {
            protocol: "zenoh".into(),
            purpose: "offline_comparison".into(),
            endpoint_owner_authorized: None,
            counterparty_opted_in: None,
            passive_tracking: None,
            network_scan: None,
            claims_global_device_location: None,
            cloud_publication: None,
        };
        assert!(!review_link(&input).accepted);
    }
}
