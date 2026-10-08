# P123 — Gas Appliance Safety Evidence Companion

**Status:** PROPOSED → ARCHITECTURE → READ-ONLY PROTOTYPE (UNIT-TESTED LOCALLY)
**Domain:** manufacturer-manual reference and professional escalation for residential gas appliances; no maintenance automation.

## Why this is a new portfolio project
The uploaded `vitrix_20_plus_s_instrukcja.pdf` is a 16-page Polish manual with title on cover **Immergas VICTRIX PLUS/S**. The exact owner device model is unverified. P74 covers automotive OBD2 and P121 covers cloud infrastructure; neither owns the safety-critical gas-appliance manual and technician-escalation domain. P123 has separate intake/output, revision, evidence and safety requirements.

## Architecture and data boundary
```
MANUFACTURER / EXACT MODEL + MANUAL REVISION
 -> SOURCE SHA-256 AND PAGE
 -> INPUT: DISPLAYED FAULT CODE
 -> HISTORIC MANUAL CODE LABEL (NO DIAGNOSIS)
 -> QUALIFIED TECHNICIAN / CURRENT MANUAL REFERRAL
 -> AUDITABLE READ-ONLY RESULT
```
The reference lookup accepts only documented numeric codes, rejects malformed input and routes undocumented codes to verification. It **never** actuates valves, burners, fans, pumps, sensors, electrical circuits or safety interlocks. It does not prescribe repair, reset attempts, pressure changes or gas adjustments.

## Initial implementation
- `triage.py` — deterministic Python mapping of ten fault codes on the supplied PDF. Source labels are not device diagnoses.
- `test_triage.py` — five `unittest` methods. Run `python -m unittest -v test_triage.py` within this directory.
- Manufacturer/model/manual provenance: SHA-256 `f4f8e2b36e4aa6cc9a77636a3e1bca6d002f5b8e884ac275bddeafdbc9e3b71e`.

## Required gates before real users
1. Confirm actual appliance model, firmware and latest manufacturer manual; do not infer from filename.
2. Qualified/licensed gas technician reviews labels, safety boundaries and escalation flow.
3. Independent safety, copyright, accessibility and privacy review.
4. No integration with any boiler control plane without a separately governed safety case (out of this project scope).
5. Device field testing must be performed by authorized personnel; no physical device was connected here.

**Verification evidence:** local Python stdlib test suite: five tests passed, without hardware. No cloud app, technician-verified diagnosis or service deployment is claimed.

Reference notes: [source extraction](../../docs/knowledge-base/2026-10-08-gas-appliance-manual-safety-boundary.md).
