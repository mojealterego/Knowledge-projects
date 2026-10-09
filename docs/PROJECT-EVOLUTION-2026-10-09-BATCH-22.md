# Batch 22 — adaptive Googlebook source (2026-10-09)

| Project | Change | Evidence |
|---|---|---|
| **P125 Sovereign Launcher** | **Actual Kotlin source**: dynamic window resize reflow, compact/expanded panes, Polish strings, keyboard focus/min touch targets, state restoration, Android manifest resizability, static tests, QA plan | COMMITTED_SOURCE / STATIC_TESTS_PENDING_CI / NO_APK |
| **P105 OmniMobile** | Distinguish desktop responsive UI from agent accessibility and sanctioned app actuation; extend postcondition matrix for window resize/focus | ARCHITECTURE |
| **P100 NeXus IDE** | Android CLI adaptive skill and emulator/Canary as optional source-derived developer QA; no unreviewed skill installs or cloud access | ARCHITECTURE |
| **P47 Portfolio Integrity** | Normalize `?m=1` article to existing batch18 source; mark new **implementation**, not new independent knowledge witness or new project | REGISTRY |

**Source:** Google Android Developers, 2026-09-22 *Land your apps on Googlebook with adaptive development*. Already received in batch 18; now revisited for actual code changes. **No new numbered projects.**

**New:** modified 3 Android source/XML files and `tools/test_googlebook_adaptive_launcher.py` (13 cases) plus manual QA matrix. CI should confirm **all test suites**, including original `tools/test_sovereign_launcher_policy.py` (7 cases).

**Not done:** Gradle compile, emulator install, true multi-instance, Navigation 3/Compose, HandoffActivityData, Googlebook badge, Play publication, OEM Android 16 device tests, paid services or other repo writes.
