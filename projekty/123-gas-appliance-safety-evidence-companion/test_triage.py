import unittest
from triage import KNOWN_CODES, lookup


class BoilerManualReferenceTests(unittest.TestCase):
    def test_every_documented_fault_is_reference_only(self):
        for code, description in KNOWN_CODES.items():
            with self.subTest(code=code):
                finding = lookup(code)
                self.assertTrue(finding.known)
                self.assertEqual(finding.manual_description_pl, description)
                self.assertEqual(finding.action, "refer_to_authorized_service")
                self.assertFalse(finding.automated_control_allowed)
                self.assertFalse(finding.verified_on_device)

    def test_unknown_code_fails_closed(self):
        item = lookup("99")
        self.assertFalse(item.known)
        self.assertIsNone(item.manual_description_pl)
        self.assertEqual(item.action, "verify_with_manual_and_service")

    def test_normalization_does_not_change_safety(self):
        self.assertEqual(lookup(" 1 ").code, "01")
        self.assertEqual(lookup(" 01 ").code, "01")

    def test_invalid_codes_are_rejected(self):
        for value in ("", "a", "1.0", "123", "١", "-1", "00a"):
            with self.subTest(value=value):
                with self.assertRaises(ValueError):
                    lookup(value)
        with self.assertRaises(TypeError):
            lookup(1)  # type: ignore[arg-type]

    def test_output_provenance_is_present(self):
        for code in ("01", "99"):
            self.assertIn("VICTRIX PLUS/S", lookup(code).to_dict()["source"])


if __name__ == "__main__":
    unittest.main()
