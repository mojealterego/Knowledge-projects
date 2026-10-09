"""Local planning matrix for communication protocols mentioned in Komunikacja.pdf.

Capabilities are conservative qualitative labels, not radio engineering
measurements, geolocation evidence, access grants or executable integrations.
"""
from __future__ import annotations
from dataclasses import dataclass

PROTOCOLS = {
    "irda": {"transport":"optical", "proximity":"line_of_sight", "pairing":"explicit"},
    "bluetooth_le": {"transport":"radio", "proximity":"nearby", "pairing":"explicit"},
    "nfc": {"transport":"radio", "proximity":"very_near", "pairing":"explicit"},
    "uwb": {"transport":"radio", "proximity":"nearby", "pairing":"explicit"},
    "wifi_direct": {"transport":"radio", "proximity":"local", "pairing":"explicit"},
    "lorawan": {"transport":"radio", "proximity":"long_range", "pairing":"provisioned"},
    "li_fi": {"transport":"optical", "proximity":"line_of_sight", "pairing":"explicit"},
    "zenoh": {"transport":"software_messaging", "proximity":"network_dependent", "pairing":"authenticated"},
}
PURPOSES = frozenset({"owned_device_transfer", "authorized_testbed", "offline_comparison"})

@dataclass(frozen=True)
class ProtocolReview:
    accepted: bool
    reasons: tuple[str, ...]
    has_physical_location_evidence: bool = False

def review_link(plan: dict) -> ProtocolReview:
    if not isinstance(plan, dict): raise TypeError("plan must be dict")
    reasons=[]
    if plan.get("protocol") not in PROTOCOLS: reasons.append("unknown_protocol")
    if plan.get("purpose") not in PURPOSES: reasons.append("unapproved_purpose")
    if plan.get("endpoint_owner_authorized") is not True: reasons.append("endpoint_owner_not_authorized")
    if plan.get("counterparty_opted_in") is not True: reasons.append("counterparty_consent_missing")
    if plan.get("passive_tracking") is not False: reasons.append("passive_tracking_forbidden")
    if plan.get("network_scan") is not False: reasons.append("network_scanning_forbidden")
    if plan.get("claims_global_device_location") is not False:
        reasons.append("protocol_does_not_prove_global_location")
    if plan.get("cloud_publication") is not False:
        reasons.append("external_data_transfer_requires_separate_approval")
    return ProtocolReview(not reasons,tuple(dict.fromkeys(reasons)))
