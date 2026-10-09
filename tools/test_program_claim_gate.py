import unittest
from datetime import datetime, timezone
from program_claim_gate import review

NOW=datetime(2026,10,9,12,tzinfo=timezone.utc)
BASE={"repository":"mojealterego/Knowledge-projects","credential_in_payload":False,
      "program":"alibaba_ai_catalyst","action":"research",
      "credits_usd":120000,"tokens":2000000000}

def check(**kwargs):
    d=dict(BASE);d.update(kwargs);return review(d,now=NOW)

class ProgramClaimTests(unittest.TestCase):
    def test_public_research_allowed(self): self.assertTrue(check().permitted)
    def test_max_not_funding_award(self): self.assertTrue(check(credits_usd=120000).permitted)
    def test_false_award_denied(self): self.assertIn("funding_not_confirmed",check(status="awarded").reasons)
    def test_claim_above_max_credits_denied(self): self.assertIn("not_a_verified_credit_limit",check(credits_usd=120001).reasons)
    def test_token_claim_above_max_denied(self): self.assertIn("not_a_verified_token_limit",check(tokens=2000000001).reasons)
    def test_bool_credit_denied(self): self.assertIn("not_a_verified_credit_limit",check(credits_usd=True).reasons)
    def test_draft_requires_company_review(self): self.assertIn("company_eligibility_unverified",check(action="draft").reasons)
    def test_company_reviewed_draft_allowed(self): self.assertTrue(check(action="draft",company_eligibility_checked=True).permitted)
    def test_external_submission_forbidden(self): self.assertIn("external_action_not_authorized",check(action="submit").reasons)
    def test_costly_provision_forbidden(self): self.assertFalse(check(action="provision_compute").permitted)
    def test_cross_repo_forbidden(self): self.assertIn("only_single_repository_authorized",check(repository="mojealterego/ODYN-AI").reasons)
    def test_credentials_forbidden(self): self.assertIn("secret_must_not_be_in_payload",check(credential_in_payload=True).reasons)
    def test_hackathon_history_allowed(self): self.assertTrue(check(program="general_learning_hacks",status="ended").permitted)
    def test_hackathon_false_open_denied(self): self.assertIn("event_is_already_ended",check(program="general_learning_hacks",status="open").reasons)
    def test_hackathon_future_submission_denied(self): self.assertIn("past_event_not_submittable",check(program="general_learning_hacks",status="ended",action="draft").reasons)
    def test_unsupported_program_denied(self): self.assertFalse(check(program="unrelated").permitted)
    def test_no_network_client(self):
        import program_claim_gate as m
        self.assertNotIn("requests",m.__dict__)

if __name__=="__main__": unittest.main()
