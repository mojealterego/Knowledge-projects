"""Tests the P126 offline adapter from its owning project directory."""
import importlib.util
from pathlib import Path
import sys

file = Path(__file__).resolve().parents[1] / "projekty" / "126-ibm-cloud-cognitive-game-backend-reference" / "state_reference.py"
spec = importlib.util.spec_from_file_location("ibm_game_state_reference", file)
module = importlib.util.module_from_spec(spec)
sys.modules[spec.name] = module
spec.loader.exec_module(module)
from ibm_game_state_reference import InMemoryGameAdapter, ConflictError, UnauthorizedAction, InvalidIntent

import unittest


class GameStateReferenceTests(unittest.TestCase):
    def setUp(self):
        self.store = InMemoryGameAdapter()

    def apply(self, **changes):
        params = dict(authenticated_player="alice", player_id="alice", expected_revision=0,
                      event_id="event-1", intent="earn_xp", quantity=5)
        params.update(changes)
        return self.store.apply(**params)

    def test_first_event_commits_revision(self):
        self.assertEqual(self.apply().revision, 1)
        self.assertEqual(self.store.get("alice").xp, 5)

    def test_repeated_event_is_idempotent(self):
        first = self.apply()
        again = self.apply()
        self.assertEqual(again, first)
        self.assertEqual(self.store.get("alice").revision, 1)

    def test_same_event_different_command_denied(self):
        self.apply()
        with self.assertRaises(InvalidIntent):
            self.apply(quantity=6)

    def test_stale_revision_rejected(self):
        self.apply()
        with self.assertRaises(ConflictError):
            self.apply(event_id="event-2")

    def test_separate_players_isolated(self):
        self.apply()
        self.assertEqual(self.store.get("bob").xp, 0)
        with self.assertRaises(UnauthorizedAction):
            self.apply(player_id="bob")

    def test_insufficient_energy_rejected(self):
        with self.assertRaises(InvalidIntent):
            self.apply(intent="spend_energy", quantity=11)
        self.assertEqual(self.store.get("alice").revision, 0)

    def test_valid_energy_spend(self):
        state = self.apply(intent="spend_energy", quantity=3)
        self.assertEqual(state.energy, 7)

    def test_oversize_reward_rejected(self):
        with self.assertRaises(InvalidIntent):
            self.apply(quantity=10**6)

    def test_negative_and_boolean_not_valid(self):
        for amount in (-3, 0, True):
            with self.subTest(amount=amount):
                with self.assertRaises(InvalidIntent):
                    self.apply(quantity=amount)

    def test_cannot_set_final_balance_as_intent(self):
        with self.assertRaises(InvalidIntent):
            self.apply(intent="set_balance")

    def test_consistent_sequence_after_refetch(self):
        first = self.apply()
        second = self.apply(expected_revision=first.revision, event_id="event-2")
        self.assertEqual(second.xp, 10)
        self.assertEqual(second.revision, 2)


if __name__ == "__main__":
    unittest.main()

