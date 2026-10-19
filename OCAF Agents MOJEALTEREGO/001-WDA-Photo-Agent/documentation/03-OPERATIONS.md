# Instrukcja obsługi WDA 001

1. **Operator i identyfikacja:** `AM-OCAF-AI-0001`, status DRAFT/PARTIAL; tylko uprawniony operator może zatwierdzić wydanie.
2. **Uruchomienie:** przegląd SHA i manifestu; `cargo test --manifest-path rust-agent/Cargo.toml --all-features` po przejściu CI; skonfiguruj rzeczywistego OCAF hosta z oddzielnym tenant i signed RPC. Bez tego używaj tylko testów offline.
3. **Wywołanie:** OCAF tool `photo.plan` z `{"goal":"adjust exposure","identity_lock":true}`. Wynik jest planem, **nie generowanym zdjęciem**.
4. **Monitorowanie:** correlation ID, odmowy authz, limity, audyt i retencja jako wymagania hosta; nie logować zdjęć/tajemnic.
5. **Incident:** wyłącz capability i dostęp providerów; brak auto-retry płatnych usług; zabezpiecz logi i uruchom recovery dopiero po ocenie operatora.
6. **Wycofanie:** revoke, eksport danych zgodny z umową, decommission i protokół; nie deklarować restore bez testu.

**Błędy:** `InvalidGoal` → popraw payload, `IdentityUnlockDenied` → zakazane odblokowanie, brak OMAs auth → deny. Eskalacja do właściciela polityk i security operatora.
