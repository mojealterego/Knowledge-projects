# Architektura — PROVENANT EVIDENCE

**PROVENANT EVIDENCE** · ewidencja **013** · serial `AM-PRV-EVD-2026-000013` · Agent ID `AM-OCAF-AI-0013` · katalog `AM-OCAF-PRV-013` · certyfikat `CERT-AM-2026-000013` (**rezerwacja; niewydany**).

```text
Operator / zaufany OCAF host
      |
      +-- OMAs / serwer agentów (NOT DEPLOYED)
      +-- OMAt / serwer narzędzi (NOT DEPLOYED)
      +-- Zenoh keyspace / routing validation only
      |
      v
OCAF Local ToolExecutor -> evidence.review_provenance
      |                   |
      |                   +-> Rust EvidenceInput validation
      |                   +-> host-owned verifier list
      +-- OSTc encrypted source access DENIED
```

## Fizyczny podział
- `src/agent.rs`: Zachowuje odrębne etykiety simulated/observed/verified; observed wymaga scope_id, source_ref, observed_at oraz hash SHA-256; verified dodatkowo wymaga wiarygodnego identyfikatora weryfikatora. Status odpowiedzi HTTP nigdy nie świadczy o istnieniu konkretnej osoby lub konta.
- `src/zenoh.rs`: lokalna implementacja `ToolManifest` i `ToolExecutor`; przestrzenie nazw `omas/v1` i `omat/v1` nie są sesją sieciową
- `src/security.rs`: fail-closed identyfikatory, transport wyłączony i zabroniony dostęp do źródeł
- `src/lifecycle.rs`: przejście do Active odrzucane
- `src/main.rs`: demonstracja offline
- `tests/`: unit, domain, tożsamość, security i lokalny OCAF

## Warunki zaufania
Niezależny rejestr weryfikatorów musi być dostarczony przez operatora do instancji AgentTool. Sam ciąg znaków verifier_ref ani deklaracja pochodzenia w JSON nie stanowi dowodu prawdziwości. Etykieta WERYFIKACJA UDOKUMENTOWANA nadal wymaga przejrzenia faktycznych dowodów.

Prawdziwa uwierzytelniona tożsamość OMAs i serwera OMAt, podpisy RPC, mTLS, OSTc, multi-host oraz podpisane artefakty nie zostały wykonane. Zabezpieczenia lokalne nie zastępują operator acceptance.
