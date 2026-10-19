//! Deterministic, non-effectful photographic planning domain.

use crate::security;

/// Stable technical agent identifier; never treat as certificate or serial.
pub const AGENT_ID: &str = "AM-OCAF-AI-0001";
/// Registered own name, distinct from product family and registry number.
pub const AGENT_NAME: &str = "WDA AURELION";
pub const REGISTRY_NUMBER: &str = "001";
/// Unique repository-issued administrative serial; not a signature.
pub const SERIAL_NUMBER: &str = "AM-WDA-AUR-2026-000001-DE338AC20B42";
pub const CATALOG_NUMBER: &str = "AM-OCAF-WDA-001";
pub const MAX_GOAL_BYTES: usize = 1024;

#[derive(Debug, PartialEq, Eq)]
pub enum PlanError {
    InvalidGoal,
    IdentityUnlockDenied,
}

#[derive(Debug, PartialEq, Eq)]
pub struct PhotoPlan {
    pub agent_id: &'static str,
    pub status: &'static str,
    pub identity_lock: bool,
    pub workflow: Vec<String>,
}

pub fn plan(goal: &str, identity_lock: bool) -> Result<PhotoPlan, PlanError> {
    if !identity_lock {
        return Err(PlanError::IdentityUnlockDenied);
    }
    security::validate_goal(goal).map_err(|_| PlanError::InvalidGoal)?;
    let normalized = goal.trim();
    Ok(PhotoPlan {
        agent_id: AGENT_ID,
        status: "plan_only",
        identity_lock: true,
        workflow: vec![
            "Observe the original photograph without inventing unseen details".to_owned(),
            format!("Plan a reversible retouching operation: {normalized}"),
            "Preserve face, pose, body proportions and natural texture".to_owned(),
            "Request separate operator approval for any external image service".to_owned(),
            "Compare the edited result to the unmodified original before delivery".to_owned(),
        ],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn locked_plan_is_deterministic() {
        let a = plan("adjust highlights", true).unwrap();
        let b = plan("adjust highlights", true).unwrap();
        assert_eq!(a, b);
        assert_eq!(a.agent_id, AGENT_ID);
        assert_eq!(a.status, "plan_only");
    }

    #[test]
    fn rejects_insecure_or_invalid_plan() {
        assert_eq!(plan("edit", false), Err(PlanError::IdentityUnlockDenied));
        assert_eq!(plan("  ", true), Err(PlanError::InvalidGoal));
        assert_eq!(plan(&"x".repeat(1025), true), Err(PlanError::InvalidGoal));
        assert_eq!(plan("a\nother", true), Err(PlanError::InvalidGoal));
        assert!(plan(&"ż".repeat(512), true).is_ok());
        assert_eq!(plan(&"ż".repeat(513), true), Err(PlanError::InvalidGoal));
    }
}
