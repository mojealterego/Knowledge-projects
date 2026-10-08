import unittest
from datetime import datetime, timezone
from dgm_cycle_gate import assess_promotion, EXPECTED_PHASES

C = "a" * 40
B = "b" * 40
NOW = datetime(2026, 10, 8, 19, tzinfo=timezone.utc)


def valid():
    return {
        "candidate_sha": C, "base_sha": B, "branch": "integration/dgm-candidate",
        "phase_receipts": {p: "trusted:receipt-" + p for p in EXPECTED_PHASES},
        "required_checks": ["tests", "security"],
        "approval_expires_at": "2026-10-09T00:00:00Z",
        "external_side_effects": False, "rollback_ref_verified": True,
        "source_provenance_verified": True,
    }


def call(proposal, *, approved=frozenset({C}), ci=None):
    return assess_promotion(proposal, verified_ci_by_candidate=ci or {C: frozenset({"tests","security"})},
                            approved_candidates=approved, now=NOW)


class DgmCycleGateTests(unittest.TestCase):
    def bad(self, change, expected):
        p = valid()
        p.update(change)
        decision = call(p)
        self.assertFalse(decision.admitted)
        self.assertIn(expected, decision.reasons)

    def test_all_independent_gates_pass(self):
        self.assertTrue(call(valid()).admitted)

    def test_missing_phase(self):
        self.bad({"phase_receipts": {"ModelScout":"ok"}}, "missing_four_phase_receipts")

    def test_bad_candidate_digest(self):
        self.bad({"candidate_sha": "oops"}, "invalid_candidate_sha")

    def test_same_parent_sha(self):
        self.bad({"base_sha": C}, "invalid_base_sha")

    def test_forbid_default_branch(self):
        self.bad({"branch": "main"}, "not_an_isolated_integration_branch")

    def test_model_self_approval_does_not_count(self):
        p = valid()
        p["approved_by_agent"] = True
        self.assertIn("not_independently_approved", call(p, approved=frozenset()).reasons)

    def test_missing_ci_evidence(self):
        p = valid()
        self.assertIn("missing_verified_ci_check",
                      call(p, ci={C: frozenset({"tests"})}).reasons)

    def test_expired(self):
        self.bad({"approval_expires_at":"2025-01-01T00:00:00Z"}, "approval_expired")

    def test_invalid_expiration(self):
        self.bad({"approval_expires_at":"tomorrow"}, "invalid_approval_expiration")

    def test_no_external_side_effect(self):
        self.bad({"external_side_effects": True}, "external_side_effects_forbidden")

    def test_missing_rollback(self):
        self.bad({"rollback_ref_verified": False}, "missing_rollback_evidence")

    def test_missing_source_provenance(self):
        self.bad({"source_provenance_verified": False}, "missing_source_provenance")

    def test_no_subprocess_or_dynamic_import(self):
        import dgm_cycle_gate
        self.assertNotIn("subprocess", dgm_cycle_gate.__dict__)
        self.assertNotIn("importlib", dgm_cycle_gate.__dict__)


if __name__ == "__main__":
    unittest.main()
