# Project 105 — OmniMobile MAS Android Agent Control Fabric MAX

## Status
PROPOSED → ARCHITECTURE BASELINE → PROJECT GENESIS 2026-09-10

## Mission
Build a multimodal, multi-agent Android control fabric that can perceive mobile UI state, plan tasks, execute authorized interactions and verify the resulting state without relying on brittle fixed coordinate scripts.

## Distinct boundary
P86 owns natural-language → game → Android build delivery. P100 owns the NeXus developer environment. P61/P57 own the broader PUI and multimodal substrate. P105 owns **mobile GUI perception, agent collaboration and verified Android actuation**.

## Canonical architecture

```text
ANDROID DEVICE / EMULATOR
        ↓
ADB / AUTHORIZED ACCESSIBILITY
        ↓
UIAUTOMATOR XML + SCREENSHOT + DEVICE STATE
        ↓
PERCEPTION FUSION
 XML semantic tree + visual grounding
        ↓
TASK STATE / E-I-F REPRESENTATION
        ↓
MANAGER AGENT
   ┌────┼──────────┐
   ↓    ↓          ↓
VISION  NAVIGATION  ACTION/VERIFY
AGENT   AGENT       AGENT
   └────┼──────────┘
        ↓
POLICY / CAPABILITY GATE
        ↓
ANDROID ACTION
        ↓
READBACK
        ↓
STATE CHANGE VERIFICATION
        ↓
MEMORY / AUDIT TRACE
```

## Perception
The supplied MAS material combines `uiautomator dump` XML with `screencap -p`, extracting bounds, clickable/focusable state, `content-desc` and text. It also proposes centroid estimation from `bounds="[x1,y1][x2,y2]"` when structural information is incomplete. fileciteturn906file1L29-L38 fileciteturn906file1L40-L55

P105 treats XML and vision as complementary modalities:
- XML gives semantics and interaction affordances;
- screenshots provide visual context;
- OCR/visual grounding is fallback evidence;
- disagreement between modalities triggers re-observation rather than blind action.

## Agent topology

Specialists are used when their observation/action contracts differ materially:
- **Manager** — task decomposition and state machine.
- **Perception Agent** — XML/screenshot fusion.
- **Navigation Agent** — target resolution and next-state prediction.
- **Action Agent** — bounded Android interaction.
- **Verification Agent** — postcondition checking.
- **Recovery Agent** — timeout, stale UI and failed-action recovery.

Agents do not receive independent authority. The capability broker owns the action scope.

## Execution model

`OBSERVE → PARSE → PLAN → AUTHORIZE → ACT → WAIT FOR STATE PROPAGATION → READBACK → VERIFY → LOG`.

The source implementation explicitly includes a wait for asynchronous UI propagation after an action. fileciteturn908file10L427-L462

## Failure handling
The system must detect:
- missing/invalid XML;
- custom Canvas/Flutter-like views;
- stale hierarchy;
- visual ambiguity;
- asynchronous rendering;
- action rejection;
- device disconnect;
- permission loss.

Failure must cause re-observation, alternative authorized strategy or explicit task failure—not uncontrolled retries.

## Security
- ADB is treated as an explicitly authorized debugging channel.
- Accessibility capability is scoped to declared tasks.
- USB/debug transport is preferred where the deployment requires a controlled physical connection.
- No root requirement is assumed.
- No credential harvesting, lock-screen bypass or covert surveillance is part of P105.
- API keys remain external secrets and never belong in source-controlled task definitions.

## Verification
1. XML parser/property tests.
2. Screenshot/XML cross-modal grounding tests.
3. Coordinate/centroid accuracy tests.
4. Dynamic UI recovery tests.
5. Action postcondition tests.
6. Multi-agent disagreement tests.
7. stale-state rejection tests.
8. device disconnect/reconnect tests.
9. permission-boundary tests.
10. end-to-end benchmark on emulated and authorized physical devices.

## Maturity
Architecture baseline. The supplied documents provide implementation concepts and code witnesses; production reliability has not been independently reproduced.

## Derived from
P08, P37, P57, P61, P72, P86, P100.

## Primary source witnesses
- `Kod Systemu Wieloagentowego (MAS) dla Androida.PDF`
- `Kod Systemu Wieloagentowego (MAS) dla Androida(1).PDF`
- `Maksymalizacja systemu agentów w oparciu o źródła ...PDF`

---

## Knowledge evolution — batch 15 / 2026-10-08: P125 launcher boundary and Android capability separation

Source `AI Launchery Android_ Ranking 2025-2026.PDF` proposes contextual HOME organization and local AI. New P125 owns **selectable launcher HOME UX**; P105 remains responsible only for **authorized UI perception/action and postcondition validation**. Launcher status does not convey Usage Access, Accessibility Service rights, notifications, location or authority to launch apps autonomously.

`LauncherActuationBoundary`: an explicit user tap to launch an installed app via a declared MAIN/LAUNCHER component is allowed; an LLM's implicit prediction of user intention never triggers Accessibility actions, message sending or sensitive app navigation. Future P105 adapter requires separate user-granted platform permissions, scoped operation consent, typed capability token, dry-run, readback and audit. P125 MVP intentionally includes **no** special permissions or network API. Android/OEM implementation behavior unverified until SDK/emulator validation.

---

## 2026-10-08 — batch 18: public web source evolution

Googlebook adaptive Android article (2026-09-22) and Android tools/agents docs recommend window size classes, desktop freeform resizing, pointer/trackpad, keyboard, multi-instance, accessible touch/click targets. Extend AdaptiveUiEvidence {compact,medium,expanded,freeform_resize,split_window,keyboard_navigation,mouse_hover,talkback,rotation,restore_state}. Android agent skills are development aids, **not permissions to control the device**. No emulator or OEM install performed.

---

## 2026-10-09 — batch 22: Googlebook responsive launcher implemented

**Source:** Android Developers' Sept 22 2026 Googlebook adaptive article, already referenced in batch 18. Now P125 has an actual native Views reflow prototype (window's `onSizeChanged`, dp breakpoint >=840 and no new permission requests). P105 must treat `window_bounds, viewport_width_dp, focused_view_id, independent_device_readback, touch/keyboard/trackpad_mode` as **runtime observation** before authorizing any UI action. A changing window invalidates previously calculated click coordinates; re-read UI hierarchy and reauthorize rather than replay stale taps. `AdaptiveUiPostcondition`: width and density, orientation, focused control, observed target bounds after resize, source permission scope and error fallback. HOME/launcher role does not grant Accessibility or silent app actuation. No ADB control, Android emulator automation or desktop hardware test executed.
