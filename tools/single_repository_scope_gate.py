"""Offline guard against cross-repository GitHub writes in Knowledge-projects.

This is a deterministic *preflight* to be called by a trusted orchestrator.
It does not possess GitHub permissions, intercept API calls, or retroactively
revert the previous unauthorized cross-repository PR.
"""
from __future__ import annotations

from dataclasses import dataclass

TARGET_REPO = "mojealterego/Knowledge-projects"
DEFAULT_BRANCH = "main"
EXTERNAL_READ_OPERATIONS = frozenset({
    "get_repo", "fetch", "fetch_file", "get_file", "list_repos",
    "search_repositories", "compare_commits", "get_commit_status",
})
WRITABLE_OPERATIONS = frozenset({
    "create_branch", "create_blob", "create_tree", "create_commit",
    "update_ref", "create_file", "update_file", "delete_file",
    "create_pull_request", "merge_pull_request",
})
WORKING_BRANCH_PREFIXES = ("knowledge/", "policy/", "feature/", "fix/")


@dataclass(frozen=True)
class ScopeVerdict:
    permitted: bool
    reasons: tuple[str, ...]


def review_github_operation(
    repository_full_name: str,
    operation: str,
    *,
    target_branch: str | None = None,
    pull_base: str | None = None,
    pull_head: str | None = None,
) -> ScopeVerdict:
    """Review one GitHub operation against the user's single-repo intent.

    The caller must supply the actual GitHub repo/ref, not LLM-inferred values.
    Only PR merge into 'main' is allowed; no direct writes to main.
    """
    if not isinstance(repository_full_name, str) or not isinstance(operation, str):
        raise TypeError("repository and operation must be strings")
    reasons: list[str] = []
    if operation in EXTERNAL_READ_OPERATIONS:
        return ScopeVerdict(True, ())
    if operation not in WRITABLE_OPERATIONS:
        return ScopeVerdict(False, ("operation_not_explicitly_allowed",))
    if repository_full_name != TARGET_REPO:
        reasons.append("external_repository_write_forbidden")
    if operation == "merge_pull_request":
        if pull_base != DEFAULT_BRANCH:
            reasons.append("unexpected_pr_base")
        if not _working_branch(pull_head):
            reasons.append("unexpected_pr_head")
    elif operation == "create_pull_request":
        if pull_base != DEFAULT_BRANCH:
            reasons.append("unexpected_pr_base")
        if not _working_branch(pull_head):
            reasons.append("unexpected_pr_head")
    else:
        if not _working_branch(target_branch):
            reasons.append("write_requires_working_branch")
    return ScopeVerdict(not reasons, tuple(reasons))


def _working_branch(name: str | None) -> bool:
    return (
        isinstance(name, str)
        and name.startswith(WORKING_BRANCH_PREFIXES)
        and name not in (DEFAULT_BRANCH, "")
        and ".." not in name
        and "\\" not in name
        and name[-1] != "/"
        and not any(ch.isspace() for ch in name)
    )
