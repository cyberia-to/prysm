---
tags: prysm, cyb, chroma
alias: time view
crystal-type: pattern
crystal-domain: cyber
status: proposed-ui
---

Time view and history chrome.

Proposed UI contract under [composition](../../system/specs/composition.md).
Layouts and ECS records below specify intended behavior, not shipped coverage.

## Ownership

[Time](../../../cyb/parts/time.md) composes Log ← Now → Plan: retained history,
present context and standing/deferred intent in one screen. Cybergraph/BBG stores
authoritative history. Log renders that history; neither this widget nor a
session-local event list is another ledger.

The robot can inspect history across several attached neurons. Every event keeps
its author, identity domain, network, source, effect status and disclosure scope.
Changing a filter does not merge subjects or change the author of pending work.
Plan updates go through the host's captured authority and current Ward policy.

## Host projections

| source | projection | meaning |
|--------|------------|---------|
| Log | retained events | render authored actions, attempts and outcomes |
| Now | context | current inspected particle/view and navigation state |
| Plan | scheduled intent | display standing orders and deferred work |
| State | network evidence | distinguish reported state, verified anchors and finality |

Navigation events may be local presentation state. Persisting one requires an
explicit record profile; a renderer switch is not automatically a signed cyberlink.
A UI timestamp does not establish consensus order or finality.

## Widget layout

```text
glass [fix × fix(bottom-right), depth overlay]
  stack vertical [align center]
    vector [clock icon]
    text [micro, last known event timestamp]
    counter [micro, unread events]
```

Tapping the widget navigates to `cyb://view/time`. It does not start a prog,
attach a neuron or sign an operation.

## Full view

```text
glass [fill × fill, depth background]
  tabs [Log | Now | Plan]
  table [sortable, history projection]
    columns: subject/network | status | type | observed time | order/evidence | action
  context [current inspected particle]
  schedule [standing and deferred intent, retained outcome]
```

Filters include attachment, network, prog/task and session; absent or undisclosed
history stays explicit. Failure, cancellation and unknown outcomes are visible.
A restored screen reads retained graph records rather than reconstructing history
from previously displayed rows.

## Time domains

| field | meaning |
|-------|---------|
| observed timestamp | source-reported or local wall-clock time, possibly absent |
| runtime commit index | local durable transaction order |
| SignalChain step | order within one neuron's signal chain |
| block height / finality evidence | declared network's ordering and confirmation profile |

These fields are not interchangeable. Relative time (“37 sec ago”) is a display
over a known timestamp; it must not invent a Unix timestamp for every cyberlink.
Machine-time formatting can use UTC days since the Unix epoch without claiming
that wall-clock order is consensus order.

## Proposed ECS adapter

`TimeProjection` contains typed routes, scoped history rows, Now context and Plan
records. `TimeViewSystem` renders it; local sorting changes presentation only.
Editing a plan emits an exact host intent, and its result retains the captured
subject and network even after navigation changes.
