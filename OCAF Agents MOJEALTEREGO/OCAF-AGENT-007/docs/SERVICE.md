# Instrukcja serwisowa — VIGIA LOOP

**VIGIA LOOP** · nr ewidencyjny **007** · nr seryjny `AM-OPS-VIG-2026-000007` · Agent ID `AM-OCAF-AI-0007` · katalog `AM-OCAF-OPS-007` · certyfikat `CERT-AM-2026-0007` — **NIEWYDANY**.

## Kontrole operatora

1. Odczytać `AGENT_IDENTITY_REGISTRY.json` i porównać nazwę, serial, katalog i Agent ID z TOML, manifestem i stałymi Rust.
2. Przeprowadzić formatowanie, testy Rust, Clippy z `-D warnings`, CLI smoke i readback SHA jobów CI.
3. Nie włączać sieci ani źródeł bez pełnej specyfikacji OSTc i bez kryptograficznych grantów OMAs/OMAt.
4. Zapewnić ciągłość stanu jednej sesji guard. Po halcie przekazać sprawę operatorowi; nie tworzyć nowego guarda, aby ominąć limity.
5. Incydenty, błędy i rollback powiązać z commit SHA oraz zanikniętymi dowodami; żaden model nie może zatwierdzać własnego release.

## Lifecycle

`Draft → Tested → Staged` jest tylko wewnętrzną ewidencją developerską. `Active` zawsze DENIED. Formalne cofanie certyfikatu, rotacja kluczy i zależne serwery OMAs/OMAt pozostają niewdrożone. SLA, RTO/RPO, ubezpieczenie i support 24/7: NIEUSTALONE.
