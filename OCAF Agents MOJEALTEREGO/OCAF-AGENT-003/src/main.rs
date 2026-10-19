//! Demonstration CLI: hypothetical sources, not verified citations.
use evidentia_research_agent::agent::{assess, Evidence, ResearchInput};
use evidentia_research_agent::security::load_config;

fn main() {
    let conf = load_config(include_str!("../config/agent.toml")).expect("invalid identity");
    let claim = std::env::args().nth(1).unwrap_or_else(|| "Demo hypothesis".into());
    let input = ResearchInput {
        claim,
        evidence: vec![
            Evidence { source_id: "demo1".into(), stance: "supports".into(), excerpt: "hypothetical only".into() },
            Evidence { source_id: "demo2".into(), stance: "contradicts".into(), excerpt: "hypothetical only".into() },
        ],
    };
    let out = assess(&input).expect("invalid input");
    println!("{} | {} | {}", conf.identity.unique_name, conf.identity.serial_number, out.status);
    for conclusion in out.conclusions {
        println!("- {conclusion}");
    }
}
