# Instrukcja użytkownika — NEXUS CONSENT

**NEXUS CONSENT** · ewidencja **013** · serial `AM-COM-NEX-2026-000013` · Agent ID `AM-OCAF-AI-0013` · katalog `AM-OCAF-COM-013` · certyfikat `CERT-AM-2026-0013` **NIEWYDANY**.

Uruchom `cargo test --all-features`, `cargo clippy --all-targets --all-features -- -D warnings` oraz `cargo run -- demo` z manifestem `OCAF Agents MOJEALTEREGO/OCAF-AGENT-013/Cargo.toml`. Nie przekazuj danych osobowych, rzeczywistych kluczy czy poświadczeń zewnętrznych; demo używa fikcyjnych wejść.

Nazwa lokalnego narzędzia OCAF: `communication.review_link`. accepted=true jest wyłącznie zgodą planistyczną, nie pozwoleniem na aktywację radia, połączenie Zenoh, lokalizację GPS ani skanowanie sieci. has_physical_location_evidence=false i device_contacted=false dla wszystkich wejść.

W razie błędu odczytaj `reasons[]`, a na potrzeby audytu zapisz SHA commita i run ID z GitHub Actions, nigdy credentiale. Manipulacja konfiguracją tożsamości → `InvalidConfiguration`. Dostęp do źródeł OSTc → odmowa. Próba przejścia do `Active` → `DeploymentNotAuthorized`.

Przed uruchomieniem produkcyjnym wymagany jest niezależny operator approval, podpisane artefakty i prawdziwa komunikacja OMAs/OMAt/Zenoh.
