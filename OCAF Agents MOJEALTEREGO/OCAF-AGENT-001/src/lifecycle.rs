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
        _evidence: Option<ReleaseEvidence>,
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
        // Fail closed until real signed OMAs/OMAt receipts and independently
        // verified OSTc capabilities are implemented. Caller-controlled
        // booleans must never authorize an ACTIVE transition.
        if next == Active {
            return Err(LifecycleError::ReleaseGateNotSatisfied);
        }
        self.state = next;
        Ok(())
    }
}
