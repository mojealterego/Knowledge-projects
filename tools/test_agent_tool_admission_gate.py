import unittest
from datetime import datetime, timezone
from hashlib import sha256
from agent_tool_admission_gate import admit_tool, SANDBOX_PROFILE

ARTIFACT = b"# standalone proposed tool, never executed by gate\n"
DIGEST = sha256(ARTIFACT).hexdigest()
NOW = datetime(2026, 10, 8, 18, 0, tzinfo=timezone.utc)


def good_manifest():
    return dict(artifact_sha256=DIGEST, capabilities=["parse_document"],
                sandbox_profile=SANDBOX_PROFILE, timeout_seconds=30,
                expires_at="2026-10-09T00:00:00Z", external_side_effects=False)


def approval():
    return {DIGEST: frozenset({"parse_document"})}


class ToolAdmissionTests(unittest.TestCase):
    def check_denied(self, manifest, approvals=None, artifact=ARTIFACT):
        r = admit_tool(manifest, artifact, trusted_approvals=approval() if approvals is None else approvals, now=NOW)
        self.assertFalse(r.accepted)
        self.assertTrue(r.reasons)

    def test_valid_offline_artifact(self):
        item = admit_tool(good_manifest(), ARTIFACT, trusted_approvals=approval(), now=NOW)
        self.assertTrue(item.accepted)
        self.assertEqual(item.artifact_sha256, DIGEST)

    def test_no_independent_approval(self):
        self.check_denied(good_manifest(), approvals={})

    def test_self_reported_approval_never_suffices(self):
        m = good_manifest(); m["reviewer_approved"] = True
        self.check_denied(m, approvals={})

    def test_changed_bytes_rejected(self):
        self.check_denied(good_manifest(), artifact=b"changed")

    def test_unapproved_shell_capability(self):
        m = good_manifest(); m["capabilities"] = ["parse_document", "shell"]
        self.check_denied(m)

    def test_privileged_scope_is_denied_even_if_reviewed(self):
        m = good_manifest(); m["capabilities"] = ["network"]
        self.check_denied(m, approvals={DIGEST: frozenset({"network"})})

    def test_no_sandbox(self):
        m = good_manifest(); m["sandbox_profile"] = "host"
        self.check_denied(m)

    def test_no_side_effects_policy(self):
        m = good_manifest(); m["external_side_effects"] = True
        self.check_denied(m)

    def test_expired_approval(self):
        m = good_manifest(); m["expires_at"] = "2026-10-01T00:00:00Z"
        self.check_denied(m)

    def test_time_budget_overflow(self):
        m = good_manifest(); m["timeout_seconds"] = 100000
        self.check_denied(m)

    def test_rejects_boolean_timeout(self):
        m = good_manifest(); m["timeout_seconds"] = True
        self.check_denied(m)

    def test_capability_exceeds_external_review(self):
        m = good_manifest(); m["capabilities"] = ["parse_document", "format_text"]
        self.check_denied(m)

    def test_invalid_expiration(self):
        m = good_manifest(); m["expires_at"] = "whenever"
        self.check_denied(m)

    def test_no_executable_code_import(self):
        import agent_tool_admission_gate as module
        self.assertNotIn("subprocess", module.__dict__)
        self.assertNotIn("importlib", module.__dict__)


if __name__ == "__main__":
    unittest.main()
