import unittest
from osint_provenance_gate import Origin, classify_http_username_status, demo_record, make_record


class ProvenanceGateTests(unittest.TestCase):
    def setUp(self):
        self.full = dict(claim="DNS record returned an address", scope_id="signed-case-123",
                         source_ref="res:primary", observed_at="2026-10-08T17:00:00Z",
                         source_sha256="a" * 64)

    def test_simulated_is_labeled_and_never_fake_verified(self):
        item = demo_record("random network edge")
        self.assertEqual(item.origin, Origin.SIMULATED)
        self.assertIn("SYMULOWANE", item.export()["ui_badge"])
        self.assertIsNone(item.verifier_ref)

    def test_observed_requires_scope_and_trace(self):
        with self.assertRaises(ValueError):
            make_record(claim="IP found", origin="observed")
        self.assertEqual(make_record(origin="observed", **self.full).origin, Origin.OBSERVED)

    def test_verification_needs_verifier(self):
        with self.assertRaises(ValueError):
            make_record(origin="verified", **self.full)
        verified = make_record(origin="verified", verifier_ref="review:human-456", **self.full)
        self.assertIn("WERYFIKACJA", verified.ui_badge)

    def test_unknown_origin_rejected(self):
        with self.assertRaises(ValueError):
            make_record(claim="x", origin="real")

    def test_hash_must_be_valid(self):
        with self.assertRaises(ValueError):
            make_record(origin="observed", **{**self.full, "source_sha256": "bad"})

    def test_http_200_never_claims_username_found(self):
        for code in (200, 301, 403, 404, 429, 500):
            with self.subTest(code=code):
                self.assertEqual(classify_http_username_status(code), "inconclusive_http_response")

    def test_invalid_http_status_rejected(self):
        for code in (True, -1, 600, "200"):
            with self.subTest(code=code):
                with self.assertRaises(ValueError):
                    classify_http_username_status(code)

    def test_no_network_module_imported(self):
        import osint_provenance_gate
        self.assertNotIn("requests", osint_provenance_gate.__dict__)
        self.assertNotIn("socket", osint_provenance_gate.__dict__)


if __name__ == "__main__":
    unittest.main()
