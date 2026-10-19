//! Local-only CLI. Network transport and source decryption are never started.
use wda_photo_agent::{
    agent::{AGENT_NAME, SERIAL_NUMBER},
    plan,
    security::load_config,
};

fn main() -> std::process::ExitCode {
    let config = match load_config(include_str!("../config/agent.toml")) {
        Ok(config) => config,
        Err(_) => {
            eprintln!("Agent configuration rejected");
            return std::process::ExitCode::FAILURE;
        }
    };
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: wda_photo_agent <photographic retouch goal>");
        return std::process::ExitCode::FAILURE;
    }
    let goal = args.join(" ");
    match plan(&goal, config.safety.identity_lock_required) {
        Ok(output) => {
            println!(
                "{AGENT_NAME} | {SERIAL_NUMBER} | {} | {}",
                output.agent_id, output.status
            );
            for item in &output.workflow {
                println!("- {item}");
            }
            std::process::ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("Agent request rejected: {error:?}");
            std::process::ExitCode::FAILURE
        }
    }
}
