import unittest
from agent_tool_loop_guard import AgentToolLoopGuard

A = "a" * 64
B = "b" * 64

class LoopGuardTests(unittest.TestCase):
    def test_initial_call_allowed(self):
        r = AgentToolLoopGuard().observe(tool="read_file", args_sha256=A,
                  trusted_progress_version=0, estimated_tokens=10)
        self.assertTrue(r.allowed)
        self.assertEqual(r.total_calls, 1)

    def test_third_identical_without_progress_allowed(self):
        g = AgentToolLoopGuard(max_identical_no_progress=3)
        for _ in range(3):
            self.assertTrue(g.observe(tool="read_file", args_sha256=A,
                   trusted_progress_version=0, estimated_tokens=2).allowed)

    def test_fourth_identical_halts(self):
        g = AgentToolLoopGuard(max_identical_no_progress=3)
        for _ in range(3):
            g.observe(tool="read_file", args_sha256=A,
                      trusted_progress_version=0, estimated_tokens=1)
        r = g.observe(tool="read_file", args_sha256=A,
                      trusted_progress_version=0, estimated_tokens=1)
        self.assertEqual(r.reason, "identical_call_without_progress")
        self.assertEqual(r.total_calls, 3)

    def test_after_halt_cannot_restart(self):
        g = AgentToolLoopGuard(max_identical_no_progress=1)
        g.observe(tool="read_file", args_sha256=A, trusted_progress_version=0, estimated_tokens=1)
        g.observe(tool="read_file", args_sha256=A, trusted_progress_version=0, estimated_tokens=1)
        self.assertEqual(g.observe(tool="diff", args_sha256=B,
                        trusted_progress_version=2, estimated_tokens=1).reason,"previously_halted")

    def test_trusted_progress_resets_repeat(self):
        g = AgentToolLoopGuard(max_identical_no_progress=2)
        g.observe(tool="read_file", args_sha256=A, trusted_progress_version=0, estimated_tokens=1)
        g.observe(tool="read_file", args_sha256=A, trusted_progress_version=0, estimated_tokens=1)
        self.assertTrue(g.observe(tool="read_file", args_sha256=A,
                                  trusted_progress_version=1, estimated_tokens=1).allowed)

    def test_different_args_reset_repeat(self):
        g = AgentToolLoopGuard(max_identical_no_progress=1)
        g.observe(tool="read_file", args_sha256=A, trusted_progress_version=0, estimated_tokens=1)
        self.assertTrue(g.observe(tool="read_file", args_sha256=B,
                                  trusted_progress_version=0, estimated_tokens=1).allowed)

    def test_call_limit_halts(self):
        g = AgentToolLoopGuard(max_calls=1)
        g.observe(tool="read_file", args_sha256=A, trusted_progress_version=0, estimated_tokens=1)
        self.assertEqual(g.observe(tool="diff", args_sha256=B,
                        trusted_progress_version=1, estimated_tokens=1).reason,"call_budget_exhausted")

    def test_token_limit_halts(self):
        g = AgentToolLoopGuard(max_tokens=4)
        g.observe(tool="read_file", args_sha256=A, trusted_progress_version=0, estimated_tokens=3)
        self.assertEqual(g.observe(tool="diff", args_sha256=B,
                        trusted_progress_version=1, estimated_tokens=3).reason,"token_budget_exhausted")

    def test_progress_cannot_go_backward(self):
        g = AgentToolLoopGuard()
        g.observe(tool="read_file", args_sha256=A, trusted_progress_version=5, estimated_tokens=1)
        self.assertEqual(g.observe(tool="read_file", args_sha256=A,
                        trusted_progress_version=4, estimated_tokens=1).reason,"invalid_progress_version")

    def test_rejects_boolean_limits(self):
        with self.assertRaises(ValueError): AgentToolLoopGuard(max_calls=True)

    def test_rejects_invalid_digest(self):
        self.assertEqual(AgentToolLoopGuard().observe(tool="read_file", args_sha256="../token",
                         trusted_progress_version=0, estimated_tokens=1).reason,"invalid_argument_digest")

    def test_no_exec_or_network(self):
        import agent_tool_loop_guard as mod
        self.assertNotIn("subprocess", mod.__dict__)
        self.assertNotIn("requests", mod.__dict__)

if __name__=="__main__":
    unittest.main()
