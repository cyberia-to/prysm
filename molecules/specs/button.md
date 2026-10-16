---
tags: prysm, cyb
crystal-type: pattern
crystal-domain: cyber
status: proposed-ui
---

Proposed UI contract under [composition](../../system/specs/composition.md). Layouts, ECS records and interactions below specify intended behavior, not shipped coverage.

call-to-action molecule in [[prysm]]

the primary interaction primitive. a button emits a user intent; autonomous and scheduled neuron actions need no button. a transparent region bounded by vertical [[prysm/saber]] lines with glow radiating inward

## protocol role

button is a molecule in the element tree $\mathcal{T}$. its membrane is whatever zone or molecule contains it. button does not have a dedicated grid zone — it lives inside [Com](../../chroma/specs/com.md), [[prysm/bar]], [[prysm/display]], and other molecules

## sizing

height: fix($6g$)

width computed from content:

$$W = \underbrace{g/4}_{saber} + \underbrace{3g}_{glow} + \underbrace{2g}_{pad} + \text{text\_width} + \underbrace{2g}_{pad} + \underbrace{3g}_{glow} + \underbrace{g/4}_{saber}$$

$W = 10.5g + \text{text\_width}$

$s_{min} = (6g, 6g)$ — shortest label (one icon) still gets $2g$ padding each side

padding from glow edge to text is always $2g$ regardless of text length. short text = narrow button. long text = wide button. breathing room is constant

## structure

```
saber [vertical, g/4, glow inward 3g, emotion color]
  — pad 2g —
text [body, label, emotion color]
  — pad 2g —
saber [vertical, g/4, glow inward 3g, emotion color]
```

no glass background — the button is transparent. sabers and glow define the boundaries

## fold

$\mathcal{F}$:
- $l_1$ ($w_{min} = 12g$): icon + label + sabers
- $l_2$ ($w_{min} = 6g$): icon only + sabers

## emotion

text color and saber glow carry [[emotion]] of the action:

| action type | text color | saber glow |
|-----------|-----------|------------|
| neutral (navigate, open) | #ffffff | #ffffff |
| confirm (sign, stake, claim) | #00fe00 | #00fe00 |
| danger (delete key, burn tokens) | #ff0000 | #ff0000 |
| caution (large transfer) | #fcf000 | #fcf000 |
| disabled | #4b4b4d | none |

White denotes a neutral action category, not a safety or authorization guarantee. A red button demands attention. destructive actions are never the leftmost button

## states

| state | visual change | trigger |
|-------|-------------|---------|
| default | sabers white/emotion, glow at $3g$ | — |
| hover | glow-spread increases to $4g$ | pointer over button |
| active | glow pulses once, scale 0.95× | tap/click |
| disabled | sabers dim to #4b4b4d, no glow, text gray | action unavailable |
| loading | glow animates (breathe) | action in progress |

state transitions: $150\text{ms}$ ease

## placement

- primary actions: inside [Com](../../chroma/specs/com.md) (commander) — sign, send, confirm
- secondary actions: inside [[prysm/bar]], [[prysm/display]], [[prysm/neuron-card]]
- destructive actions: never first or leftmost — always require deliberate reach
- on mobile ($\square_w \leq 96g$): buttons stretch to fill when inside commander
- composition of multiple buttons (side by side with shared sabers) is view-level — not defined here

## 3D

button renders at the same $p_z$ as its membrane. in 3D, button sabers glow in the xy-plane. button faces the viewer (billboard)

## ECS

- Entity: button organelle
- Components:
  - `Sizing { width, height: Fix(6) }`
  - `ButtonVariant { default }`
  - `ButtonLabel { text }`
  - `ButtonIcon { glyph: Option }`
  - `Emotion { color }` — determines text + saber glow
  - `FoldSet { conformations }`
  - `TapAction { callback }`
- System: `ButtonSystem` reads tap events, emits typed `TapAction` intent and manages visual transitions. The host captures subject/network/payload and checks Ward/Vault policy before effects; a callback cannot inherit authority from button appearance
