import unittest
from datetime import datetime, timezone, timedelta
from marketplace_agent_review_gate import evaluate_marketplace_agent, OwnerGrant

NOW=datetime(2026,10,9,12,tzinfo=timezone.utc)
GRANT={"approval-1":OwnerGrant(
    repository="mojealterego/Knowledge-projects",
    listing_slug="sonarqube-agent",
    capabilities=frozenset({"view_existing_pr","comment_on_pr"}),
    expires_at=NOW+timedelta(days=1),approved=True)}

def proposed():
    return dict(listing_slug="sonarqube-agent",
                listing_url="https://github.com/marketplace/sonarqube-agent",
                repository="mojealterego/Knowledge-projects",
                grant_id="approval-1",capability="view_existing_pr",
                secrets_in_prompt=False,data_export_allowed=False,
                explicit_effect_approval=False)

class GateTests(unittest.TestCase):
    def test_read_with_external_grant(self):
        self.assertTrue(evaluate_marketplace_agent(proposed(),grants=GRANT,now=NOW).accepted)

    def test_effect_requires_extra_approval(self):
        p=proposed();p["capability"]="comment_on_pr"
        self.assertIn("effectful_action_not_approved",evaluate_marketplace_agent(p,grants=GRANT,now=NOW).reasons)

    def test_effect_approved(self):
        p=proposed();p["capability"]="comment_on_pr";p["explicit_effect_approval"]=True
        self.assertTrue(evaluate_marketplace_agent(p,grants=GRANT,now=NOW).accepted)

    def test_self_declared_verified_publisher_not_enough(self):
        p=proposed();p["verified_publisher"]=True
        self.assertIn("missing_external_owner_approval",evaluate_marketplace_agent(p,grants={},now=NOW).reasons)

    def test_wrong_listing(self):
        p=proposed();p["listing_slug"]="packfiles-agent"
        self.assertIn("listing_url_mismatch",evaluate_marketplace_agent(p,grants=GRANT,now=NOW).reasons)

    def test_wrong_repository(self):
        p=proposed();p["repository"]="somebody-else/private"
        self.assertIn("approval_scope_mismatch",evaluate_marketplace_agent(p,grants=GRANT,now=NOW).reasons)

    def test_shell_capability_not_allowed(self):
        p=proposed();p["capability"]="shell_anywhere"
        self.assertIn("unknown_capability",evaluate_marketplace_agent(p,grants=GRANT,now=NOW).reasons)

    def test_no_deploy_approval(self):
        p=proposed();p["capability"]="trigger_deployment";p["explicit_effect_approval"]=True
        self.assertIn("capability_not_approved",evaluate_marketplace_agent(p,grants=GRANT,now=NOW).reasons)

    def test_expired(self):
        past={"approval-1":OwnerGrant(GRANT["approval-1"].repository,"sonarqube-agent",
             frozenset({"view_existing_pr"}),NOW-timedelta(hours=1),True)}
        self.assertIn("approval_expired",evaluate_marketplace_agent(proposed(),grants=past,now=NOW).reasons)

    def test_secrets_denied(self):
        p=proposed();p["secrets_in_prompt"]=True
        self.assertIn("prompt_secrets_forbidden",evaluate_marketplace_agent(p,grants=GRANT,now=NOW).reasons)

    def test_data_export_denied(self):
        p=proposed();p["data_export_allowed"]=True
        self.assertIn("unapproved_data_export",evaluate_marketplace_agent(p,grants=GRANT,now=NOW).reasons)

    def test_local_scan_without_isolation_denied(self):
        g={"approval-1":OwnerGrant("mojealterego/Knowledge-projects",
              "sonarqube-agent",frozenset({"launch_local_scanner"}),NOW+timedelta(days=1),True)}
        p=proposed();p["capability"]="launch_local_scanner";p["explicit_effect_approval"]=True
        self.assertIn("scanner_must_target_owned_isolated_lab",evaluate_marketplace_agent(p,grants=g,now=NOW).reasons)

    def test_scanner_with_isolation_and_owner_grant(self):
        g={"approval-1":OwnerGrant("mojealterego/Knowledge-projects",
              "sonarqube-agent",frozenset({"launch_local_scanner"}),NOW+timedelta(days=1),True)}
        p=proposed();p["capability"]="launch_local_scanner";p["explicit_effect_approval"]=True;p["isolated_test_target"]=True
        self.assertTrue(evaluate_marketplace_agent(p,grants=g,now=NOW).accepted)

if __name__ == "__main__":
    unittest.main()
