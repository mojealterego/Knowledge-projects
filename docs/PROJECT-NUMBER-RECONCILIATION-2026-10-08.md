# Project numbering reconciliation — 2026-10-08

## Detected invariant violation
At the beginning of the batch-12 analysis, the live `main` tree contained:
- `projekty/122-chemia-consent-aware-intimate-two-player-game/README.md`: **P122 CHEMIA**, existing consent-aware two-player mobile-game product.
- `projekty/122-gas-appliance-safety-evidence-companion/README.md`: **P122 gas appliance reference**, newly added by batch-11 auto-ingestion.

They are **two separate products** with the **same numeric ID**. Prior batch-11 genesis failed to check the complete portfolio before allocating P122.

## Reversible repair (history preserved)
1. Leave `P122 CHEMIA` completely unchanged.
2. Reassign the gas safety-reference to the unused numeric identifier **P123** by moving README, `triage.py`, and `test_triage.py` to `projekty/123-gas-appliance-safety-evidence-companion/`.
3. Preserve the two Python file Git blob SHAs exactly across the rename; no gas appliance control behavior is added.
4. Update all **current** batch-11 source ledger / evolution / YAML delta / KB index / projects index references to P123 and include historical correction notes.
5. Change the P47 portfolio-registry canonical specification and permanent project-creation protocol to preflight proposed IDs against **all** existing canonical markdown files and README-backed directories.

## Verification
- The number P122 remains allocated to CHEMIA, without edits.
- P123 now owns only the read-only manufacturer-documentation reference.
- Old mis-numbered gas path must not appear in latest main tree.
- `python tools/project_id_gate.py 122` must reject a proposed new P122; `python tools/project_id_gate.py 123` rejects P123. Run `python -m unittest discover -s tools -p 'test_project_id_gate.py' -v` to verify edge cases.
- The gate scans the **local checkout only**; fetch the current remote main and resolve branch races before merge.

## Historical note
Old Git commits and the previously merged PR #6 remain immutable evidence of the original numbering mistake. Do not rewrite Git history to pretend that the error never happened. References in the earlier archived snapshot may still say P122, but current-state docs carry this correction.
