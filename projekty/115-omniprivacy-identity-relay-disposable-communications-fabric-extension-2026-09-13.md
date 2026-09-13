# P115 Extension — Subscriber / Endpoint Lifecycle Hardening

**Date:** 2026-09-13  
**Parent:** P115 — OmniPrivacy Identity Relay & Disposable Communications Fabric MAX

## Scope
Extend P115 from phone/email alias abstraction to a stronger communication-identity lifecycle that can safely represent eSIM test profiles, SIP identities, disposable mailboxes, persistent masks and platform aliases.

## Canonical identity object

`identity_id + channel + provider + endpoint + owner + purpose + consent + activation_window + retention + provenance + revocation_state`

## New channel classes

- `email_disposable`
- `email_persistent_mask`
- `email_platform_alias`
- `phone_temporary`
- `phone_persistent_secondary`
- `sip_identity`
- `esim_test_profile`
- `esim_production_profile`

## Lifecycle

`PROPOSED → TESTING → ACTIVE → PAUSED → EXPIRING → REVOKED → DESTROYED`

Transitions require authoritative state and audit records.

## eSIM security boundary

Research material containing subscriber cryptographic values, database passwords or access tokens is treated as sensitive. P115 must never store or reproduce such values in source code, prompts, examples or knowledge artifacts.

Production secrets must be generated/stored/rotated through an HSM, secure element or dedicated secret-management system. Test identities must be cryptographically and operationally separated from production identities.

## Disposable communication boundary

An alias does not imply anonymity. A temporary mailbox is not suitable for critical recovery unless the provider explicitly supports it. Forwarding is not equivalent to end-to-end confidentiality. Provider privacy claims remain source claims unless independently audited.

## Verification additions

- profile collision tests;
- revocation propagation;
- expired identity rejection;
- provider-account dependency tests;
- stale routing prevention;
- secret-leak scanning;
- identity-to-recipient binding;
- deletion/retention verification;
- cross-tenant isolation;
- anti-abuse regression via P108.
