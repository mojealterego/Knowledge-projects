//! Pure-data GitHub operation preflight port of tools/single_repository_scope_gate.py.
//! No API calls; a "permitted" result never authorizes an actual GitHub mutation.
use serde::{Deserialize, Serialize};

pub const AGENT_NAME: &str = "LIMES GIT";
pub const REGISTRY_NUMBER: &str = "011";
pub const AGENT_ID: &str = "AM-OCAF-AI-0011";
pub const SERIAL_NUMBER: &str = "AM-GIT-LIM-2026-000011";
pub const CATALOG_NUMBER: &str = "AM-OCAF-GIT-011";
pub const TARGET_REPO: &str = "mojealterego/Knowledge-projects";

const READ_OPERATIONS: [&str; 8] = ["get_repo","fetch","fetch_file","get_file",
    "list_repos","search_repositories","compare_commits","get_commit_status"];
const WRITE_OPERATIONS: [&str; 10] = ["create_branch","create_blob","create_tree",
    "create_commit","update_ref","create_file","update_file","delete_file",
    "create_pull_request","merge_pull_request"];

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GitOperation {
    pub repository_full_name: String,
    pub operation: String,
    pub target_branch: Option<String>,
    pub pull_base: Option<String>,
    pub pull_head: Option<String>,
}
#[derive(Debug, Clone, Serialize)]
pub struct ScopeVerdict {
    pub agent_id: &'static str,
    pub permitted: bool,
    pub reasons: Vec<String>,
    pub execution_authorized: bool,
    pub api_called: bool,
}
fn working_branch(name: Option<&str>) -> bool {
    name.is_some_and(|n| {
        n.len() <= 160
        && ["knowledge/","policy/","feature/","fix/"].iter().any(|prefix| n.starts_with(prefix))
        && !n.contains("..") && !n.contains('\\') && !n.ends_with('/')
        && !n.chars().any(char::is_whitespace)
    })
}
pub fn review_github_operation(req: &GitOperation) -> ScopeVerdict {
    let mut reasons=Vec::new();
    if READ_OPERATIONS.contains(&req.operation.as_str()) {
        return ScopeVerdict {
            agent_id:AGENT_ID,permitted:true,reasons,execution_authorized:false,api_called:false,
        };
    }
    if !WRITE_OPERATIONS.contains(&req.operation.as_str()) {
        reasons.push("operation_not_explicitly_allowed".to_owned());
    } else {
        if req.repository_full_name != TARGET_REPO {
            reasons.push("external_repository_write_forbidden".to_owned());
        }
        if matches!(req.operation.as_str(),"create_pull_request"|"merge_pull_request") {
            if req.pull_base.as_deref() != Some("main") {
                reasons.push("unexpected_pr_base".to_owned());
            }
            if !working_branch(req.pull_head.as_deref()) {
                reasons.push("unexpected_pr_head".to_owned());
            }
        } else if !working_branch(req.target_branch.as_deref()) {
            reasons.push("write_requires_working_branch".to_owned());
        }
    }
    ScopeVerdict {
        agent_id:AGENT_ID,permitted:reasons.is_empty(),reasons,
        execution_authorized:false,api_called:false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn req(op: &str) -> GitOperation {
        GitOperation {
            repository_full_name:TARGET_REPO.into(),operation:op.into(),
            target_branch:Some("policy/demo".into()),pull_base:None,pull_head:None
        }
    }
    #[test]
    fn legitimate_branch_write_passes_preflight_only() {
        let d=review_github_operation(&req("create_commit"));
        assert!(d.permitted);
        assert!(!d.execution_authorized);
        assert!(!d.api_called);
    }
    #[test]
    fn malformed_branch_and_unknown_action_denied() {
        let mut q=req("create_commit");
        q.target_branch=Some("policy/../main".into());
        assert!(!review_github_operation(&q).permitted);
        let q=req("change_default_branch");
        assert!(!review_github_operation(&q).permitted);
    }
}
