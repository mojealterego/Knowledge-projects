use limes_git_agent::agent::{review_github_operation, GitOperation};
fn req(operation:&str) -> GitOperation {
    GitOperation { repository_full_name:"mojealterego/Knowledge-projects".into(),
        operation:operation.into(), target_branch:Some("policy/agent-test".into()),
        pull_base:None, pull_head:None }
}
#[test]
fn read_of_other_repository_allowed_but_write_denied() {
    let mut q=req("fetch_file");q.repository_full_name="mojealterego/ODYN-AI".into();
    assert!(review_github_operation(&q).permitted);
    q.operation="update_ref".into();
    let result=review_github_operation(&q);
    assert!(!result.permitted);
    assert!(result.reasons.contains(&"external_repository_write_forbidden".to_owned()));
    assert!(!result.execution_authorized);
}
#[test]
fn direct_main_writes_and_unscoped_writes_are_denied() {
    let mut q=req("update_file");q.target_branch=Some("main".into());
    assert!(review_github_operation(&q).reasons.contains(&"write_requires_working_branch".to_owned()));
    q.target_branch=None;
    assert!(!review_github_operation(&q).permitted);
}
#[test]
fn review_gates_pr_head_and_base_without_merging() {
    let mut q=req("merge_pull_request");q.target_branch=None;
    q.pull_base=Some("main".into());q.pull_head=Some("policy/agent-test".into());
    let result=review_github_operation(&q);
    assert!(result.permitted);
    assert!(!result.execution_authorized);
    q.pull_head=Some("policy/../main".into());
    assert!(review_github_operation(&q).reasons.contains(&"unexpected_pr_head".to_owned()));
}
#[test]
fn unknown_mutation_not_explicitly_allowed() {
    assert_eq!(review_github_operation(&req("change_default_branch")).reasons,
      vec!["operation_not_explicitly_allowed".to_owned()]);
}
