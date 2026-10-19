//! Fail-closed lifecycle: development identity does not authorize deployment.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Stage {
    Draft,
    Tested,
    Staged,
    Active,
    Retired,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum LifecycleError {
    InvalidTransition,
    DeploymentNotAuthorized,
}

pub fn transition(from: Stage, to: Stage) -> Result<Stage, LifecycleError> {
    if to == Stage::Active {
        // Requires external, genuine OMAs+OMAt+OSTc receipts and operator acceptance.
        return Err(LifecycleError::DeploymentNotAuthorized);
    }
    if matches!(
        (from, to),
        (Stage::Draft, Stage::Tested)
            | (Stage::Tested, Stage::Staged)
            | (Stage::Draft | Stage::Tested | Stage::Staged, Stage::Retired)
    ) {
        Ok(to)
    } else {
        Err(LifecycleError::InvalidTransition)
    }
}
