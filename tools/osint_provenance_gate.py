"""Pure-data OSINT evidence gate. No discovery, network calls, or person profiling.

A source-generated or demo result must not be promoted to observed/verified.
Recorded provenance is a necessary condition for review, never independent proof.
"""
from __future__ import annotations

from dataclasses import asdict, dataclass
from enum import Enum
from typing import Any


class Origin(str, Enum):
    SIMULATED = "simulated"
    OBSERVED = "observed"
    VERIFIED = "verified"


@dataclass(frozen=True)
class EvidenceRecord:
    claim: str
    origin: Origin
    scope_id: str | None
    source_ref: str | None
    observed_at: str | None
    verifier_ref: str | None
    source_sha256: str | None
    ui_badge: str

    def export(self) -> dict[str, Any]:
        return {
            "claim": self.claim,
            "origin": self.origin.value,
            "scope_id": self.scope_id,
            "source_ref": self.source_ref,
            "observed_at": self.observed_at,
            "verifier_ref": self.verifier_ref,
            "source_sha256": self.source_sha256,
            "ui_badge": self.ui_badge,
        }


def make_record(*, claim: str, origin: str,
                scope_id: str | None = None, source_ref: str | None = None,
                observed_at: str | None = None, verifier_ref: str | None = None,
                source_sha256: str | None = None) -> EvidenceRecord:
    if not isinstance(claim, str) or not claim.strip():
        raise ValueError("claim must be nonempty text")
    try:
        state = Origin(origin)
    except (ValueError, TypeError):
        raise ValueError("unknown evidence origin") from None
    if source_sha256 is not None:
        if not isinstance(source_sha256, str) or len(source_sha256) != 64 or any(c not in "0123456789abcdef" for c in source_sha256):
            raise ValueError("source_sha256 must be a lowercase 64-character hex digest")
    if state is not Origin.SIMULATED:
        if not all(isinstance(x, str) and x.strip() for x in (scope_id, source_ref, observed_at, source_sha256)):
            raise ValueError("non-simulated claims require scope, source, timestamp and content hash")
    if state is Origin.VERIFIED and not (isinstance(verifier_ref, str) and verifier_ref.strip()):
        raise ValueError("verified claim requires an independent verifier reference")
    badge = {
        Origin.SIMULATED: "SYMULOWANE — brak ustaleń faktycznych",
        Origin.OBSERVED: "ZAOBSERWOWANE — niezweryfikowane",
        Origin.VERIFIED: "WERYFIKACJA UDOKUMENTOWANA — sprawdź dowody",
    }[state]
    return EvidenceRecord(claim=claim.strip(), origin=state, scope_id=scope_id,
                          source_ref=source_ref, observed_at=observed_at,
                          verifier_ref=verifier_ref, source_sha256=source_sha256,
                          ui_badge=badge)


def classify_http_username_status(status: int) -> str:
    """HTTP status is NOT proof that an online account exists or belongs to a person."""
    if not isinstance(status, int) or isinstance(status, bool) or not 100 <= status <= 599:
        raise ValueError("invalid HTTP status")
    return "inconclusive_http_response"


def demo_record(claim: str) -> EvidenceRecord:
    """Build a clearly marked synthetic visualization item without network access."""
    return make_record(claim=claim, origin=Origin.SIMULATED.value)
