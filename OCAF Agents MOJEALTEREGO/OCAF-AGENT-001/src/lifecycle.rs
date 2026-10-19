//! Controlled local lifecycle of agent 001. Deployment requires independent release gates.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgentState {
    Draft,
    Validated,
    Tested,
    Staged,
    Active,
    Paused,
    Retired,
}

#[derive(Debug, Clone, Copy)]
pub struct ReleaseEvidence {
    pub tests_passed: bool,
    pub omas_authenticated: bool,
    pub omat_authorized: bool,
    pub ostc_source_access_verified: bool,
    pub operator_approved: bool,
}

impl ReleaseEvidence {
    fn fully_approved(self) -> bool {
        self.tests_passed
            && self.omas_authenticated
            && self.omat_authorized
            && self.ostc_source_access_verified
            && self.operator_approved
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum LifecycleError {
    InvalidTransition,
    ReleaseGateNotSatisfied,
}

#[derive(Debug)]
pub struct Lifecycle {
    state: AgentState,
}

impl Default for Lifecycle {
    fn default() -> Self {
        Self::new()
    }
}

impl Lifecycle {
    pub fn new() -> Self {
        Self {
            state: AgentState::Draft,
        }
    }

    pub fn state(&self) -> AgentState {
        self.state
    }

    pub fn transition(
        &mut self,
        next: AgentState,
        evidence: Option<ReleaseEvidence>,
    ) -> Result<(), LifecycleError> {
        use AgentState::{Active, Draft, Paused, Retired, Staged, Tested, Validated};
        let legal = matches!(
            (self.state, next),
            (Draft, Validated)
                | (Validated, Tested)
                | (Tested, Staged)
                | (Staged, Active)
                | (Active, Paused)
                | (Paused, Active)
                | (
                    Draft | Validated | Tested | Staged | Paused | Active,
                    Retired
                )
        );
        if !legal {
            return Err(LifecycleError::InvalidTransition);
        }
        if next == Active && !evidence.is_some_and(ReleaseEvidence::fully_approved) {
            return Err(LifecycleError::ReleaseGateNotSatisfied);
        }
        self.state = next;
        Ok(())
    }
}
