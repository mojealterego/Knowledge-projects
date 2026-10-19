use axiom_forge_agent::agent::{review, PlanInput, TaskNode};
#[test]
fn valid_dependencies_are_sorted_but_never_authorized_for_execution() {
    let graph = PlanInput {
        graph_id: "G001".into(),
        tasks: vec![
            TaskNode { task_id: "test".into(), dependencies: vec!["build".into()], stage: "verify".into() },
            TaskNode { task_id: "build".into(), dependencies: vec![], stage: "build".into() },
        ],
    };
    let result = review(&graph).unwrap();
    assert_eq!(result.order, vec!["build", "test"]);
    assert_eq!(result.status, "structurally_valid");
    assert!(!result.execution_authorized);
    assert!(!result.verified);
}
#[test]
fn cycle_or_missing_dependency_must_block_graph() {
    let cycle = PlanInput {
        graph_id: "G002".into(),
        tasks: vec![
            TaskNode { task_id: "a".into(), dependencies: vec!["b".into()], stage: "plan".into() },
            TaskNode { task_id: "b".into(), dependencies: vec!["a".into()], stage: "build".into() },
        ],
    };
    let result = review(&cycle).unwrap();
    assert_eq!(result.status, "blocked");
    assert!(!result.issues.is_empty());
}
