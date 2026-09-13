# P120 Extension — CALL-E Current API / Reliability Hardening

**Date:** 2026-09-13  
**Parent:** P120 — AegisFleet

## Trigger
Current CALL-E Developer API documentation and the user's AegisFleet implementation were compared. The current provider surface is more explicit than the earlier source snapshot.

## Updated provider model

Support two execution modes:

### One-shot Calls
`POST /v1/calls` → asynchronous CallTask → `GET /v1/calls/{call_id}` / events → terminal result.

### Published Goals / Goal Runs
`GET /v1/goals` → select a previously published Goal by stored `goal_id` → `POST /v1/goals/{goal_id}/runs` → Goal Run lifecycle.

Goal Run requests are bound to the published RunSpec; callers cannot replace or weaken the published contract.

## Reliability hardening

- persist a stable `Idempotency-Key` before provider submission;
- never generate a new idempotency key for a network retry of the same logical operation;
- persist provider event IDs before applying side effects;
- reject duplicate/replayed events;
- preserve raw provider failure codes without branching business logic on undocumented values;
- treat cursors as opaque;
- store the intended Goal ID rather than executing the first listed Goal;
- distinguish provider acceptance from terminal completion;
- distinguish terminal completion from verified logistics outcome.

## Result contract

Use `yes | no | unknown` rather than a bare boolean for business decisions. Use `unknown` whenever terminal evidence is insufficient.

`structured_result + summary + confidence + evidence` remains an extraction package, not independent truth.

## Updated AegisFleet pipeline

`INCIDENT → GOAL → PLAN → POLICY → AUTHORIZATION → CALL/GOAL RUN → EVENT → RESULT → EVIDENCE → HUMAN/ERP DECISION → READBACK → VERIFIED POSTCONDITION`

## Simulation boundary

The existing deterministic local simulator remains valuable. It must share the same validation path as live execution but must be labelled synthetic. A simulator cannot create evidence about a real driver, carrier, ETA or route.

## Production gates

Before any automated ERP mutation:
1. provider terminal state verified;
2. result schema validated;
3. evidence sufficient;
4. policy allows the mutation;
5. business entity correlation exact;
6. approval requirements satisfied;
7. ERP write idempotent;
8. ERP readback confirms postcondition.
