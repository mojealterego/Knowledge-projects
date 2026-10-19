//! Demonstration only, no engineering side effects.
use axiom_forge_agent::agent::{review, PlanInput, TaskNode};
use axiom_forge_agent::security::load_config;

fn main() {
    let config = load_config(include_str!("../config/agent.toml")).expect("invalid identity");
    let graph = PlanInput {
        graph_id: "demo".into(),
        tasks: vec![
            TaskNode {
                task_id: "build".into(),
                dependencies: vec!["plan".into()],
                stage: "build".into(),
            },
            TaskNode {
                task_id: "plan".into(),
                dependencies: vec![],
                stage: "plan".into(),
            },
        ],
    };
    let output = review(&graph).expect("invalid graph");
    println!(
        "{} | {} | {}",
        config.identity.unique_name, config.identity.serial_number, output.status
    );
    for node in output.order {
        println!("review step: {node}");
    }
}
