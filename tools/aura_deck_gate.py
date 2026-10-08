"""Read-only manifest validator for a proposed AURA physical deck.

Does not design covert suggestion, generate gameplay prompts, activate NFC, or
claim manufacturing feasibility. Source lists 60 cards, 4 domains and three
special-card classes totaling 10. Remaining domain allocation is unresolved.
"""
from __future__ import annotations
from collections import Counter

TOTAL_CARDS = 60
DOMAIN_COUNT = 4
ANOMALY_QUOTAS = {"mirror": 4, "venom": 4, "black_swan": 2}


class DeckValidationError(ValueError):
    pass


def validate_deck(manifest: dict) -> dict:
    """Fail-closed validation of count, provenance and consent-oriented design.

    Each item: id, kind ('domain'/'anomaly'), domain or anomaly,
    effect_disclosed (bool), covert_behavioral_targeting (bool),
    gambling_pressure (bool), optional nfc_requires_opt_in (bool).
    No hidden persuasion scripts or payloads are stored by this validator.
    """
    if not isinstance(manifest, dict):
        raise DeckValidationError("Expected an object")
    domains = manifest.get("domains")
    cards = manifest.get("cards")
    if not isinstance(domains, list) or len(domains) != DOMAIN_COUNT or any(
        not isinstance(d, str) or not d.strip() for d in domains
    ) or len(set(domains)) != DOMAIN_COUNT:
        raise DeckValidationError("Four distinct named domains are required")
    if not isinstance(cards, list) or len(cards) != TOTAL_CARDS:
        raise DeckValidationError("Exactly 60 card instances required")

    ids = set()
    special = Counter()
    ordinary = Counter()
    for i, card in enumerate(cards):
        if not isinstance(card, dict):
            raise DeckValidationError(f"Card {i}: expected object")
        cid = card.get("id")
        if not isinstance(cid, str) or not cid.strip() or cid in ids:
            raise DeckValidationError(f"Card {i}: invalid or duplicate id")
        ids.add(cid)
        if card.get("effect_disclosed") is not True:
            raise DeckValidationError(f"Card {cid}: reveal/game effects must be disclosed")
        if card.get("covert_behavioral_targeting") is not False:
            raise DeckValidationError(f"Card {cid}: covert influence not permitted")
        if card.get("gambling_pressure") is not False:
            raise DeckValidationError(f"Card {cid}: gambling-pressure effects not permitted")
        if card.get("nfc_requires_opt_in", True) is not True:
            raise DeckValidationError(f"Card {cid}: NFC must be opt-in")
        kind = card.get("kind")
        if kind == "domain":
            if card.get("domain") not in domains or card.get("anomaly") is not None:
                raise DeckValidationError(f"Card {cid}: invalid domain card")
            ordinary[card["domain"]] += 1
        elif kind == "anomaly":
            anomaly = card.get("anomaly")
            if anomaly not in ANOMALY_QUOTAS or card.get("domain") is not None:
                raise DeckValidationError(f"Card {cid}: invalid anomaly")
            special[anomaly] += 1
        else:
            raise DeckValidationError(f"Card {cid}: unknown kind")

    if dict(special) != ANOMALY_QUOTAS:
        raise DeckValidationError("Special cards must be mirror=4, venom=4, black_swan=2")
    if any(ordinary[d] == 0 for d in domains):
        raise DeckValidationError("Every domain must have an assigned card")
    return {
        "valid": True,
        "total_cards": len(cards),
        "domain_counts": dict(ordinary),
        "anomaly_counts": dict(special),
        "manufacturing_validated": False,
        "behavioral_effectiveness_validated": False,
    }
