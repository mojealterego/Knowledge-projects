import importlib.util
from pathlib import Path
import sys
path = Path(__file__).resolve().parents[1] / "projekty" / "122-chemia-consent-aware-intimate-two-player-game" / "consent_intersection.py"
spec=importlib.util.spec_from_file_location("consent_intersection",path)
module=importlib.util.module_from_spec(spec)
sys.modules[spec.name]=module
spec.loader.exec_module(module)
import unittest
from consent_intersection import PlayerChoices, intersect_choices

def p(*, agreed=frozenset(), blocked=frozenset(), active=True, adult=True, session="pair-1"):
    return PlayerChoices(session,adult,active,agreed,blocked)

class MutualChoicesTests(unittest.TestCase):
    def test_only_intersection_is_returned(self):
        result=intersect_choices(p(agreed=frozenset({"talk","dance","memory"})),
                                 p(agreed=frozenset({"talk","walk","dance"})))
        self.assertEqual(result.shared_topic_ids,("dance","talk"))
        self.assertNotIn("memory",result.shared_topic_ids)
        self.assertNotIn("walk",result.shared_topic_ids)

    def test_zero_overlap_is_valid(self):
        self.assertEqual(intersect_choices(p(agreed=frozenset({"walk"})),
                                           p(agreed=frozenset({"dance"}))).shared_topic_ids,())

    def test_block_list_takes_priority(self):
        x=intersect_choices(p(agreed=frozenset({"dance"}),blocked=frozenset({"dance"})),
                            p(agreed=frozenset({"dance"})))
        self.assertEqual(x.shared_topic_ids,())

    def test_either_person_can_block(self):
        x=intersect_choices(p(agreed=frozenset({"dance"})),
                            p(agreed=frozenset({"dance"}),blocked=frozenset({"dance"})))
        self.assertEqual(x.shared_topic_ids,())

    def test_revocation_removes_results(self):
        x=intersect_choices(p(agreed=frozenset({"talk"}),active=False),
                            p(agreed=frozenset({"talk"})))
        self.assertEqual(x.status,"inactive_or_revoked")
        self.assertEqual(x.shared_topic_ids,())

    def test_both_need_active_participation(self):
        x=intersect_choices(p(agreed=frozenset({"talk"})),
                            p(agreed=frozenset({"talk"}),active=False))
        self.assertEqual(x.shared_topic_ids,())

    def test_both_self_attest_adulthood(self):
        x=intersect_choices(p(agreed=frozenset({"talk"}),adult=False),
                            p(agreed=frozenset({"talk"})))
        self.assertEqual(x.status,"adult_attestation_required")
        self.assertEqual(x.shared_topic_ids,())

    def test_mismatched_sessions_blocked(self):
        x=intersect_choices(p(agreed=frozenset({"talk"}),session="a"),
                            p(agreed=frozenset({"talk"}),session="b"))
        self.assertEqual(x.status,"session_mismatch")

    def test_order_independent(self):
        a=p(agreed=frozenset({"walk","talk"}))
        b=p(agreed=frozenset({"talk"}))
        self.assertEqual(intersect_choices(a,b),intersect_choices(b,a))

    def test_bad_topic_does_not_leak(self):
        with self.assertRaises(ValueError):
            intersect_choices(p(agreed=frozenset({""})),p())

    def test_wrong_input_type(self):
        with self.assertRaises(TypeError):
            intersect_choices({},p())

    def test_no_global_profile_or_network(self):
        import consent_intersection
        self.assertNotIn("requests",consent_intersection.__dict__)
        self.assertNotIn("sqlite3",consent_intersection.__dict__)

if __name__ == "__main__":
    unittest.main()
