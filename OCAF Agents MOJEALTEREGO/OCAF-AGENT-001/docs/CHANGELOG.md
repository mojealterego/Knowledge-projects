# Historia zmian — WDA AURELION

## 0.2.1-dev (2026-10-19)
- Nadano własną nazwę agenta: **WDA AURELION**, zachowując WDA Photo Agent jako rodzinę P116.
- Nadano numer seryjny: `AM-WDA-AUR-2026-000001-DE338AC20B42`, odrębny od numeru ewidencyjnego 001, Agent ID i numeru certyfikatu.
- Utworzono centralny JSON rejestr z walidacją unikalności i testy Rust `tests/identity.rs`.
- Powiązano manifest, konfigurację, kod i dokumentację z niezmiennymi polami tożsamości.
- Rozbudowano architekturę, instrukcję, specyfikację, zagrożenia, serwis, zależności, kryteria odbioru oraz dokumenty handlowe i rejestr certyfikacyjny.
- Agent nadal **INCOMPLETE_DEVELOPMENT**; nie zaimplementowano pełnej obróbki obrazów, OMAs/OMAt E2E ani OSTc.

## 0.2.0-dev (2026-10-19)
- Sformalizowano katalog `OCAF-AGENT-001/`, Rust crate i rozdzielenie OMAs/OMAt.
- Dodano 12 testów Rust (poprzedni [CI SUCCESS](https://github.com/mojealterego/Knowledge-projects/actions/runs/39203631208)).

## 0.1.0-dev (2026-10-19)
- Pierwszy wpis 001, historyczne 8 plików WDA przeniesione z P116, zachowane hash blobów.
