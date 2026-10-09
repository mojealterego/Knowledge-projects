"""Offline proof-of-authorization gate for defensive OSINT evidence records.

NO network access, device tracking, contact enumeration, social scraping,
SS7/HLR interrogation, IMEI lookups, BLE scans or location inference.
Input must be a de-identified record from a separately authorized workflow.
"""
from __future__ import annotations
from dataclasses import dataclass
from datetime import datetime
import re

_SHA = re.compile(r"^[a-f0-9]{64}$")
_ALLOWED_CASES = frozenset({"own_device_incident", "consenting_subject", "public_entity_threat_intel"})
_ALLOWED_EVIDENCE = frozenset({"user_supplied_photo_exif", "owned_device_inventory",
                              "public_advisory", "owner_provided_log"})
_BLOCKED_ACTIONS = frozenset({"location_tracking", "contact_enumeration",
                              "private_account_probe", "ss7_hlr_query",
                              "remote_device_scan", "secret_access"})
@dataclass(frozen=True)
class Assessment:
    admissible: bool
    reasons: tuple[str,...]
    physical_location_verified: bool = False

def assess(case: dict, *, now: datetime, verified_scope_ids: frozenset[str]) -> Assessment:
    if not isinstance(case, dict):
        raise TypeError("case must be dict")
    if not isinstance(now, datetime) or now.tzinfo is None:
        raise ValueError("now must be timezone-aware")
    errors=[]
    if case.get("case_type") not in _ALLOWED_CASES: errors.append("unapproved_case_type")
    if case.get("source_kind") not in _ALLOWED_EVIDENCE: errors.append("source_requires_new_review")
    scope=case.get("authorization_scope_id")
    if not isinstance(scope,str) or scope not in verified_scope_ids: errors.append("authorization_not_verified")
    if case.get("owner_consent") is not True: errors.append("consent_not_verified")
    digest=case.get("source_sha256")
    if not isinstance(digest,str) or not _SHA.fullmatch(digest): errors.append("invalid_source_hash")
    if case.get("network_access") is not False: errors.append("network_access_forbidden")
    if case.get("contains_personal_identifiers") is not False: errors.append("private_identifiers_must_be_redacted")
    if case.get("action") in _BLOCKED_ACTIONS: errors.append("surveillance_or_intrusive_action_forbidden")
    if case.get("action") not in ("record_metadata", "review_source_provenance"):
        errors.append("action_not_in_safe_review_scope")
    if case.get("claims_live_device_location") is not False:
        errors.append("location_cannot_be_inferred_from_metadata")
    if case.get("physical_geolocation_from_mac_or_imei") is True:
        errors.append("mac_imei_not_location_proof")
    return Assessment(not errors,tuple(dict.fromkeys(errors)))
