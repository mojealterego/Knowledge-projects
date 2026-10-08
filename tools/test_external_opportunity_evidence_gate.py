import unittest
from datetime import datetime,timezone,timedelta
from external_opportunity_evidence_gate import SourceObservation,qualify_source,TARGET_REPO

NOW=datetime(2026,10,9,17,0,tzinfo=timezone.utc)
ZED="https://github.com/zed-industries/zed/issues/65199"
RAIL="https://station.railway.com/questions/template-request-node-bb-b54455b9"
WP="https://wordpress.com/plugins/browse/paid/mojealteregopl.wordpress.com"

def obs(kind="zed_issue",url=ZED,status="open", verified=True, at=NOW):
    return SourceObservation(kind,url,status,at,verified)

def decide(o,action="prototype_in_knowledge_projects",write=TARGET_REPO):
    return qualify_source(o,now=NOW,action=action,write_target=write)

class SourceQualificationTests(unittest.TestCase):
    def test_fresh_open_zed_issue_local_prototype(self):
        self.assertTrue(decide(obs()).allowed)

    def test_fresh_railway_bounty_local_prototype(self):
        self.assertTrue(decide(obs("railway_template_bounty",RAIL)).allowed)

    def test_open_issue_not_verified_payout(self):
        self.assertFalse(decide(obs()).claimable_reward_verified)

    def test_zed_board_shell_not_task(self):
        r=decide(obs("zed_project_board","https://github.com/orgs/zed-industries/projects/74"))
        self.assertIn("board_or_catalog_is_not_specific_task",r.reasons)

    def test_railway_project_board_not_task(self):
        r=decide(obs("railway_project_board","https://github.com/orgs/railwayapp/projects/2"))
        self.assertIn("board_or_catalog_is_not_specific_task",r.reasons)

    def test_railway_solved_bounty_not_open(self):
        self.assertIn("no_verified_open_task",
            decide(obs("railway_template_bounty",RAIL,status="solved")).reasons)

    def test_zed_closed_issue_not_open(self):
        self.assertIn("no_verified_open_task",
            decide(obs(status="closed")).reasons)

    def test_wp_catalog_research_only(self):
        self.assertTrue(decide(obs("wordpress_paid_plugins",WP,status="login_required"),"research").allowed)

    def test_wp_plugin_buy_denied(self):
        self.assertIn("external_action_not_authorized",
            decide(obs("wordpress_paid_plugins",WP),"buy").reasons)

    def test_wp_catalog_not_local_task(self):
        self.assertIn("board_or_catalog_is_not_specific_task",
            decide(obs("wordpress_paid_plugins",WP)).reasons)

    def test_no_write_to_zed(self):
        self.assertIn("write_outside_knowledge_projects_forbidden",
            decide(obs(),write="zed-industries/zed").reasons)

    def test_no_write_to_railway(self):
        self.assertIn("write_outside_knowledge_projects_forbidden",
            decide(obs("railway_template_bounty",RAIL),write="railwayapp/templates").reasons)

    def test_stale_evidence(self):
        self.assertIn("stale_or_future_evidence",
            decide(obs(at=NOW-timedelta(days=20))).reasons)

    def test_no_independent_verification(self):
        self.assertIn("source_not_independently_verified",
            decide(obs(verified=False)).reasons)

    def test_fake_url_denied(self):
        self.assertIn("source_url_does_not_match_provider",
            decide(obs(url="https://evil.example/zed-industries/zed/issues/65199")).reasons)

    def test_arbitrary_github_issue_cannot_be_impersonated(self):
        self.assertIn("source_url_does_not_match_provider",
            decide(obs(url="https://github.com/other/zed/issues/65199")).reasons)

    def test_project_board_research_allowed(self):
        r=decide(obs("zed_project_board","https://github.com/orgs/zed-industries/projects/74",
                     "shell_only"),"research")
        self.assertTrue(r.allowed)

    def test_no_network_or_github_integration(self):
        import external_opportunity_evidence_gate as m
        self.assertNotIn("requests",m.__dict__)
        self.assertNotIn("subprocess",m.__dict__)

if __name__=="__main__":
    unittest.main()
