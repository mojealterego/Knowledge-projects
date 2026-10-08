"""Read-only adoption preflight for NousResearch/Hermes ecosystem repositories.

Never clones code, executes scripts, downloads model weights, uses credentials,
grants tools, merges PRs or calls a provider. Evidence is supplied by a trusted
external reviewer, not the agent-proposed repository manifest.
"""
from __future__ import annotations
from dataclasses import dataclass
from datetime import datetime, timedelta, timezone
import re
from typing import Mapping

SHA = re.compile(r"^[0-9a-f]{40}$")
SOURCE = re.compile(r"^NousResearch/[a-zA-Z0-9_.-]+$")
MODES = frozenset({"document_only", "isolated_evaluation", "runtime_adoption"})

@dataclass(frozen=True)
class TrustedReview:
    revision_sha: str
    repository: str
    source_checked_at: datetime
    license_reviewed: bool
    security_reviewed: bool
    tests_passed: bool
    fork_synced: bool
    owner_approved: bool

@dataclass(frozen=True)
class Decision:
    allowed: bool
    reasons: tuple[str, ...]

def evaluate_upstream_adoption(
    proposal: dict, *,
    trusted_reviews: Mapping[str, TrustedReview],
    now: datetime,
) -> Decision:
    if not isinstance(proposal, dict):
        raise TypeError("proposal must be a dict")
    if not isinstance(now, datetime) or now.tzinfo is None:
        raise ValueError("now must be timezone aware")
    errors = []
    repo = proposal.get("repository")
    sha = proposal.get("revision_sha")
    mode = proposal.get("mode")
    ticket = proposal.get("review_ticket")
    if not isinstance(repo, str) or not SOURCE.fullmatch(repo):
        errors.append("repository_scope_not_allowed")
    if not isinstance(sha, str) or not SHA.fullmatch(sha):
        errors.append("unpinned_revision")
    if mode not in MODES:
        errors.append("unknown_mode")
    if proposal.get("is_archived") is not False and mode == "runtime_adoption":
        errors.append("archived_or_unknown_repo_cannot_be_runtime")
    if proposal.get("license_declared") is not True and mode != "document_only":
        errors.append("license_evidence_missing")
    review = trusted_reviews.get(ticket) if isinstance(ticket,str) else None
    if mode == "document_only":
        # Public metadata can be discussed without downloading code or claiming
        # any artifact is safe to execute.
        if proposal.get("external_side_effects") is not False:
            errors.append("effects_not_allowed")
        return Decision(not errors, tuple(dict.fromkeys(errors)))
    if review is None:
        errors.append("no_independent_review")
    else:
        if review.repository != repo or review.revision_sha != sha:
            errors.append("review_revision_scope_mismatch")
        if review.source_checked_at.tzinfo is None or review.source_checked_at > now or now-review.source_checked_at>timedelta(days=30):
            errors.append("stale_upstream_review")
        if not review.license_reviewed:
            errors.append("license_not_independently_checked")
        if mode == "runtime_adoption":
            if not review.security_reviewed or not review.tests_passed:
                errors.append("missing_security_ci_evidence")
            if proposal.get("is_fork") is True and not review.fork_synced:
                errors.append("fork_sync_unverified")
            if not review.owner_approved:
                errors.append("owner_approval_missing")
    if proposal.get("external_side_effects") is not False:
        errors.append("effects_not_allowed")
    return Decision(not errors, tuple(dict.fromkeys(errors)))
