# P123 Extension — Voice Provider Federation & Provenance

**Date:** 2026-09-13  
**Parent:** P123 — Podcast Agent Factory MAX

## Objective
Federate ElevenLabs, local TTS/voice-cloning systems and future voice providers behind a single production contract.

## Provider adapter contract

`VOICE_INTENT → PROVIDER_CAPABILITY_CHECK → VOICE_PLAN → GENERATION → TAKE_METADATA → QA → APPROVAL`

Adapter manifest includes:
- provider/model/version;
- language/locale support;
- realtime capability;
- voice design/cloning capability;
- latency characteristics;
- licensing/provenance constraints;
- input/output formats;
- watermark/provenance features where available;
- cost metadata;
- failure modes;
- local/cloud privacy mode.

## Voice provenance record

`voice_profile_id + source_audio_provenance + consent_scope + provider + model_version + generation_config + operator + timestamp + artifact_digest`

## New invariant

`GENERATED VOICE ≠ IDENTITY PROOF`

A cloned or designed voice may be used only under the project's explicit voice-rights/provenance policy.

## Benchmark integration

The existing TTS latency, direction and design benchmark projects become reusable P123 evaluation suites. Evaluation should cover:
- transcript fidelity;
- role/persona fit;
- pronunciation;
- emotional continuity;
- latency;
- interruption behavior;
- voice consistency;
- artifact provenance.

## Integration

P126 provides communications execution; P123 remains responsible for creative voice production and episode lineage.
