"""GitHub MCP Registry snapshot audit / offline qualification.

Only evaluates a pinned, user-requested PUBLIC LISTING SNAPSHOT; this code
does NOT reach GitHub, authenticate, install MCP servers, call tools,
buy services, or confer GitHub/ChatGPT permissions.
"""
from __future__ import annotations
from collections import Counter
from dataclasses import dataclass
from datetime import datetime
import json
from pathlib import Path
import re
from typing import Mapping

TARGET_REPOSITORY = "mojealterego/Knowledge-projects"
REGISTRY_URL = re.compile(r"^https://github\.com/mcp/[^/?#\s]+/[^/?#\s]+$")
SHA256 = re.compile(r"^[a-f0-9]{64}$")
READ_ACTIONS = frozenset({"catalog_only", "review_metadata"})
EFFECT_ACTIONS = frozenset({"install", "connect", "invoke_tool", "enable_subscription", "make_payment"})
SAFE_ACTIONS = READ_ACTIONS | EFFECT_ACTIONS

CATEGORY_PATTERNS: tuple[tuple[str, str], ...] = (
    ("security", r"security|vulnerabilit|scan|snyk|secret|cve|attack|zero trust|pentest"),
    ("cloud_devops", r"deploy|infrastructure|kubernetes|devops|cloud|terraform|hosting|build|sre"),
    ("developer_tools", r"code|coding|ide|program|git|repo|debug|documentation|compiler"),
    ("research_search", r"search|research|scrap|crawl|web|scholar|citation"),
    ("knowledge_memory", r"memory|knowledge|context|notion|rag|semantic"),
    ("data_database", r"sql|database|postgres|mongo|data warehouse|analytics|vector|index"),
    ("media_design", r"design|image|video|audio|diagram|figma|ui|drawing"),
    ("business_commerce", r"payment|tax|financ|shop|e.?commerce|commerce|customer|invoice|trade|stock|booking"),
    ("communications", r"email|mail|sms|messag|phone|voice|chat|call"),
)

class CatalogError(ValueError):
    """Malformed or non-authoritative registry snapshot."""

@dataclass(frozen=True)
class Summary:
    pages: int
    observed_cards: int
    advertised_registry_total: int
    unreviewed_by_this_snapshot: int
    missing_titles: int
    categories: dict[str, int]

@dataclass(frozen=True)
class Admission:
    allowed: bool
    reasons: tuple[str, ...]

def classify_listing(title: str | None, description: str) -> str:
    """One heuristic bucket for retrieval/navigation, NOT a permission label."""
    sentence = f"{title or ''} {description}".lower()
    for kind, pattern in CATEGORY_PATTERNS:
        if re.search(pattern, sentence):
            return kind
    return "other"

def validate_snapshot(data: dict) -> Summary:
    if not isinstance(data, dict):
        raise CatalogError("snapshot_must_be_object")
    cards = data.get("source_card_entries")
    pages = data.get("source_pages")
    total = data.get("advertised_total")
    if not isinstance(cards, list) or not isinstance(pages, list) or not pages:
        raise CatalogError("missing_catalog_entries")
    if len(cards) > 500 or len(pages) > 20:
        raise CatalogError("snapshot_size_limit")
    if type(total) is not int or total < len(cards):
        raise CatalogError("bad_advertised_total")
    page_numbers = []
    for source in pages:
        if not isinstance(source, dict) or type(source.get("page")) is not int:
            raise CatalogError("invalid_source_page")
        n = source["page"]
        if source.get("source_url") != f"https://github.com/mcp?page={n}":
            raise CatalogError("invalid_page_url")
        if source.get("visible_cards") != 30:
            raise CatalogError("page_card_count_mismatch")
        page_numbers.append(n)
    if sorted(page_numbers) != list(range(1, len(page_numbers) + 1)):
        raise CatalogError("pages_not_contiguous")
    seen, counts = set(), Counter()
    missing_title = 0
    category = Counter()
    for card in cards:
        if not isinstance(card, dict):
            raise CatalogError("invalid_card")
        n = card.get("page")
        pos = card.get("position")
        url = card.get("registry_url")
        if type(n) is not int or n not in page_numbers or type(pos) is not int or not 1 <= pos <= 30:
            raise CatalogError("invalid_card_position")
        if not isinstance(url, str) or not REGISTRY_URL.fullmatch(url):
            raise CatalogError("invalid_registry_url")
        if url in seen:
            raise CatalogError("duplicate_registry_url")
        seen.add(url)
        if not isinstance(card.get("description"), str) or not card["description"].strip():
            raise CatalogError("missing_description")
        title = card.get("title")
        if title is None:
            if card.get("title_missing_in_extract") is not True:
                raise CatalogError("title_missing_without_provenance")
            missing_title += 1
        elif not isinstance(title, str) or not title.strip():
            raise CatalogError("invalid_title")
        if card.get("claim_status") != "PUBLIC_LISTING_DESCRIPTION_UNVERIFIED":
            raise CatalogError("unsupported_claim_state")
        if card.get("tools_list_verified") is not False:
            raise CatalogError("runtime_evidence_not_observed")
        counts[n] += 1
        category[classify_listing(title, card["description"])] += 1
    if len(cards) != sum(counts.values()) or any(counts[x] != 30 for x in page_numbers):
        raise CatalogError("source_page_card_count_mismatch")
    return Summary(len(pages), len(cards), total, total-len(cards), missing_title, dict(sorted(category.items())))

