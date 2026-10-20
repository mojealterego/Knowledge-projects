use chrono::{TimeZone, Utc};
use dgm_sentinel_agent::agent::{assess_promotion, PromotionProposal, PromotionTrust, EXPECTED_PHASES};
use dgm_sentinel_agent::security::load_config;
use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let cfg = load_config(include_str!("../config/agent.toml")).expect("invalid identity");
    let candidate = "a".repeat(40);
    let receipts: BTreeMap<String, String> = EXPECTED_PHASES.iter()
        .map(|s| ((*s).to_owned(), format!("trusted:{s}"))).collect();
    let proposal = PromotionProposal {
        candidate_sha: candidate.clone(), base_sha: "b".repeat(40),
        branch: "integration/demo".into(), phase_receipts: receipts.clone(),
        required_checks: vec!["tests".into()],
        approval_expires_at: "2026-10-22T00:00:00Z".into(),
        external_side_effects: false,
        rollback_ref_verified: true, source_provenance_verified: true,
    };
    let trust = PromotionTrust {
        verified_ci_by_candidate:BTreeMap::from([(candidate.clone(),BTreeSet::from(["tests".to_owned()]))]),
        approved_candidates:BTreeSet::from([candidate.clone()]),
        approved_phase_receipts:BTreeMap::from([(candidate.clone(),receipts.values().cloned().collect())]),
        verified_rollback_candidates:BTreeSet::from([candidate.clone()]),
        verified_source_candidates:BTreeSet::from([candidate]),
    };
    let result = assess_promotion(&proposal, &trust,
        Utc.with_ymd_and_hms(2026,10,20,12,0,0).unwrap());
    println!("{} | {} | admitted={} | execution_authorized={}",
        cfg.identity.unique_name, cfg.identity.serial_number,
        result.admitted, result.execution_authorized);
}
