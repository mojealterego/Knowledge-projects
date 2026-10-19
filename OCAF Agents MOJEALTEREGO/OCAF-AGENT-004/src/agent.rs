//! A bounded offline structural task graph reviewer, not an executor or policy authority.
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, HashSet};

pub const AGENT_NAME: &str = "AXIOM FORGE";
pub const REGISTRY_NUMBER: &str = "004";
pub const AGENT_ID: &str = "AM-OCAF-AI-0004";
pub const SERIAL_NUMBER: &str = "AM-ENG-AXF-2026-000004";
pub const CATALOG_NUMBER: &str = "AM-OCAF-ENG-004";

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskNode {
    pub task_id: String,
    pub dependencies: Vec<String>,
    pub stage: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlanInput {
    pub graph_id: String,
    pub tasks: Vec<TaskNode>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ReviewError {
    InvalidInput,
    DuplicateTask,
    DuplicateDependency,
}

#[derive(Debug, Serialize)]
pub struct GraphReview {
    pub agent_id: &'static str,
    pub status: &'static str,
    pub task_count: usize,
    pub order: Vec<String>,
    pub issues: Vec<String>,
    pub execution_authorized: bool,
    pub verified: bool,
}

fn valid_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 64
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
}

pub fn review(input: &PlanInput) -> Result<GraphReview, ReviewError> {
    if !valid_id(&input.graph_id) || input.tasks.is_empty() || input.tasks.len() > 32 {
        return Err(ReviewError::InvalidInput);
    }
    let mut tasks: BTreeMap<&str, &TaskNode> = BTreeMap::new();
    for task in &input.tasks {
        if !valid_id(&task.task_id)
            || task.dependencies.len() > 16
            || !matches!(task.stage.as_str(), "plan" | "build" | "verify" | "release")
        {
            return Err(ReviewError::InvalidInput);
        }
        if tasks.insert(task.task_id.as_str(), task).is_some() {
            return Err(ReviewError::DuplicateTask);
        }
    }

    let mut issues = Vec::new();
    for task in tasks.values() {
        let mut seen = HashSet::new();
        for dep in &task.dependencies {
            if !valid_id(dep) {
                return Err(ReviewError::InvalidInput);
            }
            if !seen.insert(dep) {
                return Err(ReviewError::DuplicateDependency);
            }
            if dep == &task.task_id {
                issues.push(format!("self_dependency:{}", task.task_id));
            } else if !tasks.contains_key(dep.as_str()) {
                issues.push(format!("missing_dependency:{}:{}", task.task_id, dep));
            }
        }
    }

    let mut done: BTreeSet<&str> = BTreeSet::new();
    let mut order: Vec<String> = Vec::new();
    while done.len() < tasks.len() {
        let next = tasks.iter().find(|(id, node)| {
            !done.contains(*id)
                && node
                    .dependencies
                    .iter()
                    .all(|dep| done.contains(dep.as_str()))
        });
        let Some((id, _)) = next else {
            break;
        };
        done.insert(*id);
        order.push((*id).to_owned());
    }
    if done.len() != tasks.len() && issues.is_empty() {
        issues.push("dependency_cycle".to_owned());
    }
    issues.sort();
    Ok(GraphReview {
        agent_id: AGENT_ID,
        status: if issues.is_empty() {
            "structurally_valid"
        } else {
            "blocked"
        },
        task_count: input.tasks.len(),
        order,
        issues,
        execution_authorized: false,
        verified: false,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unknown_dependencies_are_blocked_not_run() {
        let graph = PlanInput {
            graph_id: "g".into(),
            tasks: vec![TaskNode {
                task_id: "build".into(),
                dependencies: vec!["unknown".into()],
                stage: "build".into(),
            }],
        };
        let result = review(&graph).unwrap();
        assert_eq!(result.status, "blocked");
        assert!(!result.execution_authorized);
    }

    #[test]
    fn duplicate_tasks_rejected() {
        let node = TaskNode {
            task_id: "same".into(),
            dependencies: vec![],
            stage: "plan".into(),
        };
        assert!(matches!(
            review(&PlanInput {
                graph_id: "g".into(),
                tasks: vec![node.clone(), node],
            }),
            Err(ReviewError::DuplicateTask)
        ));
    }
}
