"""Deterministic *offline* state adapter inspired by IBM Cloudant _rev OCC.

NOT a Cloudant client, production wallet, auth system, or IBM deployment.
Intents, not client-specified final balances, produce server-owned changes.
"""
from __future__ import annotations
from dataclasses import dataclass, replace
from threading import RLock


class ConflictError(Exception):
    """A stale revision must be fetched again; do not overwrite new state."""


class UnauthorizedAction(Exception):
    pass


class InvalidIntent(Exception):
    pass


@dataclass(frozen=True)
class PlayerState:
    player_id: str
    revision: int = 0
    xp: int = 0
    energy: int = 10


class InMemoryGameAdapter:
    """Revision-checked operations; one in-process lock only.

    This serves as an executable reference test. A production cloud app
    must use Cloudant conditional _rev writes and persistent idempotency.
    """
    def __init__(self):
        self._lock = RLock()
        self._players: dict[str, PlayerState] = {}
        self._idempotency: dict[tuple[str, str], tuple[tuple, PlayerState]] = {}

    def get(self, player_id: str) -> PlayerState:
        with self._lock:
            return self._players.get(player_id, PlayerState(player_id))

    def apply(
        self,
        *,
        authenticated_player: str,
        player_id: str,
        expected_revision: int,
        event_id: str,
        intent: str,
        quantity: int,
    ) -> PlayerState:
        if not authenticated_player or authenticated_player != player_id:
            raise UnauthorizedAction("authenticated actor must own state")
        if not isinstance(event_id, str) or not event_id.strip():
            raise InvalidIntent("stable event id required")
        if type(expected_revision) is not int or expected_revision < 0:
            raise InvalidIntent("invalid expected revision")
        if type(quantity) is not int or not (1 <= quantity <= 100):
            raise InvalidIntent("quantity out of range")
        if intent not in ("earn_xp", "spend_energy"):
            raise InvalidIntent("unknown intent")
        signature = (player_id, intent, quantity)
        with self._lock:
            key = (player_id, event_id)
            seen = self._idempotency.get(key)
            if seen is not None:
                if seen[0] != signature:
                    raise InvalidIntent("event-id reuse with changed intent")
                return seen[1]
            before = self._players.get(player_id, PlayerState(player_id))
            if expected_revision != before.revision:
                raise ConflictError("stale revision, refetch and recompute")
            if intent == "spend_energy":
                if before.energy < quantity:
                    raise InvalidIntent("insufficient energy")
                after = replace(before, revision=before.revision + 1, energy=before.energy - quantity)
            else:
                after = replace(before, revision=before.revision + 1, xp=before.xp + quantity)
            self._players[player_id] = after
            self._idempotency[key] = (signature, after)
            return after
