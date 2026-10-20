# Instrukcja użytkownika — FIDES GRANTIS

**FIDES GRANTIS** · ewidencja **012** · serial `AM-GRT-FID-2026-000012` · Agent ID `AM-OCAF-AI-0012` · katalog `AM-OCAF-GRT-012` · certyfikat `CERT-AM-2026-0012` **NIEWYDANY**.

Uruchom `cargo test --all-features`, `cargo clippy --all-targets --all-features -- -D warnings` oraz `cargo run -- demo` z manifestem `OCAF Agents MOJEALTEREGO/OCAF-AGENT-012/Cargo.toml`. Nie przekazuj danych osobowych, rzeczywistych kluczy czy poświadczeń zewnętrznych; demo używa fikcyjnych wejść.

Nazwa lokalnego narzędzia OCAF: `program.claim_review`. Wynik permitted=true jest lokalną decyzją walidatora; nie potwierdza otrzymania pieniędzy ani grantów, nie składa wniosku, nie uruchamia obliczeń i nie sprawdza na żywo stanu programów. Czas UTC pochodzi od hosta, nie z modelowego JSON.

W razie błędu odczytaj `reasons[]`, a na potrzeby audytu zapisz SHA commita i run ID z GitHub Actions, nigdy credentiale. Manipulacja konfiguracją tożsamości → `InvalidConfiguration`. Dostęp do źródeł OSTc → odmowa. Próba przejścia do `Active` → `DeploymentNotAuthorized`.

Przed uruchomieniem produkcyjnym wymagany jest niezależny operator approval, podpisane artefakty i prawdziwa komunikacja OMAs/OMAt/Zenoh.
