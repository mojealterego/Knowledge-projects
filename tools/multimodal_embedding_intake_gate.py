"""Non-network qualification gate for Gemini Embedding 2 multimodal RAG inputs.

Source: Google Developers Blog 2026-04-30 and Gemini API model documentation.
This module never sends source data, embeds content, spends credits or
assumes a console link grants access to a private Google Cloud project.
"""
from __future__ import annotations

from dataclasses import dataclass
import re

SHA256 = re.compile(r"^[0-9a-f]{64}$")
MODEL = "gemini-embedding-2"
# Documentation lists flexible output sizes from 128 to 3072.
MIN_DIM, MAX_DIM = 128, 3072
SOURCE_LIMITS = {"text": 8192, "image": 6, "video": 120, "audio": 180, "pdf": 6}
CLASSIFICATIONS = frozenset({"public", "internal", "restricted"})
TASKS = frozenset({"retrieval_document", "retrieval_query", "fact_checking", "code_retrieval", "semantic_similarity"})
PROVIDERS = frozenset({"gemini_api", "gemini_enterprise_agent_platform"})


@dataclass(frozen=True)
class Review:
    qualified: bool
    reasons: tuple[str, ...]
    estimated_cost_usd: None = None  # not a price calculator


def qualify_multimodal_intake(
    manifest: dict,
    *,
    externally_approved_source_hashes: frozenset[str],
) -> Review:
    """Validate conservative per-request limits and explicit remote-data consent.

    Each item has modality and a measured unit count:
    - text: tokenizer token count, image: image count,
    - video/audio: duration seconds, PDF: rendered page count.
    Claims in input documents must not set externally_approved_source_hashes.
    """
    if not isinstance(manifest, dict):
        raise TypeError("manifest must be an object")
    errors: list[str] = []
    if manifest.get("model") != MODEL:
        errors.append("unsupported_model")
    if manifest.get("provider") not in PROVIDERS:
        errors.append("unknown_provider")
    dimension = manifest.get("output_dimensions")
    if type(dimension) is not int or not MIN_DIM <= dimension <= MAX_DIM:
        errors.append("invalid_dimensions")
    if manifest.get("task") not in TASKS:
        errors.append("unknown_task")
    digest = manifest.get("source_sha256")
    if not isinstance(digest, str) or not SHA256.fullmatch(digest):
        errors.append("invalid_source_sha256")
    classification = manifest.get("classification")
    if classification not in CLASSIFICATIONS:
        errors.append("invalid_classification")
    if manifest.get("remote_processing_opt_in") is not True:
        errors.append("remote_processing_not_authorized")
    if classification in ("internal", "restricted") and digest not in externally_approved_source_hashes:
        errors.append("sensitive_source_not_independently_approved")
    items = manifest.get("items")
    if not isinstance(items, list) or not items:
        return Review(False, tuple(errors + ["missing_items"]))
    totals: dict[str, int] = {name: 0 for name in SOURCE_LIMITS}
    for item in items:
        if not isinstance(item, dict) or item.get("modality") not in SOURCE_LIMITS:
            errors.append("unsupported_modality")
            continue
        kind = item["modality"]
        units = item.get("units")
        if type(units) is not int or units <= 0:
            errors.append("unmeasured_or_invalid_units")
            continue
        totals[kind] += units
    for kind, limit in SOURCE_LIMITS.items():
        if totals[kind] > limit:
            errors.append(f"{kind}_limit_exceeded")
    if manifest.get("contains_credentials") is not False:
        errors.append("credentials_must_not_leave_client")
    if manifest.get("contains_nonconsensual_personal_data") is not False:
        errors.append("personal_data_without_consent")
    return Review(not errors, tuple(dict.fromkeys(errors)))
