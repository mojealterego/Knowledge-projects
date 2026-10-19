# VIGILIS SRE — OCAF-AGENT-002

**Nazwa własna:** VIGILIS SRE · **Numer ewidencyjny:** 002 · **Numer seryjny:** `AM-SRE-VIG-2026-000002` · **Agent ID:** `AM-OCAF-AI-0002` · **Nr katalogowy:** `AM-OCAF-SRE-002` · **Certyfikat:** `CERT-AM-2026-0002` — REZERWACJA, NIEWYDANY.

**Stan:** `INCOMPLETE_DEVELOPMENT`. Lokalny kod Rust do celu: diagnostyka incydentów infrastruktury. Nie jest to kompletna migracja projektu źródłowego ani aktywny produkt OMAs/OMAt/OSTc.

## Instalacja i test

Z checkoutu `Knowledge-projects` uruchom:

```sh
cargo fmt --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-002/Cargo.toml" -- --check
cargo test --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-002/Cargo.toml" --all-features
cargo clippy --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-002/Cargo.toml" --all-targets --all-features -- -D warnings
cargo run --manifest-path "OCAF Agents MOJEALTEREGO/OCAF-AGENT-002/Cargo.toml" -- "CrashLoopBackOff"
```

`main.rs` demonstruje **jawnie sztuczne** dane wejściowe, nie połączenie z dostawcą ani rzeczywistą infrastrukturą. Użytkownik nie musi dostarczać kluczy API; nie przekazuj kluczy do tego procesu.

## Wywołanie testowe OCAF

Nazwa narzędzia: `incident.triage`, payload:

```json
{
  "incident_id": "INC-1",
  "tenant_id": "team_a",
  "signal": "CrashLoopBackOff",
  "observations": [
    "operator reports restart loop"
  ],
  "risk_tier": "R3"
}
```

OCAF lokalnie waliduje JSON i przekazuje go do deterministycznej funkcji domenowej. Błędne lub nieznane pola powodują odmowę. Zewnętrzne informacje są niezaufane; żadna klasyfikacja ryzyka nie otwiera prawa do poleceń, podniesienia uprawnień ani zmian produkcji.

## Błędy i incydenty

Błędna nazwa/serial/TOML → `InvalidConfiguration`, niepoprawne dane → `InvalidInput/UnsupportedSignal/InvalidRiskTier`. Próba odczytu plików źródłowych → `SourceEncryptionNotImplemented`. Wymuszenie ACTIVE → `DeploymentNotAuthorized`. Zachowaj jedynie niezbędne logi bez osobowych danych i sekretów; brak serwera do restartu.

## Ostrzeżenia

Agent **nie wykonuje poleceń i nie pozyskuje źródeł**, nie przesyła danych przez Zenoh, nie integruje się sieciowo z OMAs/OMAt. Zobacz `RELEASE_CHECKLIST.md` przed jakimkolwiek wdrożeniem.
