# Project 106 — Android Mobile Forensics & Data Recovery Validation Lab MAX

## Status
PROPOSED → ARCHITECTURE BASELINE → PROJECT GENESIS 2026-09-10

## Mission
Create a defensive and legally bounded research laboratory for Android digital forensics, focused on understanding encryption, evidence acquisition, integrity, artifact interpretation and recoverability limits across modern devices.

## Distinct boundary
P32 owns OSINT/evidence investigation and relationship analysis. P54/P59/P60 own broader security and hardware-trust research. P106 owns **mobile forensic acquisition, Android artifact integrity and recoverability validation**.

## Canonical architecture

```text
AUTHORIZED DEVICE / FORENSIC IMAGE
          ↓
CHAIN OF CUSTODY
          ↓
ACQUISITION VALIDATION
  logical / approved physical
          ↓
CRYPTO + STORAGE MODEL
 FBE / Keystore / TEE / UFS
          ↓
ARTIFACT EXTRACTION
 SQLite / app artifacts / system metadata
          ↓
INTEGRITY + HASHING
          ↓
TIMELINE / CORRELATION
          ↓
EVIDENCE CLASSIFICATION
          ↓
REPRODUCIBLE REPORT
```

## Source-derived technical model
The supplied Android forensic report emphasizes the transition from FDE to FBE and hardware-backed TEE protection. It describes file encryption keys, class keys and the role of isolated hardware in protecting cryptographic material. fileciteturn908file6L289-L305

The Motorola G54 case study identifies a MediaTek Dimensity 7020/MT6855 platform and UFS 2.2 storage, and discusses how storage-controller behavior and hardware security affect recoverability. fileciteturn908file11L25-L49

The corpus also references open-source forensic/audit projects including Android Quick Forensics, Android Keystore Audit, OWASP MSTG tooling and MediaTek-oriented utilities. These references become a **research/tool-evaluation catalogue**, not authorization to bypass device protections. fileciteturn908file0L17-L40

## Recoverability model
P106 records recoverability as a function of:
- device state;
- authentication state;
- encryption state;
- hardware-backed key availability;
- storage-controller behavior;
- artifact persistence;
- acquisition method;
- application-specific protections;
- chain-of-custody integrity.

"Deleted" does not automatically mean "recoverable", and a raw physical image does not automatically mean "decryptable".

## Evidence classes

```text
RAW ACQUISITION
    ↓
CRYPTOGRAPHICALLY INTEGRITY-CHECKED IMAGE
    ↓
DERIVED ARTIFACT
    ↓
INTERPRETATION
    ↓
CORROBORATED FINDING
```

Every derived finding retains its acquisition and transformation provenance.

## Motorola G54 validation track
The G54 material is used as a concrete test-device profile. The project validates hardware/storage assumptions, acquisition repeatability and artifact recovery in an authorized laboratory. It does **not** turn the document's described BootROM/auth-bypass techniques into an operational capability.

## Security and legal boundary
- Only devices/data for which the operator has lawful authorization enter the lab.
- No credential theft, lock-screen bypass playbook, covert extraction or unauthorized access workflow is implemented.
- Exploit references are retained only as defensive threat-model context.
- Evidence handling follows documented chain-of-custody and reproducibility requirements.
- Personal identifiers from case documents are not copied into the public knowledge base.

## Verification
1. Known-device synthetic corpus.
2. Hash and chain-of-custody validation.
3. FBE/Keystore artifact consistency tests.
4. SQLite parsing against known fixtures.
5. Deleted-artifact recoverability experiments on disposable lab devices.
6. Acquisition reproducibility.
7. Cross-tool artifact comparison.
8. Timeline consistency tests.
9. False-positive/false-attribution testing.
10. Reproducible forensic-report generation.

## Maturity
Architecture baseline. The supplied documents contain conceptual and source-derived forensic claims; no repository-independent recovery capability is established.

## Derived from
P31, P32, P54, P59, P60, P72.

## Primary source witnesses
- `Odzyskiwanie Danych Android GitHub.PDF`
- `Odzyskiwanie danych Motorola G54.PDF`

---

## 2026-10-08 — batch 18: public web source evolution

The public `9aylas/Pegasus-samples` repository README explicitly describes Android/iOS spyware samples and links to technical analysis. Treat it as **untrusted high-risk malware source**. Add DefensibleThreatIntelEvidence {repository_url,readme_revision,trust_claims,third_party_analysis,actual_obtained_binary_sha_only_if_acquired,device_scope_and_consent}. Do not download ZIP, execute binaries, claim infection or derive covert surveillance capability. No sample archive retrieved.
