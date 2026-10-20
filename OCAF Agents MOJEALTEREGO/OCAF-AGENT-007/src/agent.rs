//! Stateful Rust port of tools/agent_tool_loop_guard.py.
//! Trusted progress must be injected by the host after an authoritative readback.
use serde::{Deserialize, Serialize};

pub const AGENT_NAME: &str = "VIGIA LOOP";
pub const REGISTRY_NUMBER: &str = "007";
pub const AGENT_ID: &str = "AM-OCAF-AI-0007";
pub const SERIAL_NUMBER: &str = "AM-OPS-VIG-2026-000007";
pub const CATALOG_NUMBER: &str = "AM-OCAF-OPS-007";

#[derive(Debug, Clone, Copy)]
pub struct Limits {
    pub max_calls: u64,
    pub max_tokens: u64,
    pub max_identical_no_progress: u64,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Observation {
    pub tool: String,
    pub args_sha256: String,
    pub estimated_tokens: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct LoopDecision {
    pub agent_id: &'static str,
    pub allowed: bool,
    pub reason: &'static str,
    pub total_calls: u64,
    pub total_tokens: u64,
    pub identical_no_progress: u64,
    pub execution_authorized: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GuardError {
    InvalidLimits,
    InvalidProgressVersion,
}

pub struct Guard {
    limits: Limits,
    calls: u64,
    tokens: u64,
    repeat: u64,
    last_call: Option<(String, String)>,
    observed_version: u64,
    trusted_progress_version: u64,
    halted: bool,
}

impl Guard {
    pub fn new(limits: Limits) -> Result<Self, GuardError> {
        if limits.max_calls == 0
            || limits.max_tokens == 0
            || limits.max_identical_no_progress == 0
        {
            return Err(GuardError::InvalidLimits);
        }
        Ok(Self {
            limits, calls: 0, tokens: 0, repeat: 0,
            last_call: None, observed_version: 0, trusted_progress_version: 0,
            halted: false,
        })
    }

    /// Host-only API: caller MUST prove new_version from independent postcondition readback.
    /// Never derive this value from a model-authored tool call or completion text.
    pub fn advance_verified_progress(&mut self, new_version: u64) -> Result<(), GuardError> {
        if self.halted || new_version <= self.trusted_progress_version {
            return Err(GuardError::InvalidProgressVersion);
        }
        self.trusted_progress_version = new_version;
        Ok(())
    }

    fn outcome(&self, allowed: bool, reason: &'static str) -> LoopDecision {
        LoopDecision {
            agent_id: AGENT_ID, allowed, reason,
            total_calls: self.calls, total_tokens: self.tokens,
            identical_no_progress: self.repeat, execution_authorized: false,
        }
    }

    fn halt(&mut self, reason: &'static str) -> LoopDecision {
        self.halted = true;
        self.outcome(false, reason)
    }

    pub fn observe(&mut self, observation: Observation) -> LoopDecision {
        if self.halted {
            return self.outcome(false, "previously_halted");
        }
        if observation.tool.trim().is_empty()
            || observation.tool.len() > 128
            || observation.tool.chars().any(char::is_control)
        {
            return self.halt("invalid_tool");
        }
        if observation.args_sha256.len() != 64
            || !observation.args_sha256.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
        {
            return self.halt("invalid_argument_digest");
        }
        let Some(next_calls) = self.calls.checked_add(1) else {
            return self.halt("call_budget_exhausted");
        };
        if next_calls > self.limits.max_calls {
            return self.halt("call_budget_exhausted");
        }
        let Some(next_tokens) = self.tokens.checked_add(observation.estimated_tokens) else {
            return self.halt("token_budget_exhausted");
        };
        if next_tokens > self.limits.max_tokens {
            return self.halt("token_budget_exhausted");
        }
        let fingerprint = (observation.tool, observation.args_sha256);
        let repeat = if self.last_call.as_ref() == Some(&fingerprint)
            && self.observed_version == self.trusted_progress_version
        {
            self.repeat.saturating_add(1)
        } else {
            1
        };
        if repeat > self.limits.max_identical_no_progress {
            return self.halt("identical_call_without_progress");
        }
        self.calls = next_calls;
        self.tokens = next_tokens;
        self.repeat = repeat;
        self.last_call = Some(fingerprint);
        self.observed_version = self.trusted_progress_version;
        self.outcome(true, "within_budget")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn good() -> Observation {
        Observation {
            tool: "read_file".into(),
            args_sha256: "a".repeat(64),
            estimated_tokens: 2,
        }
    }

    #[test]
    fn hard_call_limit_and_halt_are_sticky() {
        let mut g = Guard::new(Limits{max_calls:1,max_tokens:10,max_identical_no_progress:5}).unwrap();
        assert!(g.observe(good()).allowed);
        assert_eq!(g.observe(good()).reason, "call_budget_exhausted");
        assert_eq!(g.observe(good()).reason, "previously_halted");
    }

    #[test]
    fn invalid_digest_halts() {
        let mut g = Guard::new(Limits{max_calls:10,max_tokens:10,max_identical_no_progress:3}).unwrap();
        let mut item = good();
        item.args_sha256 = "../path".into();
        assert_eq!(g.observe(item).reason, "invalid_argument_digest");
    }

    #[test]
    fn reported_progress_cannot_be_reverted() {
        let mut g = Guard::new(Limits{max_calls:10,max_tokens:10,max_identical_no_progress:3}).unwrap();
        g.advance_verified_progress(5).unwrap();
        assert_eq!(g.advance_verified_progress(4), Err(GuardError::InvalidProgressVersion));
    }
}
