"""Offline, non-executing four-stage DGM promotion-policy gate.

This module does NOT discover models, mutate code, invoke CI, sign releases or
merge PRs. Evidence is supplied by independently trusted caller, not LLM text.
"""
from __future__ import annotations

from dataclasses import dataclass
from datetime import datetime, timezone
import re
from typing import Mapping

_GIT_SHA = re.compile(r"^[a-f0-9]{40}$")
EXPECTED_PHASES = ("ModelScout", "TechRecon", "Strategist", "DGM_Core")


@dataclass(frozen=True)
class Decision:
    admitted: bool
    reasons: tuple[str, ...]


def assess_promotion(
    proposal: dict,
    *,
    verified_ci_by_candidate: Mapping[str, frozenset[str]],
    approved_candidates: frozenset[str],
    now: datetime,
) -> Decision:
    """Review candidate evidence, not source text promises or model self-approval.

    Restrict use to a non-default branch; real repository permissions and CI
    attestation verification MUST still be enforced by hosting platform.
    """
    if not isinstance(proposal, dict):
        raise TypeError("proposal must be a dict")
    if not isinstance(now, datetime) or now.tzinfo is None:
        raise ValueError("now must be timezone aware")
    errors = []
    candidate = proposal.get("candidate_sha")
    base_sha = proposal.get("base_sha")
    if not isinstance(candidate, str) or not _GIT_SHA.fullmatch(candidate):
        errors.append("invalid_candidate_sha")
    if not isinstance(base_sha, str) or not _GIT_SHA.fullmatch(base_sha) or base_sha == candidate:
        errors.append("invalid_base_sha")
    branch = proposal.get("branch")
    if not isinstance(branch, str) or not branch.startswith("integration/") or ".." in branch or " " in branch:
        errors.append("not_an_isolated_integration_branch")
    phases = proposal.get("phase_receipts")
    if not isinstance(phases, dict) or set(phases) != set(EXPECTED_PHASES) or not all(
        isinstance(phases.get(x), str) and phases[x].strip() for x in EXPECTED_PHASES
    ):
        errors.append("missing_four_phase_receipts")
    if not isinstance(candidate, str) or candidate not in approved_candidates:
        errors.append("not_independently_approved")
    required = proposal.get("required_checks")
    if not isinstance(required, list) or not required or not all(isinstance(x, str) and x for x in required):
        errors.append("invalid_required_checks")
    else:
        checks = verified_ci_by_candidate.get(candidate, frozenset())
        if not set(required).issubset(checks):
            errors.append("missing_verified_ci_check")
    exp = proposal.get("approval_expires_at")
    try:
        if not isinstance(exp, str) or not exp.endswith("Z"):
            raise ValueError()
        end = datetime.fromisoformat(exp.replace("Z", "+00:00"))
        if end <= now.astimezone(timezone.utc):
            errors.append("approval_expired")
    except (ValueError, TypeError):
        errors.append("invalid_approval_expiration")
    if proposal.get("external_side_effects") is not False:
        errors.append("external_side_effects_forbidden")
    if proposal.get("rollback_ref_verified") is not True:
        errors.append("missing_rollback_evidence")
    if proposal.get("source_provenance_verified") is not True:
        errors.append("missing_source_provenance")
    return Decision(not errors, tuple(errors))
