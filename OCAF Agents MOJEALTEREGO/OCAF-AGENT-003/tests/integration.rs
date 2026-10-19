use evidentia_research_agent::agent::{assess, Evidence, ResearchInput};
#[test]
fn contradictory_evidence_must_not_be_presented_as_verified() {
    let q = ResearchInput {
        claim: "A claim requiring independent verification".into(),
        evidence: vec![
            Evidence {
                source_id: "s1".into(),
                stance: "supports".into(),
                excerpt: "source says yes".into(),
            },
            Evidence {
                source_id: "s2".into(),
                stance: "contradicts".into(),
                excerpt: "source says no".into(),
            },
        ],
    };
    let result = assess(&q).unwrap();
    assert!(result.has_contradiction);
    assert_ne!(result.status, "verified");
}
