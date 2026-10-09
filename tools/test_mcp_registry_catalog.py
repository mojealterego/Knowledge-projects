"""Offline stdlib tests of seven-page pinned MCP registry snapshot.

No external registry installation, OAuth, network or credential access.
"""
import copy
from datetime import datetime, timezone
from pathlib import Path
import unittest

from mcp_registry_catalog import (
    CatalogError, TARGET_REPOSITORY, classify_listing, evaluate_integration,
    load_snapshot, validate_snapshot,
)

SNAPSHOT = (Path(__file__).resolve().parents[1] / "docs" / "mcp-registry" /
            "2026-10-09-github-mcp-pages-1-7.json")
NOW = datetime(2026, 10, 9, 12, tzinfo=timezone.utc)


class McpRegistrySnapshotTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.snapshot, cls.summary = load_snapshot(SNAPSHOT)
        cls.url = cls.snapshot["source_card_entries"][0]["registry_url"]

    def mutated(self):
        return copy.deepcopy(self.snapshot)

    def test_exact_seven_pages(self):
        self.assertEqual(self.summary.pages, 7)

    def test_exactly_210_visible_cards(self):
        self.assertEqual(self.summary.observed_cards, 210)

    def test_live_snapshot_advertised_total_not_claimed_fully_read(self):
        self.assertEqual(self.summary.advertised_registry_total, 394)
        self.assertEqual(self.summary.unreviewed_by_this_snapshot, 184)

    def test_gaps_in_extracted_titles_preserved(self):
        self.assertEqual(self.summary.missing_titles, 2)

    def test_each_card_has_unique_registry_url(self):
        urls = [x["registry_url"] for x in self.snapshot["source_card_entries"]]
        self.assertEqual(len(urls), len(set(urls)))

    def test_heuristic_categories_are_not_truth_claims(self):
        self.assertEqual(classify_listing("Security scanner", "vulnerability scan"), "security")
        self.assertEqual(classify_listing("Memory", "persistent agent memory"), "knowledge_memory")

    def test_rejects_duplicate_entry(self):
        d = self.mutated()
        d["source_card_entries"][1]["registry_url"] = d["source_card_entries"][0]["registry_url"]
        with self.assertRaisesRegex(CatalogError, "duplicate_registry_url"):
            validate_snapshot(d)

    def test_rejects_unverified_outbound_link(self):
        d = self.mutated()
        d["source_card_entries"][0]["registry_url"] = "https://github.com/attacker/other"
        with self.assertRaisesRegex(CatalogError, "invalid_registry_url"):
            validate_snapshot(d)

    def test_rejects_missing_description(self):
        d = self.mutated()
        d["source_card_entries"][0]["description"] = ""
        with self.assertRaisesRegex(CatalogError, "missing_description"):
            validate_snapshot(d)

    def test_rejects_unsupported_claim_that_tools_were_read(self):
        d = self.mutated()
        d["source_card_entries"][0]["tools_list_verified"] = True
        with self.assertRaisesRegex(CatalogError, "runtime_evidence_not_observed"):
            validate_snapshot(d)

    def test_rejects_out_of_scope_page(self):
        d = self.mutated()
        d["source_pages"][0]["source_url"] = "https://example.com/"
        with self.assertRaisesRegex(CatalogError, "invalid_page_url"):
            validate_snapshot(d)

    def test_rejects_invented_missing_title_without_origin_marker(self):
        d = self.mutated()
        d["source_card_entries"][0]["title"] = None
        with self.assertRaisesRegex(CatalogError, "title_missing_without_provenance"):
            validate_snapshot(d)

    def call(self, changes=None, grants=None):
        p = dict(registry_url=self.url, action="catalog_only",
                 target_repository=TARGET_REPOSITORY)
        p.update(changes or {})
        return evaluate_integration(p, catalog=self.snapshot,
                                    independent_grants=grants or {}, now=NOW)

    def test_offline_catalog_research_allowed(self):
        self.assertTrue(self.call().allowed)

    def test_external_repo_write_denied(self):
        d = self.call({"target_repository":"mojealterego/ODYN-AI"})
        self.assertIn("write_scope_outside_knowledge_projects", d.reasons)

    def test_unlisted_server_denied(self):
        d = self.call({"registry_url":"https://github.com/mcp/not-real/none"})
        self.assertIn("not_in_reviewed_catalog", d.reasons)

    def test_install_denied_even_with_plausible_listing(self):
        d = self.call({"action":"install"})
        self.assertIn("platform_install_or_payment_not_supported", d.reasons)
        self.assertIn("no_independent_permission_for_action", d.reasons)

    def test_connect_requires_real_tools_list(self):
        d = self.call({"action":"connect"})
        self.assertIn("tools_list_not_verified", d.reasons)

    def test_install_metadata_cannot_become_install_permission(self):
        d = self.call({
            "action":"install",
            "verified_manifest_sha256":"a"*64,
            "issuer_provenance_verified":True,
            "actual_tools_list_verified":True,
            "independent_owner_grant_id":"independent",
            "explicit_user_connection_approval":True,
            "secrets_in_prompt":False,
            "external_side_effects":False,
        }, grants={"independent":frozenset({"install"})})
        self.assertFalse(d.allowed)
        self.assertIn("platform_install_or_payment_not_supported", d.reasons)

    def test_no_unapproved_paid_action(self):
        self.assertFalse(self.call({"action":"make_payment"}).allowed)

    def test_rejects_secret_fields_on_effect(self):
        d = self.call({"action":"invoke_tool",
                       "verified_manifest_sha256":"a"*64,
                       "issuer_provenance_verified":True,
                       "actual_tools_list_verified":True,
                       "independent_owner_grant_id":"grant",
                       "explicit_user_connection_approval":True,
                       "secrets_in_prompt":True,
                       "external_side_effects":False},
                      grants={"grant":frozenset({"invoke_tool"})})
        self.assertIn("secrets_in_prompt_forbidden", d.reasons)

    def test_refuses_unknown_actions(self):
        self.assertIn("unknown_action", self.call({"action":"shell_anywhere"}).reasons)

    def test_no_sdk_or_network_client_imports(self):
        import mcp_registry_catalog as m
        self.assertNotIn("requests", m.__dict__)
        self.assertNotIn("subprocess", m.__dict__)


if __name__=="__main__":
    unittest.main()
