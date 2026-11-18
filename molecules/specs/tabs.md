---
tags: prysm, cyb
crystal-type: pattern
crystal-domain: cyber
status: proposed-ui
---

Proposed UI contract under [composition](../../system/specs/composition.md). Layouts, ECS records and interactions below specify intended behavior, not shipped coverage.

section navigation molecule in [[prysm]]

a horizontal row of selectable items. one active, the rest idle. switches between sections within a view. glass + ion (per tab) + saber (active underline)

## protocol role

molecule in $\mathcal{T}$. lives inside space zone, within view content

## sizing

fill × fix($6g$)

$s_{min} = (15g, 4g)$ — minimum 3 tabs visible

## structure

```
glass [fill × fix(6g), depth midground]
  stack horizontal [gap 0]
    stack vertical [per tab]
      vector [2g, tab icon]
      text [micro, tab label]
      saber [horizontal, g/4, glow] — active tab only
```

## fold

$\mathcal{F}$:
- $l_1$ ($w_{min} = 30g$): 5 tabs, icon + label
- $l_2$ ($w_{min} = 15g$): 3 tabs, icon only
- $l_3$ ($w_{min} = 8g$, mobile): 3 tabs as bottom bar, icon only

## emotion

active tab saber glow = [[emotion]] of the active section (default #00fe00 green)

## states

| state | visual change | trigger |
|-------|-------------|---------|
| default | idle tabs #777777, active tab #ffffff + saber underline | — |
| hover | hovered tab text #d7d7d7 | pointer over tab |
| active | saber slides to new tab ($150\text{ms}$), tab text #ffffff | tap |

state transitions: saber slide $150\text{ms}$ ease

## 3D

renders at membrane's $p_z$

## ECS

- Entity: tabs organelle
- Components:
  - `Sizing { width: Fill, height: Fix(6) }`
  - `TabItems { list of (icon, label, section_id) }`
  - `ActiveTab { index }`
  - `FoldSet { conformations }`
- System: `TabsSystem` handles tap, updates `ActiveTab`, animates saber
