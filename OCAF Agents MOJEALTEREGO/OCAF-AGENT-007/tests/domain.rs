use vigia_loop_agent::agent::{Guard, Limits, Observation};
const A: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const B: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
fn obs(tool: &str, digest: &str, tokens: u64) -> Observation {
    Observation {
        tool: tool.into(),
        args_sha256: digest.into(),
        estimated_tokens: tokens,
    }
}
#[test]
fn repeated_calls_halt_at_fourth_and_remain_halted() {
    let mut g = Guard::new(Limits {
        max_calls: 50,
        max_tokens: 100000,
        max_identical_no_progress: 3,
    })
    .unwrap();
    for _ in 0..3 {
        assert!(g.observe(obs("read_file", A, 1)).allowed);
    }
    let fourth = g.observe(obs("read_file", A, 1));
    assert_eq!(fourth.reason, "identical_call_without_progress");
    assert_eq!(fourth.total_calls, 3);
    assert_eq!(g.observe(obs("other", B, 1)).reason, "previously_halted");
}
#[test]
fn trusted_progress_after_external_readback_resets_repeats() {
    let mut g = Guard::new(Limits {
        max_calls: 10,
        max_tokens: 10,
        max_identical_no_progress: 1,
    })
    .unwrap();
    assert!(g.observe(obs("read_file", A, 1)).allowed);
    g.advance_verified_progress(1).unwrap();
    assert!(g.observe(obs("read_file", A, 1)).allowed);
}
#[test]
fn token_budget_cannot_be_overrun() {
    let mut g = Guard::new(Limits {
        max_calls: 2,
        max_tokens: 4,
        max_identical_no_progress: 3,
    })
    .unwrap();
    assert!(g.observe(obs("read_file", A, 3)).allowed);
    assert_eq!(
        g.observe(obs("diff", B, 2)).reason,
        "token_budget_exhausted"
    );
}
