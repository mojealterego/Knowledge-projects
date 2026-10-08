# Adult domestic-abuse assistance — safeguarding and evidence, 2026-10-08

**Source:** Agnieszka Filipek, `Wspomaganie człowieka dorosłego w sytuacji przemocy w rodzinie`, doctoral dissertation, University of Białystok, 2017, 394 PDF pages. Not a current clinical protocol, present-day hotline directory, legal notice template or risk-scoring model.

## Design-relevant study provenance
- Qualitative/theoretical chapters distinguish psychological, physical, sexual and economic abuse, consequences, active/passive coping, social work, police, psychological assistance and institutional support.
- Own empirical research: diagnostic questionnaire survey among 152 adult survivors enrolled in `Niebieskie Karty` in three cities; purposive selection (97 Białystok, 31 Łomża, 24 Suwałki); May–September 2016. This sample cannot estimate nationwide prevalence or individual risk.
- The study emphasizes confidentiality, participants' voluntary participation and no forced disclosure. Cooperation or passivity under coercion **cannot be interpreted as consent**.
- Historical law and organizational arrangements must be verified against **current** Polish law and actual municipal services before any service navigator is released. A 2017 thesis is insufficient.

## P124 design boundaries
1. **User-controlled information service**, not a diagnostic profiler or surveillance system. No covert recording, monitoring of partner, inference of guilt/abuse from a third person's behavior, or automated report to police/social services.
2. **Safe discovery and exit:** minimal history, explicit control to erase a session, no tracking pixels, no push reminders or saved notifications by default, and review of shared-device/threat-model exposure before UX release. Do not promise that a browser's quick-exit feature deletes browsing history.
3. **Risk-aware language:** distinguish source facts and options from personal legal/clinical advice; do not require disclosure of a person's story to read support options.
4. **Independent current resource verification:** a professional-reviewed directory of institution/service, geographic scope, contact method, checked-as-of timestamp, eligibility, accessibility and fallback when outdated. No invented phone number.
5. **Human oversight:** healthcare/legal/social-work review, trauma-informed wording, safe consent, multilingual/accessibility support, confidential escalation by the person's explicit request except legal mandatory-reporting rules independently verified.
6. **No direct software actuation:** the app cannot make contact, call external services, collect personal evidence or share location as a result of a model suggestion alone.
7. **Evidence provenance:** dated sources, jurisdiction and disclaimers. Sample-specific findings never used to rank individual survivors or alleged abusers.

## Proposed minimal records
```yaml
ServiceReference:
  jurisdiction: PL
  service_type: emergency|social_work|psychological|legal|shelter|ngo
  official_source_url: null
  contact_information: null
  verified_as_of: null
  reviewed_by_qualified_professional: false
  eligibility: null
  reliability_status: not_checked
UserSession:
  persisted_by_default: false
  telemetry_enabled: false
  outbound_contacts_allowed: false
  event_log_personal_data: forbidden
```

## Mandatory product gate
Professional safeguarding review, current primary-source legal verification, threat modeling for coercive partners, privacy impact assessment, usability testing by appropriate experts and release-owner approval. At present **P124 is an architecture/evidence proposal only**; no support service, victim-data collection or public release was built.
