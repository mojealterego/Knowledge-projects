# Visual influence, neurocognitive claims and defensive UI safety — 2026-10-08

Source: `Wpływ Wizualny na Ludzki Umysł.pdf` (report: ACC, visual cortex, attention/priming, optical illusions, flicker, AI image generation, legal/ethical constraints).
Owners: P36 / P40 / P43 Influence Literacy, P57 Multimodal Interface, P113 Visual Prompt Compiler.

## Source-derived research map
The PDF proposes links among attention, visual salience, anticipation/framing, hypnotic susceptibility, geometric illusions, priming, repetition, emotional imagery and visual persuasion. It distinguishes normal persuasion from "absolute mind control" fantasies. Some sections overstate causal neuroscience: changes in a studied brain region cannot justify an arbitrary claim that a design "turns off critical thinking" or commands behavior against will. No validated operational mind-control protocol is established.

## Defensive threat taxonomy
| Category | Observable product-design signal | Safety response |
|---|---|---|
| Covert persuasion | undisclosed advertising, deliberate concealment of material facts | disclosure, reviewer escalation |
| Consent / opt-out friction | asymmetric choices, deceptive button hierarchy | equivalent controls, reversibility test |
| Cognitive overload | excessive density, motion and time pressure | reduce animation, offer accessible static presentation |
| Emotional vulnerability targeting | personalization based on inferred distress | minimize signals, protect user autonomy |
| Photostimulation | flashing/rapid high-contrast patterns, repeated motion | WCAG-aligned motion/flash checks; photosensitive seizure risk test |
| AI image persuasion | provenance missing, synthetic characters shown as people | disclosure, provenance, user control |
| Subliminal-effect claims | promised reliable behavioral control via hidden frames | reject as unverified efficacy claim; flag for ethics review |

## Evidence & design invariants
- `VisualStimulusEvent{asset_hash,modality,display_window,motion_class,flashing_risk,disclosure,consent,accessibility_review,source_refs}`.
- Evaluate **the artifact/UI**, not whether an identified viewer is suggestible or has a medical condition.
- Record perception / attention changes separately from claims about action, intention or persuasion.
- No hidden behavioral manipulation, no compulsive engagement optimization, no induced hallucination/strobe experiments on unwitting subjects.
- Photostimulation testing in safe simulation and published accessibility criteria; no hazardous flash presets.
- When assessing AI Act, advertising, or accessibility law: verify provisions and dates against current official legal text (not the PDF's summary).

## Test plan
1. Automated animation/flash audit plus human accessibility review.
2. Contrast, reduced-motion, keyboard and alternative text checks.
3. Inspect whether user can distinguish advertisement from organic content.
4. Measure opt-out parity and reversibility in consent UX.
5. Audit prompts/creative briefs for covert influencing or exploitation claims.
6. Validate claims with cited, controlled evidence rather than graphic-generation success.

## Project evolution
P36 gets a `VisualInfluenceRisk` schema and UX policy tests. P57/P113 receives disclosure/provenance hooks for synthetic imagery. P40/P43 gains a literacy module that teaches pattern recognition rather than engineering coercive stimuli.

Status: research-derived taxonomy, **not experimentally proven effectiveness of the PDF's stronger brain-influence assertions**.
