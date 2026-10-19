#![forbid(unsafe_code)]

//! A domain-level photographic planning tool. It deliberately does not access
//! images, credentials, networks, filesystems, or external editing services.

pub const AGENT_ID: &str = "AM-OCAF-AI-0001";
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

/// Produce a reversible plan. Approval and actual effects belong to a
/// separately authenticated host, never to this function.
pub fn plan(goal: &str, identity_lock: bool) -> Result<PhotoPlan, PlanError> {
    if !identity_lock {
        return Err(PlanError::IdentityUnlockDenied);
    }
    let normalized = goal.trim();
    if normalized.is_empty()
        || normalized.len() > MAX_GOAL_BYTES
        || normalized.chars().any(char::is_control)
    {
        return Err(PlanError::InvalidGoal);
    }
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

#[cfg(feature = "ocaf-integration")]
pub mod ocaf;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_identity_locked_non_effectful_plan() {
        let output = plan("  correct highlights  ", true).unwrap();
        assert_eq!(output.agent_id, AGENT_ID);
        assert_eq!(output.status, "plan_only");
        assert!(output.identity_lock);
        assert_eq!(output.workflow.len(), 5);
        assert!(output.workflow[1].contains("correct highlights"));
    }

    #[test]
    fn rejects_unlock_empty_control_chars_and_oversize() {
        assert_eq!(plan("adjust", false), Err(PlanError::IdentityUnlockDenied));
        assert_eq!(plan("  ", true), Err(PlanError::InvalidGoal));
        assert_eq!(plan(&"x".repeat(1025), true), Err(PlanError::InvalidGoal));
        assert_eq!(plan("line\nnext", true), Err(PlanError::InvalidGoal));
    }

    #[test]
    fn size_is_validated_as_utf8_bytes() {
        assert!(plan(&"ż".repeat(512), true).is_ok());
        assert_eq!(plan(&"ż".repeat(513), true), Err(PlanError::InvalidGoal));
    }
}
