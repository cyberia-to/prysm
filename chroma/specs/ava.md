---
tags: prysm, cyb, chroma
alias: who, avatar, identity
crystal-type: pattern
crystal-domain: cyber
---

avatar chroma — top-right

**who**: identity. shows the active [[neuron]]'s avatar, name, and connection state. gateway to [[settings]] and the full [[robot]] profile

## protocol role

avatar is a molecule in the element tree $\mathcal{T}$. membrane = avatar zone of [[prysm/grid]] (row 1, col 3). fix($25g$) × fix($6g$) on desktop, auto × fix($4g$) on mobile. $\mathcal{U} = 10$ (persistent)

## core function

ava is the identity anchor. the neuron's avatar icon is always visible here. tapping opens the robot profile or the settings overlay. connection state (connected / locked / offline) is reflected in ava's visual.

sends `(ava, sigma, identity, …)` when identity changes so sigma can load the correct balances. receives `(sigma, ava, resource, …)` to display inline balance summary.

## cyberlinks

| receives from | token | meaning |
|---------------|-------|---------|
| sigma | resource | show inline balance next to avatar |
| any | identity-change | re-render avatar for new neuron |

| sends to | token | meaning |
|----------|-------|---------|
| sigma | identity | neuron changed — reload balances |
| spacetime | switch-renderer | open robot or settings in space zone |

## sizing

| viewport | sizing |
|----------|--------|
| desktop | fix($25g$) × fix($6g$) |
| mobile ($\square_w \leq 96g$) | auto × fix($4g$) |

$s_{min} = (4g, 4g)$ — avatar image only

## structure

desktop:
```
glass [fix(25g) × fix(6g), depth midground]
  stack horizontal [gap g/2]
    glass [fix(4g) × fix(4g), corner-radius 2g] — avatar image (circle)
    stack vertical
      text [caption, name or alias]
      text [micro, truncated address or karma]
```

mobile:
```
glass [fix(4g) × fix(4g), corner-radius 2g] — avatar image only
```

tap avatar → open [[robot]] in spacetime  
tap gear icon → open [[settings]] in spacetime

## fold

$\mathcal{F}$:
- $l_1$ ($w_{min} = 25g$): image + name + address/karma
- $l_2$ ($w_{min} = 12g$): image + name
- $l_3$ ($w_{min} = 4g$): image only (mobile)

## emotion

avatar border reflects the neuron's [[karma]] state:

| karma | border color |
|-------|-------------|
| high | #00fe00 (green) |
| medium | #ffffff (white, no border) |
| low | #ff0000 (red) |

## states

| state | visual | trigger |
|-------|--------|---------|
| connected | avatar + green pill | — |
| locked | avatar + yellow pill | — |
| no identity | placeholder icon + "connect" | — |
| offline | avatar + red dot | — |
| hover | glass opacity +0.1, name underline | pointer over avatar |
| active | scale 0.95× | tap |

state transitions: $150\text{ms}$ ease

## variants

| variant | use |
|---------|-----|
| grid zone | top-right of [[prysm/grid]] — primary identity display |
| 2-line | inside messages, comments — compact attribution |
| chooser | in menu avatar — selectable identity with radio indicator |

## 3D

renders at frame $p_z$ ($\mathcal{U} = 10$). avatar image faces the neuron (billboard)

## ECS

- Entity: avatar organelle
- Components:
  - `Sizing { width, height }`
  - `GridArea { name: "avatar" }`
  - `FoldSet { conformations }`
  - `NeuronIdentity { address, name, karma, avatar_cid }`
  - `Trigger::Tap { opens: menu_avatar }`
  - `Emotion { border_color }` — from karma
- System: `AvatarSystem` reads active neuron identity, writes `NeuronIdentity` and `Emotion`
