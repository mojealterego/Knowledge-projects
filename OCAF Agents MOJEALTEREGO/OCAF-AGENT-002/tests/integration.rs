use vigilis_sre_agent::agent::{triage, Incident};
#[test]
fn incident_triage_must_not_execute_changes() {
    let i = Incident {
        incident_id: "INC-1".into(),
        tenant_id: "team_a".into(),
        signal: "CrashLoopBackOff".into(),
        observations: vec!["operator reports repeated restarts".into()],
        risk_tier: "R3".into(),
    };
    let result = triage(&i).unwrap();
    assert_eq!(result.status, "read_only");
    assert!(!result.allow_mutation);
}
