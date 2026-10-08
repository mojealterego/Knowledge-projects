import unittest

from single_repository_scope_gate import TARGET_REPO, review_github_operation

BRANCH = "policy/single-repository-only-2026-10-09"

class ScopeTests(unittest.TestCase):
    def test_read_odyn_is_allowed(self):
        v = review_github_operation("mojealterego/ODYN-AI", "fetch_file")
        self.assertTrue(v.permitted)

    def test_read_nous_is_allowed(self):
        self.assertTrue(review_github_operation("NousResearch/hermes-agent","get_repo").permitted)

    def test_create_branch_in_knowledge_is_allowed(self):
        self.assertTrue(review_github_operation(TARGET_REPO, "create_branch", target_branch=BRANCH).permitted)

    def test_blob_in_knowledge_work_branch_allowed(self):
        self.assertTrue(review_github_operation(TARGET_REPO, "create_blob", target_branch=BRANCH).permitted)

    def test_reject_odyn_branch(self):
        v = review_github_operation("mojealterego/ODYN-AI", "create_branch", target_branch=BRANCH)
        self.assertIn("external_repository_write_forbidden", v.reasons)

    def test_reject_odyn_pr(self):
        v = review_github_operation("mojealterego/ODYN-AI", "create_pull_request",pull_base="main", pull_head=BRANCH)
        self.assertFalse(v.permitted)

    def test_reject_odyn_merge_even_if_valid_head(self):
        v = review_github_operation("mojealterego/ODYN-AI", "merge_pull_request", pull_base="main",pull_head=BRANCH)
        self.assertIn("external_repository_write_forbidden", v.reasons)

    def test_reject_direct_write_to_main(self):
        v = review_github_operation(TARGET_REPO, "update_file", target_branch="main")
        self.assertIn("write_requires_working_branch", v.reasons)

    def test_reject_unscoped_write(self):
        v = review_github_operation(TARGET_REPO, "create_commit")
        self.assertFalse(v.permitted)

    def test_correct_pr_into_main(self):
        v = review_github_operation(TARGET_REPO, "create_pull_request",pull_base="main",pull_head=BRANCH)
        self.assertTrue(v.permitted)

    def test_correct_merge_into_main(self):
        self.assertTrue(review_github_operation(TARGET_REPO, "merge_pull_request",pull_base="main",pull_head=BRANCH).permitted)

    def test_reject_other_base(self):
        v = review_github_operation(TARGET_REPO, "create_pull_request",pull_base="codex/termux-five-goals",pull_head=BRANCH)
        self.assertIn("unexpected_pr_base",v.reasons)

    def test_reject_unknown_mutation(self):
        v = review_github_operation(TARGET_REPO,"change_default_branch",target_branch=BRANCH)
        self.assertFalse(v.permitted)

    def test_reject_incorrect_spelling_of_repo(self):
        v = review_github_operation("mojealterego/Knowledge-project", "update_ref", target_branch=BRANCH)
        self.assertFalse(v.permitted)

    def test_reject_suspicious_branch_ref(self):
        v = review_github_operation(TARGET_REPO, "update_ref", target_branch="policy/../main")
        self.assertFalse(v.permitted)

    def test_no_actual_git_api_calls(self):
        import single_repository_scope_gate as m
        self.assertNotIn("requests",m.__dict__)
        self.assertNotIn("subprocess",m.__dict__)

if __name__=="__main__":
    unittest.main()
