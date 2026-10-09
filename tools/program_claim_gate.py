"""Offline grant and event documentation status validation."""
from dataclasses import dataclass
from datetime import datetime, timezone

@dataclass(frozen=True)
class Result:
    permitted: bool
    reasons: tuple[str, ...]

def review(record: dict, *, now: datetime) -> Result:
    if not isinstance(record, dict):
        raise TypeError("record must be dict")
    if now.tzinfo is None:
        raise ValueError("timezone-aware now required")
    reasons = []
    if record.get("repository") != "mojealterego/Knowledge-projects":
        reasons.append("only_single_repository_authorized")
    if record.get("action") not in ("research", "draft"):
        reasons.append("external_action_not_authorized")
    if record.get("credential_in_payload") is not False:
        reasons.append("secret_must_not_be_in_payload")
    program = record.get("program")
    if program == "alibaba_ai_catalyst":
        money = record.get("credits_usd")
        tokens = record.get("tokens")
        if type(money) is not int or not 0 <= money <= 120000:
            reasons.append("not_a_verified_credit_limit")
        if type(tokens) is not int or not 0 <= tokens <= 2000000000:
            reasons.append("not_a_verified_token_limit")
        if record.get("status") == "awarded":
            reasons.append("funding_not_confirmed")
        if record.get("action") == "draft" and record.get("company_eligibility_checked") is not True:
            reasons.append("company_eligibility_unverified")
    elif program == "general_learning_hacks":
        if record.get("status") != "ended":
            reasons.append("event_is_already_ended")
        if record.get("action") == "draft":
            reasons.append("past_event_not_submittable")
        if now.astimezone(timezone.utc) < datetime(2026, 9, 20, 1, tzinfo=timezone.utc):
            reasons.append("review_date_precedes_source_cutoff")
    else:
        reasons.append("unknown_program")
    return Result(not reasons, tuple(dict.fromkeys(reasons)))
