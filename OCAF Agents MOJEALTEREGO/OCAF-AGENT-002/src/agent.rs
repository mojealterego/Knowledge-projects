//! SRE incident classification from operator-supplied observations.
//! No infrastructure credentials, network calls, shell commands, or mutations.
use serde::{Deserialize, Serialize};

pub const AGENT_NAME: &str = "VIGILIS SRE";
pub const REGISTRY_NUMBER: &str = "002";
pub const AGENT_ID: &str = "AM-OCAF-AI-0002";
pub const SERIAL_NUMBER: &str = "AM-SRE-VIG-2026-000002";
pub const CATALOG_NUMBER: &str = "AM-OCAF-SRE-002";

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Incident {
    pub incident_id: String,
    pub tenant_id: String,
    pub signal: String,
    pub observations: Vec<String>,
    pub risk_tier: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TriageError {
    InvalidInput,
    UnsupportedSignal,
    InvalidRiskTier,
}

#[derive(Debug, Clone, Serialize)]
pub struct Triage {
    pub agent_id: &'static str,
    pub status: &'static str,
    pub signal: String,
    pub risk_tier: String,
    pub evidence_count: usize,
    pub allow_mutation: bool,
    pub verified: bool,
    pub recommendations: Vec<String>,
}

fn valid_token(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 64
        && value.bytes().all(|b| {
            b.is_ascii_alphanumeric() || b == b'_' || b == b'-'
        })
}

fn valid_observation(value: &str) -> bool {
    !value.trim().is_empty()
        && value.len() <= 256
        && !value.chars().any(char::is_control)
}

pub fn triage(input: &Incident) -> Result<Triage, TriageError> {
    if !valid_token(&input.incident_id)
        || !valid_token(&input.tenant_id)
        || input.observations.is_empty()
        || input.observations.len() > 8
        || !input.observations.iter().all(|s| valid_observation(s))
    {
        return Err(TriageError::InvalidInput);
    }
    if !matches!(input.risk_tier.as_str(), "R0" | "R1" | "R2" | "R3" | "R4") {
        return Err(TriageError::InvalidRiskTier);
    }
    let investigation = match input.signal.as_str() {
        "CrashLoopBackOff" => "Inspect the existing pod status, prior container logs and declared probes",
        "NodeNotReady" => "Review existing node conditions, event timeline and capacity signals",
        "PodPending" => "Review existing scheduling events, requests, limits and quota metadata",
        "ServiceUnavailable" => "Review existing service endpoints, target health and incident timing",
        _ => return Err(TriageError::UnsupportedSignal),
    };
    let mut recommendations = vec![
        "Treat all observations as unverified operator-provided statements".to_owned(),
        investigation.to_owned(),
        "Acquire independent read-only evidence with an authorized operator".to_owned(),
    ];
    if matches!(input.risk_tier.as_str(), "R2" | "R3" | "R4") {
        recommendations.push("Escalate to the responsible operator before any remediation".to_owned());
    }
    recommendations.push("Require independent postcondition verification before closure".to_owned());
    Ok(Triage {
        agent_id: AGENT_ID,
        status: "read_only",
        signal: input.signal.clone(),
        risk_tier: input.risk_tier.clone(),
        evidence_count: input.observations.len(),
        allow_mutation: false,
        verified: false,
        recommendations,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn incident() -> Incident {
        Incident {
            incident_id: "INC-1".into(),
            tenant_id: "tenant1".into(),
            signal: "CrashLoopBackOff".into(),
            risk_tier: "R4".into(),
            observations: vec!["restart loop seen".into()],
        }
    }

    #[test]
    fn high_risk_never_authorizes_a_change() {
        let result = triage(&incident()).unwrap();
        assert!(!result.allow_mutation);
        assert!(!result.verified);
        assert!(result.recommendations.iter().any(|x| x.contains("Escalate")));
    }

    #[test]
    fn malformed_incidents_are_denied() {
        let mut x = incident();
        x.tenant_id = "a/../b".into();
        assert!(matches!(triage(&x), Err(TriageError::InvalidInput)));
        x = incident();
        x.risk_tier = "R5".into();
        assert!(matches!(triage(&x), Err(TriageError::InvalidRiskTier)));
        x = incident();
        x.signal = "run kubectl delete".into();
        assert!(matches!(triage(&x), Err(TriageError::UnsupportedSignal)));
    }
}
