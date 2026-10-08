"""Ephemeral mutual-choice calculation for two consenting adult players.

Does not store input, create accounts, perform age verification, or
communicate with the other person. Do not use returned intersection as
real-world consent; it is a voluntary game preference only.
"""
from __future__ import annotations
from dataclasses import dataclass

@dataclass(frozen=True)
class PlayerChoices:
    session_id: str
    adult_self_attested: bool
    participates_now: bool
    accepted_topics: frozenset[str]
    excluded_topics: frozenset[str]

@dataclass(frozen=True)
class MutualChoice:
    status: str
    shared_topic_ids: tuple[str, ...]

def intersect_choices(a: PlayerChoices, b: PlayerChoices) -> MutualChoice:
    """Return ONLY mutually accepted topic IDs. No unilateral choice leak."""
    if not isinstance(a, PlayerChoices) or not isinstance(b, PlayerChoices):
        raise TypeError("both players must provide typed choices")
    if not a.session_id or a.session_id != b.session_id:
        return MutualChoice("session_mismatch", ())
    if a.adult_self_attested is not True or b.adult_self_attested is not True:
        return MutualChoice("adult_attestation_required", ())
    if a.participates_now is not True or b.participates_now is not True:
        return MutualChoice("inactive_or_revoked", ())
    fields=(a.accepted_topics,a.excluded_topics,b.accepted_topics,b.excluded_topics)
    if any(not isinstance(field,frozenset) for field in fields):
        raise TypeError("topics must be immutable sets")
    if any(not isinstance(t,str) or not t or len(t)>100 for f in fields for t in f):
        raise ValueError("topic ids must be nonempty bounded strings")
    common=(a.accepted_topics & b.accepted_topics) - a.excluded_topics - b.excluded_topics
    return MutualChoice("ready", tuple(sorted(common)))
