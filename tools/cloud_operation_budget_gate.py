"""Offline Google Cloud operation/budget policy review, never an API client.

A URL with project=, authuser=, or billing path establishes *no* IAM
authorization. This checks plans against an independently administered
approval registry; actual Google Cloud IAM, billing, API scopes, audit logs
and quotas must still be verified externally before executing anything.
"""
from __future__ import annotations

from dataclasses import dataclass
from decimal import Decimal, InvalidOperation
import re
from typing import Mapping


ALLOWED_PRODUCTS = frozenset({
    "agent_platform", "compute_engine", "cloud_storage",
    "vision_ai", "speech_to_text", "translation",
    "natural_language", "video_intelligence",
})
ALLOWED_ACTIONS = frozenset({
    "describe_public_documentation", "read_metadata",
    "estimate_cost", "create_resource", "enable_api", "invoke_model",
    "delete_resource", "make_payment",
})
WRITE_ACTIONS = frozenset({"create_resource", "enable_api", "invoke_model",
                          "delete_resource", "make_payment"})
ID = re.compile(r"^[a-z0-9][a-z0-9._-]{2,80}$")


@dataclass(frozen=True)
class CloudApproval:
    """Approval supplied by a separate trusted policy/owner system."""
    project_scope: str
    allowed_actions: frozenset[str]
    allowed_products: frozenset[str]
    max_cost_usd: Decimal
    human_reviewed: bool


@dataclass(frozen=True)
class Verdict:
    allowed: bool
    reasons: tuple[str, ...]


def _cost(value: object) -> Decimal:
    if isinstance(value, bool) or not isinstance(value, str):
        raise ValueError("cost must be decimal string")
    try:
        number = Decimal(value)
    except InvalidOperation as error:
        raise ValueError("bad decimal cost") from error
    if not number.is_finite() or number < 0:
        raise ValueError("invalid_cost")
    return number


def review_cloud_plan(
    plan: dict,
    *,
    independently_approved_requests: Mapping[str, CloudApproval],
) -> Verdict:
    """No network, provisioning, code execution or charge; checks a plan only."""
    if not isinstance(plan, dict):
        raise TypeError("plan must be object")
    problems: list[str] = []
    action = plan.get("action")
    product = plan.get("product")
    if action not in ALLOWED_ACTIONS:
        problems.append("unknown_action")
    if product not in ALLOWED_PRODUCTS:
        problems.append("unknown_product")
    try:
        cost = _cost(plan.get("maximum_cost_usd"))
    except ValueError:
        problems.append("bad_cost")
        cost = Decimal("Infinity")
    if action == "describe_public_documentation":
        if cost != 0:
            problems.append("public_docs_must_be_free")
        if plan.get("data_classification") != "public":
            problems.append("public_docs_require_public_data")
        return Verdict(not problems, tuple(problems))
    request_id = plan.get("request_id")
    if not isinstance(request_id, str) or not ID.fullmatch(request_id):
        problems.append("missing_request_id")
        request_id = ""
    grant = independently_approved_requests.get(request_id)
    if grant is None or grant.human_reviewed is not True:
        problems.append("missing_independent_approval")
    else:
        if plan.get("project_scope") != grant.project_scope:
            problems.append("project_scope_mismatch")
        if action not in grant.allowed_actions:
            problems.append("operation_not_authorized")
        if product not in grant.allowed_products:
            problems.append("product_not_authorized")
        if cost > grant.max_cost_usd:
            problems.append("over_budget")
    if action in WRITE_ACTIONS and plan.get("explicit_external_side_effect_approval") is not True:
        problems.append("side_effect_not_approved")
    if plan.get("credentials_in_prompt") is not False:
        problems.append("secrets_in_prompt_forbidden")
    return Verdict(not problems, tuple(dict.fromkeys(problems)))
