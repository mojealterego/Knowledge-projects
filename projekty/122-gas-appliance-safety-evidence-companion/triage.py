"""Read-only, evidence-bounded reference for historical Immergas VICTRIX PLUS/S codes.

This module does not operate the boiler, diagnose its condition, or prescribe repairs.
"""
from __future__ import annotations

from dataclasses import asdict, dataclass
from typing import Literal

SOURCE = "vitrix_20_plus_s_instrukcja.pdf; cover: VICTRIX PLUS/S; manufacturer manual"
KNOWN_CODES: dict[str, str] = {
    "01": "blokada zapłonu",
    "02": "blokada przed przegrzaniem",
    "05": "uszkodzona sonda zasilania centralnego ogrzewania",
    "10": "brak interwencji presostatu wody",
    "12": "uszkodzona lub niepodłączona sonda zasobnika",
    "14": "uszkodzona centralka kontroli zapłonu",
    "16": "uszkodzony wentylator",
    "17": "niewłaściwa liczba obrotów wentylatora",
    "26": "uszkodzony presostat wody",
    "31": "niewłaściwy sterownik pokojowy",
}

@dataclass(frozen=True)
class ReferenceFinding:
    code: str
    known: bool
    manual_description_pl: str | None
    action: Literal["refer_to_authorized_service", "verify_with_manual_and_service"]
    source: str
    automated_control_allowed: bool = False
    verified_on_device: bool = False

    def to_dict(self) -> dict[str, object]:
        return asdict(self)


def lookup(code: str) -> ReferenceFinding:
    if not isinstance(code, str):
        raise TypeError("Fault code must be text")
    normalized = code.strip()
    if not normalized or not normalized.isascii() or not normalized.isdigit() or len(normalized) > 2:
        raise ValueError("Expected a numeric fault code (1-2 ASCII digits)")
    normalized = normalized.zfill(2)
    if normalized in KNOWN_CODES:
        return ReferenceFinding(
            code=normalized, known=True, manual_description_pl=KNOWN_CODES[normalized],
            action="refer_to_authorized_service", source=SOURCE,
        )
    return ReferenceFinding(
        code=normalized, known=False, manual_description_pl=None,
        action="verify_with_manual_and_service", source=SOURCE,
    )
