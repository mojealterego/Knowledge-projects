# Architektura — ZIP WARDEN

**ZIP WARDEN** · ewidencja **010** · serial `AM-ARC-ZWD-2026-000010` · Agent ID `AM-OCAF-AI-0010` · katalog `AM-OCAF-ARC-010` · certyfikat `CERT-AM-2026-000010` (**rezerwacja; niewydany**).

```text
Operator / zaufany OCAF host
      |
      +-- OMAs / serwer agentów (NOT DEPLOYED)
      +-- OMAt / serwer narzędzi (NOT DEPLOYED)
      +-- Zenoh keyspace / routing validation only
      |
      v
OCAF Local ToolExecutor -> archive.audit_zip
      |                   |
      |                   +-> Rust ZIP byte inventory
      |                   +-> privacy_names_included=false
      +-- OSTc encrypted source access DENIED
```

## Fizyczny podział
- `src/agent.rs`: Hash SHA-256 całego ZIP i rozpakowanej treści każdego pliku, lista rozszerzeń i rozmiarów, wykrywanie zduplikowanej treści, blokada traversal i niebezpiecznych ścieżek, case-insensitive duplikatów, dowolnych linków symbolicznych, nieobsługiwanych zaszyfrowanych elementów i bomb kompresyjnych.
- `src/zenoh.rs`: lokalna implementacja `ToolManifest` i `ToolExecutor`; przestrzenie nazw `omas/v1` i `omat/v1` nie są sesją sieciową
- `src/security.rs`: fail-closed identyfikatory, transport wyłączony i zabroniony dostęp do źródeł
- `src/lifecycle.rs`: przejście do Active odrzucane
- `src/main.rs`: demonstracja offline
- `tests/`: unit, domain, tożsamość, security i lokalny OCAF

## Warunki zaufania
W wersji OCAF nazwy archiwalnych plików nigdy nie są ujawniane. Flagę include_names=false wybiera host na sztywno, a klient nie może włączyć jej w JSON. Wynik oznacza jedynie INVENTORIED_NOT_SEMANTICALLY_REVIEWED.

Prawdziwa uwierzytelniona tożsamość OMAs i serwera OMAt, podpisy RPC, mTLS, OSTc, multi-host oraz podpisane artefakty nie zostały wykonane. Zabezpieczenia lokalne nie zastępują operator acceptance.
