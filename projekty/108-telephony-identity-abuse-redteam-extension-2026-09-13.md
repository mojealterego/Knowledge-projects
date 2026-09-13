# P108 Extension — Telephony / Identity / Communications Red-Team

**Date:** 2026-09-13  
**Parent:** P108 — Adversarial Validation / Red-Team

## Purpose
Turn the communications corpus into a defensive adversarial test suite for P100/P126/P115/P120/P123/P32.

## Test families

1. Caller-ID spoofing detection.
2. Identity/endpoint mismatch.
3. Replay of terminal webhooks.
4. Duplicate Goal Run submission.
5. Stale authorization envelope.
6. Expired temporary identity.
7. Cross-tenant endpoint access.
8. Secret leakage through logs/prompts/artifacts.
9. Malicious caller instructions / prompt injection.
10. Provider failure masquerading as business success.
11. Synthetic-call evidence accidentally promoted to real evidence.
12. Unauthorized recording/transcription.
13. Excessive outbound call volume.
14. Bulk provisioning/activation abuse.
15. Capability confused-deputy attacks.

## Expected outcome
All tests should fail closed, escalate, or produce a bounded non-authoritative observation. None should silently grant authority.

## Safety boundary
The red-team suite may model attack conditions, but production adapters do not implement spoofing, verification bypass, credential extraction, covert surveillance or stealth provisioning.
