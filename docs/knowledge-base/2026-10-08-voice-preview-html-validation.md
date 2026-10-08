# ElevenLabs voice preview upload — artifact validity note (2026-10-08)

Source filename: `voice_preview_ian — polish narrator (warm_deep).mp3.html`.
Observed MIME/content: HTML / Next.js app document; page title "AI Voice Generator & Text to Speech | ElevenLabs".
Observed references: JavaScript and CSS resources, generic service/SEO metadata and text-to-speech UI route.
**Not observed:** `<audio>`, `<source>`, embedded `data:audio`, an `audio/mpeg` response, usable MP3 bytes, Ian voice sample metadata or a proven voice ID.

## Decision
This is a **saved web page**, not a voice recording despite its filename. No waveform, voice characteristics, naturalness, accent, language proficiency or sample ownership can be evaluated from these bytes.

## Reusable asset validation gate for P81 / P109
1. Validate actual file signature and MIME independent of extension; do not trust names.
2. If MP3, decode a bounded preview, record duration, sample rate, channels and content checksum.
3. Save provider voice ID, provider/model/version, licensing/consent basis and source URL separately from any preview.
4. For cloud TTS, require configured provider credentials through server-side secret management; never persist tokens in a public report.
5. Speech UX evaluation: Polish pronunciation, pauses, clarity, loudness consistency, latency and safe fallback to captions.
6. Consent for voice cloning/impersonation must be explicit; avoid implying this HTML establishes voice rights.
7. Audio output and narrated game content are reviewable artifacts, not authority to change game state.

Status: **INVALID_AUDIO_REFERENCE**. Correct input for speech assessment is the actual audio file, not an HTML-exported application shell.
