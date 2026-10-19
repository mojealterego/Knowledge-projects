use argus_mobile_agent::agent::{assess, ActionProposal};
#[test]
fn destructive_action_is_blocked_even_with_claimed_operator_grant() {
    let proposal = ActionProposal {
        session_id: "sessionA".into(),
        snapshot_id: "snapshot1".into(),
        node_id: "node2".into(),
        action: "DELETE".into(),
        operator_grant: true,
        expected_postcondition: "Object removed".into(),
    };
    let result = assess(&proposal).unwrap();
    assert_eq!(result.status, "blocked");
    assert!(!result.execution_authorized);
    assert!(!result.verified);
}
#[test]
fn bounded_click_proposal_still_never_executes() {
    let proposal = ActionProposal {
        session_id: "sessionA".into(),
        snapshot_id: "snapshot1".into(),
        node_id: "node2".into(),
        action: "CLICK".into(),
        operator_grant: true,
        expected_postcondition: "Page changed".into(),
    };
    let result = assess(&proposal).unwrap();
    assert_eq!(result.status, "operator_review");
    assert!(!result.execution_authorized);
    assert!(result.readback_required);
}
