use nexus_consent_agent::agent::{review_link,LinkPlan,protocol_info};
fn owned()->LinkPlan {
    LinkPlan {
        protocol:"bluetooth_le".into(),purpose:"owned_device_transfer".into(),
        endpoint_owner_authorized:Some(true),counterparty_opted_in:Some(true),
        passive_tracking:Some(false),network_scan:Some(false),
        claims_global_device_location:Some(false),cloud_publication:Some(false),
    }
}
#[test]
fn owned_device_pairing_is_planning_only() {
    let d=review_link(&owned());
    assert!(d.accepted);
    assert!(!d.has_physical_location_evidence);
    assert!(!d.execution_authorized);
}
#[test]
fn tracking_and_missing_consent_are_denied() {
    let mut p=owned();p.counterparty_opted_in=Some(false);p.passive_tracking=Some(true);
    let d=review_link(&p);
    assert!(d.reasons.contains(&"counterparty_consent_missing".to_owned()));
    assert!(d.reasons.contains(&"passive_tracking_forbidden".to_owned()));
}
#[test]
fn zenoh_is_software_messaging_not_a_radio_location_proof() {
    let info=protocol_info("zenoh").unwrap();
    assert_eq!(info.transport,"software_messaging");
    assert_eq!(info.proximity,"network_dependent");
    assert_eq!(protocol_info("nfc").unwrap().proximity,"very_near");
}
#[test]
fn approval_does_not_allow_network_scanning_or_cloud_publication() {
    let mut p=owned();p.network_scan=Some(true);p.cloud_publication=Some(true);
    assert!(!review_link(&p).accepted);
}
