# P81 extension — TTS asset provenance and malformed-preview detection (2026-10-08)

Reference: `docs/knowledge-base/2026-10-08-voice-preview-html-validation.md`.
Actual uploaded `voice_preview_ian — polish narrator (warm_deep).mp3.html` is HTML application shell; no audio bytes embedded in analyzed file.

## Proposed verification pipeline
```text
UPLOAD -> MAGIC BYTE / MIME / DECODER
 -> LICENSE + PROVIDER VOICE ID + CONSENT RECORD
 -> AUDIO METADATA (duration, sample rate, channels, sha256)
 -> POLISH PRONUNCIATION + NATURALNESS + LATENCY EVAL
 -> CAPTIONS / TEXT FALLBACK -> APPROVED GAME NARRATION
```
False extension `.mp3.html` must be rejected by the audio importer; it can be retained only as an HTML provenance/reference entry. No voice quality or identity can be inferred from the filename.

Preserve P81 invariant: narrated/LLM-generated content cannot mutate authoritative game state. Voice cloning requires authorization from rights holders.

State: test specification; no TTS audio generated, imported or evaluated.
