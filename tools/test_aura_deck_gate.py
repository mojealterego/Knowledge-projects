import copy
import unittest
from aura_deck_gate import DeckValidationError, validate_deck


def make_complete_demo():
    """Synthetic completeness fixture, NOT a canonical AURA deck or missing rule data."""
    names = ["instinct", "void", "UNSPECIFIED_A", "UNSPECIFIED_B"]
    cards = [
        dict(id=f"domain-{i}", kind="domain", domain=names[i % 4],
             effect_disclosed=True, covert_behavioral_targeting=False,
             gambling_pressure=False, nfc_requires_opt_in=True)
        for i in range(50)
    ]
    idx = 0
    for anomaly, n in (("mirror", 4), ("venom", 4), ("black_swan", 2)):
        for _ in range(n):
            cards.append(dict(id=f"special-{idx}", kind="anomaly", anomaly=anomaly,
                              effect_disclosed=True, covert_behavioral_targeting=False,
                              gambling_pressure=False))
            idx += 1
    return dict(domains=names, cards=cards)


class AuraDeckTests(unittest.TestCase):
    def test_complete_synthetic_fixture_passes(self):
        result = validate_deck(make_complete_demo())
        self.assertTrue(result["valid"])
        self.assertEqual(result["total_cards"], 60)
        self.assertFalse(result["manufacturing_validated"])

    def test_missing_domain_rejected(self):
        case = make_complete_demo()
        case["domains"].pop()
        with self.assertRaises(DeckValidationError):
            validate_deck(case)

    def test_unresolved_30_cards_fail_closed(self):
        case = make_complete_demo()
        case["cards"] = case["cards"][:30]
        with self.assertRaises(DeckValidationError):
            validate_deck(case)

    def test_duplicate_id_rejected(self):
        case = make_complete_demo()
        case["cards"][1]["id"] = case["cards"][0]["id"]
        with self.assertRaises(DeckValidationError):
            validate_deck(case)

    def test_anomaly_quota_mismatch_rejected(self):
        case = make_complete_demo()
        case["cards"][-1]["anomaly"] = "mirror"
        with self.assertRaises(DeckValidationError):
            validate_deck(case)

    def test_covert_behavioral_targeting_rejected(self):
        case = make_complete_demo()
        case["cards"][0]["covert_behavioral_targeting"] = True
        with self.assertRaises(DeckValidationError):
            validate_deck(case)

    def test_gambling_pressure_rejected(self):
        case = make_complete_demo()
        case["cards"][1]["gambling_pressure"] = True
        with self.assertRaises(DeckValidationError):
            validate_deck(case)

    def test_undisclosed_effect_rejected(self):
        case = make_complete_demo()
        case["cards"][2]["effect_disclosed"] = False
        with self.assertRaises(DeckValidationError):
            validate_deck(case)

    def test_nfc_opt_in_mandatory(self):
        case = make_complete_demo()
        case["cards"][3]["nfc_requires_opt_in"] = False
        with self.assertRaises(DeckValidationError):
            validate_deck(case)

    def test_invalid_anomaly_label_rejected(self):
        case = make_complete_demo()
        case["cards"][-1]["anomaly"] = "unknown"
        with self.assertRaises(DeckValidationError):
            validate_deck(case)

    def test_card_domain_unknown_rejected(self):
        case = make_complete_demo()
        case["cards"][0]["domain"] = "hidden"
        with self.assertRaises(DeckValidationError):
            validate_deck(case)


if __name__ == "__main__":
    unittest.main()
