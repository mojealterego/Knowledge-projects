"""Read-only admission policy for proposed GitHub Marketplace agent integration.

Never installs apps, grants permissions, scans repositories, touches CI, pays,
or authenticates into a third-party vendor. Verified publisher badges and
Marketplace free tiers are NOT substitutes for repo-owner authorization.
"""
from __future__ import annotations
from dataclasses import dataclass
from datetime import datetime, timezone
from typing import Mapping

READ_ONLY = frozenset({"read_public_docs", "read_dependency_advisory", "view_existing_pr"})
EFFECTFUL = frozenset({"comment_on_pr", "create_branch", "edit_code",
                       "launch_local_scanner", "create_feature_flag",
                       "trigger_deployment", "modify_remote_board"})
CAPABILITIES = READ_ONLY | EFFECTFUL

@dataclass(frozen=True)
class OwnerGrant:
    repository: str
    listing_slug: str
    capabilities: frozenset[str]
    expires_at: datetime
    approved: bool

@dataclass(frozen=True)
class Review:
    accepted: bool
    reasons: tuple[str, ...]

def evaluate_marketplace_agent(
    proposed: dict, *,
    grants: Mapping[str, OwnerGrant],
    now: datetime
) -> Review:
    if not isinstance(proposed, dict):
        raise TypeError("proposed must be a dict")
    if not isinstance(now, datetime) or now.tzinfo is None:
        raise ValueError("now must be timezone-aware")
    errors: list[str] = []
    slug = proposed.get("listing_slug")
    url = proposed.get("listing_url")
    repo = proposed.get("repository")
    ticket = proposed.get("grant_id")
    capability = proposed.get("capability")
    if not isinstance(slug,str) or not slug or "/" in slug:
        errors.append("invalid_listing_slug")
    if url != f"https://github.com/marketplace/{slug}":
        errors.append("listing_url_mismatch")
    if not isinstance(repo,str) or repo.count("/")!=1:
        errors.append("invalid_repo_scope")
    if capability not in CAPABILITIES:
        errors.append("unknown_capability")
    grant = grants.get(ticket) if isinstance(ticket,str) else None
    if grant is None or grant.approved is not True:
        errors.append("missing_external_owner_approval")
    else:
        if grant.repository != repo or grant.listing_slug != slug:
            errors.append("approval_scope_mismatch")
        if capability not in grant.capabilities:
            errors.append("capability_not_approved")
        if grant.expires_at.tzinfo is None or grant.expires_at <= now.astimezone(timezone.utc):
            errors.append("approval_expired")
    if proposed.get("secrets_in_prompt") is not False:
        errors.append("prompt_secrets_forbidden")
    if proposed.get("data_export_allowed") is not False:
        errors.append("unapproved_data_export")
    if capability in EFFECTFUL:
        if proposed.get("explicit_effect_approval") is not True:
            errors.append("effectful_action_not_approved")
        if proposed.get("isolated_test_target") is not True and capability=="launch_local_scanner":
            errors.append("scanner_must_target_owned_isolated_lab")
    return Review(not errors, tuple(dict.fromkeys(errors)))
