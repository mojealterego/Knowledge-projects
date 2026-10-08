"""Static, non-executing admission gate for AI-proposed tool artifacts.

A prototype policy, not a sandbox, signing service or execution engine.
The approvals map MUST originate outside the model/proposed manifest.
"""
from __future__ import annotations

from dataclasses import dataclass
from datetime import datetime, timezone
from hashlib import sha256
from typing import Mapping

ALLOWED_CAPABILITIES = frozenset({"read_repo_metadata", "parse_document", "format_text", "unit_test_fixture"})
SANDBOX_PROFILE = "isolated_no_network_read_only"


@dataclass(frozen=True)
class AdmissionDecision:
    accepted: bool
    reasons: tuple[str, ...]
    artifact_sha256: str


def admit_tool(
    manifest: dict,
    artifact: bytes,
    *,
    trusted_approvals: Mapping[str, frozenset[str]],
    now: datetime,
) -> AdmissionDecision:
    """Inspect manifest and bytes without importing or executing the artifact.

    trusted_approvals maps artifact sha256 to externally reviewed capabilities.
    A manifest's self-declared reviewer or approval flag is never sufficient.
    """
    if not isinstance(artifact, bytes):
        raise TypeError("artifact must be bytes")
    if not isinstance(manifest, dict):
        raise TypeError("manifest must be an object")
    if not isinstance(now, datetime) or now.tzinfo is None:
        raise ValueError("now must be timezone-aware")
    digest = sha256(artifact).hexdigest()
    reasons: list[str] = []
    if not artifact:
        reasons.append("empty_artifact")
    if manifest.get("artifact_sha256") != digest:
        reasons.append("artifact_hash_mismatch")
    capability_list = manifest.get("capabilities")
    if not isinstance(capability_list, list) or not capability_list or not all(
        isinstance(x, str) for x in capability_list
    ):
        reasons.append("invalid_capabilities")
        caps = set()
    else:
        caps = set(capability_list)
        if len(caps) != len(capability_list):
            reasons.append("duplicate_capability")
        if not caps.issubset(ALLOWED_CAPABILITIES):
            reasons.append("privileged_or_unknown_capability")
    reviewed = trusted_approvals.get(digest)
    if reviewed is None:
        reasons.append("no_independent_approval")
    elif not caps.issubset(reviewed):
        reasons.append("outside_approved_capability_scope")
    if manifest.get("sandbox_profile") != SANDBOX_PROFILE:
        reasons.append("missing_isolated_read_only_sandbox")
    timeout = manifest.get("timeout_seconds")
    if type(timeout) is not int or not 1 <= timeout <= 60:
        reasons.append("invalid_timeout")
    expires = manifest.get("expires_at")
    try:
        if not isinstance(expires, str) or not expires.endswith("Z"):
            raise ValueError("must end in Z")
        end = datetime.fromisoformat(expires.replace("Z", "+00:00"))
        if end <= now.astimezone(timezone.utc):
            reasons.append("approval_expired")
    except (TypeError, ValueError):
        reasons.append("invalid_expiration")
    if manifest.get("external_side_effects") is not False:
        reasons.append("side_effects_not_disabled")
    return AdmissionDecision(not reasons, tuple(reasons), digest)
