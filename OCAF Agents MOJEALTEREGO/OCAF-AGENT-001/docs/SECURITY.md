# Model zagrożeń

| Zagrożenie | Stan | Wymaganie |
|---|---|---|
| Prompt injection w zdjęciach | Rust nie czyta obrazów | images untrusted; model-output sandbox |
| Płatne operacje bez autoryzacji | legacy Node ma niechronione endpointy | deny by default; approval + budget |
| Duże HTTP requesty / DoS | legacy body jest nieograniczone | streaming limit, timeout |
| Path traversal / arbitrary file exposure | legacy static readFile bez walidacji ścieżki | safe path containment |
| Tenant escape / confused deputy | brak E2E OMAs | signed identity / isolation gate |
| Replay / token substitution | brak E2E OMAs | nonce, bound RPC sender and digest |
| Sekrety / provider API | legacy wymaga credentials | vault, rotation, redaction |
| OSTc | implementacja i wektory niezweryfikowane | formalna specyfikacja i audyt |

**Zasady:** Rust `photo.plan` bez sieci, obrazów i sekretów, limit 1024 bajty wejścia; brak zgody na automatyczne wywołania API Adobe/OpenAI. Legacy PWA uruchamiać wyłącznie odizolowane do audytu; obecna wersja nie jest prod.
