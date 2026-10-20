use limes_git_agent::agent::{review_github_operation,GitOperation};
use limes_git_agent::security::load_config;
fn main() {
    let conf=load_config(include_str!("../config/agent.toml")).expect("invalid identity");
    let result=review_github_operation(&GitOperation{
        repository_full_name:"mojealterego/Knowledge-projects".into(),
        operation:"update_ref".into(),target_branch:Some("main".into()),
        pull_base:None,pull_head:None
    });
    println!("{} | {} | permitted={} | execution_authorized={}",
        conf.identity.unique_name,conf.identity.serial_number,
        result.permitted,result.execution_authorized);
}