def load_snapshot(path: str | Path) -> tuple[dict, Summary]:
    with Path(path).open("r", encoding="utf-8") as f:
        data = json.load(f)
    return data, validate_snapshot(data)

def evaluate_integration(
    proposal: dict, *, catalog: dict,
    independent_grants: Mapping[str, frozenset[str]],
    now: datetime
) -> Admission:
    """Always deny effects without separately attested live scopes and grants.

    Even passing this metadata gate is NOT authorization by GitHub/MCP host.
    """
    if not isinstance(proposal, dict):
        raise TypeError("proposal must be dict")
    if not isinstance(now, datetime) or now.tzinfo is None:
        raise ValueError("timezone-aware evaluation timestamp required")
    validate_snapshot(catalog)
    reasons = []
    url = proposal.get("registry_url")
    action = proposal.get("action")
    if not isinstance(url, str) or url not in {x["registry_url"] for x in catalog["source_card_entries"]}:
        reasons.append("not_in_reviewed_catalog")
    if action not in SAFE_ACTIONS:
        reasons.append("unknown_action")
    if proposal.get("target_repository") != TARGET_REPOSITORY:
        reasons.append("write_scope_outside_knowledge_projects")
    if action in EFFECT_ACTIONS:
        digest = proposal.get("verified_manifest_sha256")
        if not isinstance(digest, str) or not SHA256.fullmatch(digest):
            reasons.append("manifest_not_independently_verified")
        if proposal.get("issuer_provenance_verified") is not True:
            reasons.append("publisher_not_verified")
        if proposal.get("actual_tools_list_verified") is not True:
            reasons.append("tools_list_not_verified")
        grant = proposal.get("independent_owner_grant_id")
        if not isinstance(grant, str) or action not in independent_grants.get(grant, frozenset()):
            reasons.append("no_independent_permission_for_action")
        if proposal.get("explicit_user_connection_approval") is not True:
            reasons.append("user_connection_approval_required")
        if proposal.get("secrets_in_prompt") is not False:
            reasons.append("secrets_in_prompt_forbidden")
        if proposal.get("external_side_effects") is not False:
            reasons.append("external_side_effects_forbidden")
        # Install/auth/payment is NOT authorized just by satisfying metadata.
        # These actions need platform OAuth/checkout outside this procedure.
        if action in {"install", "connect", "enable_subscription", "make_payment"}:
            reasons.append("platform_install_or_payment_not_supported")
    return Admission(not reasons, tuple(dict.fromkeys(reasons)))

if __name__ == "__main__":
    import argparse
    parser = argparse.ArgumentParser(description="Offline MCP listing audit (never connects MCP)")
    parser.add_argument("snapshot", type=Path)
    args = parser.parse_args()
    _, result = load_snapshot(args.snapshot)
    print(json.dumps(result.__dict__, indent=2, sort_keys=True))
