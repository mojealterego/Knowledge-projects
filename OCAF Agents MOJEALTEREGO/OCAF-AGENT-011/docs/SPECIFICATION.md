# Charakterystyka i zakres — PROVENANT EVIDENCE

**PROVENANT EVIDENCE** · ewidencja **011** · serial `AM-PRV-EVD-2026-000011` · Agent ID `AM-OCAF-AI-0011` · katalog `AM-OCAF-PRV-011` · certyfikat `CERT-AM-2026-000011` (**rezerwacja; niewydany**).

## Wejście
Nazwa wywołania: `evidence.review_provenance`.

```json
{"claim":"DNS returned address","origin":"observed","scope_id":"case123","source_ref":"res:primary","observed_at":"2026-10-20T10:00:00Z","source_sha256":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"}
```

## Wyjście
`EvidenceRecord { claim, origin, ui_badge, independently_verified, execution_authorized=false }`.

## Implementacja
Zachowuje odrębne etykiety simulated/observed/verified; observed wymaga scope_id, source_ref, observed_at oraz hash SHA-256; verified dodatkowo wymaga wiarygodnego identyfikatora weryfikatora. Status odpowiedzi HTTP nigdy nie świadczy o istnieniu konkretnej osoby lub konta.

W Pythonie do utworzenia verified wystarczało niepuste verifier_ref. Port Rust wymaga dodatkowo, by identyfikator należał do trusted_verifiers dostarczanych poza żądaniem przez hosta. Waliduje rozmiar claim oraz pól odniesienia, a oddzielny adapter OCAF limituje JSON do 8192 bajtów. To wzmocniona, nie bitowo równoważna wersja modułu Python.

## Niezrealizowane funkcje
samodzielna identyfikacja źródeł w sieci, rozpoznawanie prawdziwości treści, signed verifier receipts, E2E OSINT, runtime do pozyskiwania danych, formalny certyfikat źródła. Produkcyjne OMAs/OMAt, Zenoh z TLS i OSTc pozostają osobnymi zadaniami. Wyniki analizy danych przekazanych przez użytkownika nie potwierdzają stanu realnego środowiska ani certyfikacji.

## Oferta i cennik
Ceny jednostkowej i rynkowej **nie zatwierdzono**; umowy mają stan DRAFT, bez autentycznych stron, polis, SLA i podpisów. Certyfikat `CERT-AM-2026-000011` zarezerwowany, **niewydany**.
