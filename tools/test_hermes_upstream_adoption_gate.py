import unittest
from datetime import datetime,timedelta,timezone
from hermes_upstream_adoption_gate import Decision,TrustedReview,evaluate_upstream_adoption

NOW=datetime(2026,10,9,12,tzinfo=timezone.utc)
SHA="c"*40
def plan():
    return dict(repository="NousResearch/hermes-example-plugins",revision_sha=SHA,
                mode="runtime_adoption",is_archived=False,is_fork=False,
                license_declared=True,review_ticket="external-review",
                external_side_effects=False)
def trusted(**changes):
    values=dict(repository="NousResearch/hermes-example-plugins",revision_sha=SHA,
      source_checked_at=NOW,license_reviewed=True,security_reviewed=True,
      tests_passed=True,fork_synced=True,owner_approved=True)
    values.update(changes)
    return {"external-review":TrustedReview(**values)}
def decide(p=None, t=None):
    return evaluate_upstream_adoption(plan() if p is None else p,
       trusted_reviews=trusted() if t is None else t,now=NOW)

class AdoptionGateTests(unittest.TestCase):
    def test_runtime_adoption_with_verified_scope(self):
        self.assertTrue(decide().allowed)

    def test_archived_cannot_be_promoted(self):
        p=plan();p["is_archived"]=True
        self.assertIn("archived_or_unknown_repo_cannot_be_runtime",decide(p).reasons)

    def test_document_only_archived_metadata_accepted(self):
        p=plan();p.update(mode="document_only",is_archived=True,license_declared=False)
        self.assertTrue(decide(p,t={}).allowed)

    def test_missing_authoritative_review_denied(self):
        self.assertIn("no_independent_review",decide(t={}).reasons)

    def test_model_self_review_not_enough(self):
        p=plan();p["model_approved"]=True
        self.assertIn("no_independent_review",decide(p,t={}).reasons)

    def test_fork_unsynced_blocks_runtime(self):
        p=plan();p["is_fork"]=True
        self.assertIn("fork_sync_unverified",decide(p,t=trusted(fork_synced=False)).reasons)

    def test_stale_review_blocks(self):
        self.assertIn("stale_upstream_review",decide(t=trusted(source_checked_at=NOW-timedelta(days=31))).reasons)

    def test_revision_mismatch_blocks(self):
        self.assertIn("review_revision_scope_mismatch",decide(t=trusted(revision_sha="a"*40)).reasons)

    def test_license_not_verified_blocks(self):
        self.assertIn("license_not_independently_checked",decide(t=trusted(license_reviewed=False)).reasons)

    def test_missing_security_checks_denied(self):
        self.assertIn("missing_security_ci_evidence",decide(t=trusted(security_reviewed=False)).reasons)

    def test_no_owner_grant_denied(self):
        self.assertIn("owner_approval_missing",decide(t=trusted(owner_approved=False)).reasons)

    def test_unpinned_source_denied(self):
        p=plan();p["revision_sha"]="main"
        self.assertIn("unpinned_revision",decide(p).reasons)

    def test_unknown_org_denied(self):
        p=plan();p["repository"]="untrusted/unknown"
        self.assertIn("repository_scope_not_allowed",decide(p).reasons)

    def test_side_effects_denied(self):
        p=plan();p["external_side_effects"]=True
        self.assertIn("effects_not_allowed",decide(p).reasons)

    def test_unknown_mode_denied(self):
        p=plan();p["mode"]="execute_now"
        self.assertIn("unknown_mode",decide(p).reasons)

    def test_isolated_eval_licenses_without_runtime_privileges(self):
        p=plan();p["mode"]="isolated_evaluation"
        self.assertTrue(decide(p,t=trusted(security_reviewed=False,tests_passed=False,owner_approved=False)).allowed)

    def test_no_unsafe_imports(self):
        import hermes_upstream_adoption_gate as mod
        self.assertNotIn("subprocess",mod.__dict__)
        self.assertNotIn("requests",mod.__dict__)

if __name__=="__main__":
    unittest.main()
