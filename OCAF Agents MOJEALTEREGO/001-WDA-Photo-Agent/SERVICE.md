# Cykl życia i serwis

Stany: DRAFT → VALIDATED → BUILT → TESTED → SIGNED → REGISTERED → STAGED → ACTIVE → DEGRADED/PAUSED → RETIRED → REVOKED/ARCHIVED. Pierwszy wpis do ewidencji 001 nie upoważnia do ACTIVE.

**Instalacja:** sklonuj repo na środowisku nieprodukcyjnym, uruchom `cargo test --manifest-path "OCAF Agents MOJEALTEREGO/001-WDA-Photo-Agent/rust-agent/Cargo.toml" --all-features`, zweryfikuj pin rev OCAF, konfigurację tool registry i deny-by-default signed RPC (nie uruchomiono testów E2E). Nie podłączaj kluczy ani publicznych API do legacy.

**Obsługa:** wyłącznie `photo.plan` z `identity_lock:true`, nie oczekuj zdjęcia wynikowego. Inne akcje muszą mieć nowy, jawny kontrakt i pozwolenia. Audyt zdarzeń, trace ID i retencję zapewnia przyszły host OCAF.

**Awaria:** wyłącz capability, zachowaj trace bez danych osobowych, cofnij zatwierdzoną wersję; zabroniony automatyczny retry nieidempotentnych operacji dostawców. Test SIGKILL/recovery wymaga faktycznego hosta.

**SLA/cennik/ubezpieczenie:** NIEZATWIERDZONE. Refurbished, migracja, eksport i cyfrowe wycofanie według zatwierdzonego protokołu operatora, RODO i obowiązującego prawa. Wzory umów nie zastępują przeglądu prawnego.
