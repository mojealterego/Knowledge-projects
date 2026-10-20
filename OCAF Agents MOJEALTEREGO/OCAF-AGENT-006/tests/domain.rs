use chrono::{TimeZone, Utc};
use custos_admission_agent::agent::{admit, ToolManifest};
use std::collections::{BTreeMap, BTreeSet};

fn example() -> (ToolManifest, Vec<u8>, BTreeMap<String, BTreeSet<String>>) {
    let data = b"independently reviewed offline fixture".to_vec();
    let digest = "a".repeat(64); // Must not pass unless the real digest is used.
    let manifest = ToolManifest {
        artifact_sha256: digest,
        capabilities: vec!["parse_document".into()],
        sandbox_profile: "isolated_no_network_read_only".into(),
        timeout_seconds: 30,
        expires_at: "2026-10-22T00:00:00Z".into(),
        external_side_effects: false,
    };
    (manifest, data, BTreeMap::new())
}

#[test]
fn self_asserted_approval_is_insufficient() {
    let (m, data, _) = example();
    let mut approvals = BTreeMap::new();
    approvals.insert(
        m.artifact_sha256.clone(),
        BTreeSet::from(["parse_document".into()]),
    );
    let now = Utc.with_ymd_and_hms(2026, 10, 20, 10, 0, 0).unwrap();
    let decision = admit(&m, &data, &approvals, now);
    assert!(!decision.accepted);
    assert!(decision
        .reasons
        .contains(&"artifact_hash_mismatch".to_string()));
}

#[test]
fn genuine_matching_digest_and_external_scope_allow_nonexecuting_admission() {
    let (mut m, data, _) = example();
    m.artifact_sha256 = sha2_digest(&data);
    let mut approvals = BTreeMap::new();
    approvals.insert(
        m.artifact_sha256.clone(),
        BTreeSet::from(["parse_document".into()]),
    );
    let now = Utc.with_ymd_and_hms(2026, 10, 20, 10, 0, 0).unwrap();
    let decision = admit(&m, &data, &approvals, now);
    assert!(decision.accepted);
    assert!(decision.reasons.is_empty());
}

#[test]
fn privileged_capability_denied_even_with_external_scope() {
    let (mut m, data, _) = example();
    m.artifact_sha256 = sha2_digest(&data);
    m.capabilities = vec!["network".into()];
    let mut approvals = BTreeMap::new();
    approvals.insert(
        m.artifact_sha256.clone(),
        BTreeSet::from(["network".into()]),
    );
    let now = Utc.with_ymd_and_hms(2026, 10, 20, 10, 0, 0).unwrap();
    assert!(!admit(&m, &data, &approvals, now).accepted);
}
fn sha2_digest(data: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    format!("{:x}", Sha256::digest(data))
}
