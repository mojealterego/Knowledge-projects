"""Offline evidence gate for external contribution/bounty/plugin opportunities.

This does not enroll in Zed Guild, submit PRs to other repos, claim bounties,
install WordPress plugins or process payment. Source evidence must come from
a trusted readback rather than generated/misleading page snippets.
"""
from __future__ import annotations
from dataclasses import dataclass
from datetime import datetime, timezone, timedelta
from urllib.parse import urlsplit

TARGET_REPO = "mojealterego/Knowledge-projects"
SOURCE_KINDS = frozenset({"zed_issue", "zed_project_board",
                           "railway_template_bounty", "railway_project_board",
                           "wordpress_paid_plugins"})
ALLOWED_ACTIONS = frozenset({"research", "prototype_in_knowledge_projects"})
MAX_AGE = timedelta(days=14)

@dataclass(frozen=True)
class SourceObservation:
    kind: str
    url: str
    status: str
    verified_at: datetime
    independent_verified: bool

@dataclass(frozen=True)
class OpportunityDecision:
    allowed: bool
    reasons: tuple[str, ...]
    claimable_reward_verified: bool = False

def qualify_source(observation: SourceObservation, *,
                   now: datetime, action: str,
                   write_target: str = TARGET_REPO) -> OpportunityDecision:
    if not isinstance(observation, SourceObservation):
        raise TypeError("typed source observation required")
    if not isinstance(now, datetime) or now.tzinfo is None:
        raise ValueError("timezone aware now required")
    reasons: list[str] = []
    if action not in ALLOWED_ACTIONS:
        reasons.append("external_action_not_authorized")
    if write_target != TARGET_REPO:
        reasons.append("write_outside_knowledge_projects_forbidden")
    if observation.kind not in SOURCE_KINDS:
        reasons.append("unknown_source_kind")
    if not _belongs_to_source(observation.kind, observation.url):
        reasons.append("source_url_does_not_match_provider")
    if observation.verified_at.tzinfo is None:
        reasons.append("evidence_has_no_timezone")
    elif observation.verified_at > now + timedelta(minutes=5) or now - observation.verified_at > MAX_AGE:
        reasons.append("stale_or_future_evidence")
    if observation.independent_verified is not True:
        reasons.append("source_not_independently_verified")
    if action == "prototype_in_knowledge_projects":
        if observation.kind not in {"zed_issue", "railway_template_bounty"}:
            reasons.append("board_or_catalog_is_not_specific_task")
        if observation.status != "open":
            reasons.append("no_verified_open_task")
    if observation.kind == "wordpress_paid_plugins" and action != "research":
        reasons.append("paid_plugin_requires_site_entitlement_and_payment_approval")
    # The tool cannot verify a payout, even when an issue or bounty is open.
    return OpportunityDecision(not reasons, tuple(dict.fromkeys(reasons)), False)

def _belongs_to_source(kind: str, url: str) -> bool:
    if not isinstance(url, str):
        return False
    try:
        u = urlsplit(url)
    except ValueError:
        return False
    if u.scheme != "https" or u.username or u.password or u.port not in (None, 443):
        return False
    paths = {
        "zed_issue": ("github.com", "/zed-industries/zed/issues/"),
        "zed_project_board": ("github.com", "/orgs/zed-industries/projects/74"),
        "railway_template_bounty": ("station.railway.com", "/"),
        "railway_project_board": ("github.com", "/orgs/railwayapp/projects/2"),
        "wordpress_paid_plugins": ("wordpress.com", "/plugins/browse/paid/"),
    }
    if kind not in paths:
        return False
    host, path = paths[kind]
    if u.hostname != host:
        return False
    if kind in {"zed_project_board", "railway_project_board"}:
        return u.path.rstrip("/") == path
    if kind == "railway_template_bounty":
        return u.path.startswith(("/templates/", "/questions/")) and len(u.path) > 12
    if kind == "zed_issue":
        return u.path[len(path):].isdigit()
    return u.path.startswith(path) and len(u.path) > len(path)
