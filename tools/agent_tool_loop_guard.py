"""Offline guard against repeated no-progress agent tool calls.

Motivation: Zed issue #65199 (2026-10-05) reports repeated read_file
calls with no edits and excessive token use. This code is a reusable
prototyping policy, NOT a patch installed in Zed or an SDK interceptor.

The authoritative progress_version MUST be supplied by a trusted executor;
the model's self-reported claim of progress is not trustworthy.
"""
from __future__ import annotations
from dataclasses import dataclass
import re

_HASH = re.compile(r"^[a-f0-9]{64}$")

@dataclass(frozen=True)
class LoopDecision:
    allowed: bool
    reason: str
    total_calls: int
    total_tokens: int
    identical_no_progress: int

class AgentToolLoopGuard:
    def __init__(self, *, max_calls: int = 50, max_tokens: int = 100_000,
                 max_identical_no_progress: int = 3):
        if any(type(x) is not int or x < 1 for x in
               (max_calls, max_tokens, max_identical_no_progress)):
            raise ValueError("all limits must be positive integers")
        self.max_calls = max_calls
        self.max_tokens = max_tokens
        self.max_identical_no_progress = max_identical_no_progress
        self.calls = 0
        self.tokens = 0
        self.repeat = 0
        self.last_call: tuple[str, str] | None = None
        self.progress_version = 0
        self.halted = False

    def observe(self, *, tool: str, args_sha256: str,
                trusted_progress_version: int, estimated_tokens: int) -> LoopDecision:
        """Check before tool execution; does not execute or read user data.

        Version is monotonically advanced by trusted postcondition readback.
        Even when progress occurs, total calls/tokens still have hard limits.
        """
        if self.halted:
            return self._deny("previously_halted")
        if not isinstance(tool, str) or not tool.strip():
            return self._halt("invalid_tool")
        if not isinstance(args_sha256, str) or not _HASH.fullmatch(args_sha256):
            return self._halt("invalid_argument_digest")
        if (type(trusted_progress_version) is not int or trusted_progress_version < self.progress_version):
            return self._halt("invalid_progress_version")
        if type(estimated_tokens) is not int or estimated_tokens < 0:
            return self._halt("invalid_token_count")
        if self.calls + 1 > self.max_calls:
            return self._halt("call_budget_exhausted")
        if self.tokens + estimated_tokens > self.max_tokens:
            return self._halt("token_budget_exhausted")
        fingerprint = (tool, args_sha256)
        if fingerprint == self.last_call and trusted_progress_version == self.progress_version:
            prospective_repeat = self.repeat + 1
        else:
            prospective_repeat = 1
        if prospective_repeat > self.max_identical_no_progress:
            return self._halt("identical_call_without_progress")
        self.calls += 1
        self.tokens += estimated_tokens
        self.repeat = prospective_repeat
        self.last_call = fingerprint
        self.progress_version = trusted_progress_version
        return LoopDecision(True, "within_budget", self.calls, self.tokens, self.repeat)

    def _deny(self, reason: str) -> LoopDecision:
        return LoopDecision(False, reason, self.calls, self.tokens, self.repeat)

    def _halt(self, reason: str) -> LoopDecision:
        self.halted = True
        return self._deny(reason)
