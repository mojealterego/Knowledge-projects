use vigia_loop_agent::agent::{Guard, Limits, Observation};
use vigia_loop_agent::security::load_config;
fn main() {
    let c = load_config(include_str!("../config/agent.toml")).expect("bad identity");
    let mut gate = Guard::new(Limits {
        max_calls: 5,
        max_tokens: 100,
        max_identical_no_progress: 2,
    })
    .expect("bad limits");
    let result = gate.observe(Observation {
        tool: "read_file".into(),
        args_sha256: "a".repeat(64),
        estimated_tokens: 10,
    });
    println!(
        "{} | {} | {} | execution_authorized={}",
        c.identity.unique_name,
        c.identity.serial_number,
        result.reason,
        result.execution_authorized
    );
}
