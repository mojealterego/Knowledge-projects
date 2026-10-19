//! Research evidence assessment from *operator supplied* claims only.
//! This code NEVER retrieves or independently authenticates research sources.
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub const AGENT_NAME: &str = "EVIDENTIA RESEARCH";
pub const REGISTRY_NUMBER: &str = "003";
pub const AGENT_ID: &str = "AM-OCAF-AI-0003";
pub const SERIAL_NUMBER: &str = "AM-RES-EVI-2026-000003";
pub const CATALOG_NUMBER: &str = "AM-OCAF-RES-003";

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Evidence {
    pub source_id: String,
    pub stance: String,
    pub excerpt: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResearchInput {
    pub claim: String,
    pub evidence: Vec<Evidence>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ResearchError {
    InvalidClaim,
    InvalidEvidence,
    DuplicateSource,
}

#[derive(Debug, Serialize)]
pub struct Assessment {
    pub agent_id: &'static str,
    pub status: &'static str,
    pub source_count: usize,
    pub has_contradiction: bool,
    pub verified: bool,
    pub conclusions: Vec<String>,
}

pub fn assess(input: &ResearchInput) -> Result<Assessment, ResearchError> {
    if input.claim.trim().is_empty()
        || input.claim.len() > 1024
        || input.claim.chars().any(char::is_control)
    {
        return Err(ResearchError::InvalidClaim);
    }
    if input.evidence.len() < 2 || input.evidence.len() > 8 {
        return Err(ResearchError::InvalidEvidence);
    }
    let mut ids = HashSet::new();
    let mut supports = false;
    let mut contradicts = false;
    for e in &input.evidence {
        if e.source_id.is_empty()
            || e.source_id.len() > 64
            || !e.source_id.bytes().all(|b| {
                b.is_ascii_alphanumeric() || b == b'_' || b == b'-'
            })
            || e.excerpt.trim().is_empty()
            || e.excerpt.len() > 256
            || e.excerpt.chars().any(char::is_control)
            || !matches!(e.stance.as_str(), "supports" | "contradicts" | "unclear")
        {
            return Err(ResearchError::InvalidEvidence);
        }
        if !ids.insert(e.source_id.to_ascii_lowercase()) {
            return Err(ResearchError::DuplicateSource);
        }
        supports |= e.stance == "supports";
        contradicts |= e.stance == "contradicts";
    }
    let disputed = supports && contradicts;
    Ok(Assessment {
        agent_id: AGENT_ID,
        status: if disputed { "disputed" } else { "unverified" },
        source_count: input.evidence.len(),
        has_contradiction: disputed,
        verified: false,
        conclusions: vec![
            "Source IDs and excerpts are unverified operator claims".into(),
            if disputed {
                "Contradictory stances require independent adjudication"
            } else {
                "No independent verification has been performed"
            }.into(),
            "Do not publish the claim as established fact".into(),
        ],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn example() -> ResearchInput {
        ResearchInput {
            claim: "A factual hypothesis".into(),
            evidence: vec![
                Evidence { source_id: "source1".into(), stance: "supports".into(), excerpt: "reported support".into() },
                Evidence { source_id: "source2".into(), stance: "contradicts".into(), excerpt: "reported disagreement".into() },
            ],
        }
    }

    #[test]
    fn conflict_never_becomes_a_verified_fact() {
        let outcome = assess(&example()).unwrap();
        assert_eq!(outcome.status, "disputed");
        assert!(!outcome.verified);
    }

    #[test]
    fn duplicate_and_invalid_stances_are_denied() {
        let mut x = example();
        x.evidence[1].source_id = "SOURCE1".into();
        assert!(matches!(assess(&x), Err(ResearchError::DuplicateSource)));
        x = example();
        x.evidence[1].stance = "verified".into();
        assert!(matches!(assess(&x), Err(ResearchError::InvalidEvidence)));
    }
}
