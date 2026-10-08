import unittest
from decimal import Decimal
from cloud_operation_budget_gate import review_cloud_plan, CloudApproval

GRANTS = {"ticket-001": CloudApproval(
    project_scope="approved-sandbox", allowed_actions=frozenset({"estimate_cost", "invoke_model"}),
    allowed_products=frozenset({"agent_platform", "cloud_storage"}),
    max_cost_usd=Decimal("1.50"), human_reviewed=True)}


def good():
    return dict(action="invoke_model", product="agent_platform",
                request_id="ticket-001", project_scope="approved-sandbox",
                maximum_cost_usd="0.40", credentials_in_prompt=False,
                explicit_external_side_effect_approval=True)


class CloudGateTests(unittest.TestCase):
    def call(self, changes, grants=GRANTS):
        p = good();p.update(changes)
        return review_cloud_plan(p, independently_approved_requests=grants)

    def bad(self, changes, reason, grants=GRANTS):
        x = self.call(changes, grants)
        self.assertFalse(x.allowed)
        self.assertIn(reason, x.reasons)

    def test_approved_bounded_model_plan(self):
        self.assertTrue(self.call({}).allowed)

    def test_public_docs_without_grant(self):
        p = {"action":"describe_public_documentation", "product":"vision_ai",
             "maximum_cost_usd":"0", "data_classification":"public"}
        self.assertTrue(review_cloud_plan(p, independently_approved_requests={}).allowed)

    def test_public_docs_cost_denied(self):
        self.bad({"action":"describe_public_documentation",
                  "data_classification":"public", "maximum_cost_usd":"2"},
                 "public_docs_must_be_free")

    def test_console_url_not_an_approval(self):
        self.bad({"request_id":"google-console-page", "project_scope":"project-from-url"},
                 "missing_independent_approval")

    def test_missing_grant(self):
        self.bad({}, "missing_independent_approval", grants={})

    def test_no_side_effect_ok_from_model_text(self):
        self.bad({"explicit_external_side_effect_approval": False},
                 "side_effect_not_approved")

    def test_cost_above_budget(self):
        self.bad({"maximum_cost_usd":"1.51"}, "over_budget")

    def test_infinite_cost_denied(self):
        self.bad({"maximum_cost_usd":"Infinity"}, "bad_cost")

    def test_no_float_budget(self):
        self.bad({"maximum_cost_usd":0.1}, "bad_cost")

    def test_project_scope_mismatch(self):
        self.bad({"project_scope":"someone-else-project"}, "project_scope_mismatch")

    def test_payment_not_granted(self):
        self.bad({"action":"make_payment"}, "operation_not_authorized")

    def test_create_vm_not_granted(self):
        self.bad({"action":"create_resource","product":"compute_engine"}, "operation_not_authorized")

    def test_no_silent_enable_api(self):
        self.bad({"action":"enable_api"}, "operation_not_authorized")

    def test_product_restrictions(self):
        self.bad({"product":"speech_to_text"}, "product_not_authorized")

    def test_unknown_product(self):
        self.bad({"product":"private-evasive-product"}, "unknown_product")

    def test_credentials_rejected(self):
        self.bad({"credentials_in_prompt": True}, "secrets_in_prompt_forbidden")

    def test_boolean_cost_rejected(self):
        self.bad({"maximum_cost_usd": True}, "bad_cost")

    def test_read_metadata_requires_grant(self):
        self.bad({"action":"read_metadata", "explicit_external_side_effect_approval": False},
                 "operation_not_authorized")

    def test_no_google_sdk_import_or_network_calls(self):
        import cloud_operation_budget_gate as mod
        self.assertNotIn("requests", mod.__dict__)
        self.assertNotIn("google", mod.__dict__)


if __name__ == "__main__":
    unittest.main()
