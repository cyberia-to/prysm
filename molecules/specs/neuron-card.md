---
tags: prysm, cyb
crystal-type: pattern
crystal-domain: cyber
status: proposed-ui
---

Proposed UI contract under [composition](../../system/specs/composition.md). Layouts, ECS records and interactions below specify intended behavior, not shipped coverage.

[[neuron]] identity card molecule in [[prysm]]

A display of a protocol subject: domain-qualified reference, display address, optional name/image, and sourced karma/rank. Images and names are presentation records; they neither prove key control nor merge the subject with a robot. A robot can attach several neurons, and a card may inspect any permitted subject without attaching it.

## protocol role

molecule in $\mathcal{T}$. lives inside space zone content, search results, feeds, the [Cyberver view proposal](../../../aos/cyberver.md)

## sizing

fill × auto

$s_{min} = (10g, 6g)$

## structure

big:
```
glass [fill × auto, depth midground]
  stack horizontal [gap g]
    glass [fix(6g) × fix(6g), corner-radius 3g] — avatar image (circle)
    stack vertical [gap g/2]
      text [h3, name or alias]
      address [small]
      stack horizontal [gap g]
        counter [karma]
        counter [rank]
```

small:
```
glass [fill × fix(6g), depth midground]
  stack horizontal [gap g/2]
    glass [fix(4g) × fix(4g), corner-radius 2g] — avatar (circle)
    text [caption, name]
    counter [karma, micro]
```

## fold

$\mathcal{F}$:
- $l_1$ ($w_{min} = 25g$): big — avatar + name + address + karma + rank
- $l_2$ ($w_{min} = 15g$): small — avatar + name + rank
- $l_3$ ($w_{min} = 6g$): avatar image only

## emotion

The optional subject-image border reflects sourced karma under the declared network profile. It is independent of the robot's Avatar and never indicates key control or authorization. Compact folds retain the full subject/domain/network in their inspection target and accessible label.

## states

| state | visual | trigger |
|-------|--------|---------|
| default | card visible | — |
| hover | glass opacity +0.1, name underline | pointer over |
| active | navigate to neuron profile | tap |

state transitions: $150\text{ms}$ ease

## 3D

renders at membrane's $p_z$. display image faces the viewer (billboard)

## ECS

- Entity: neuron-card organelle
- Components:
  - `Sizing { width: Fill, height: auto }`
  - `NeuronDisplay { subject_ref, network, display_profile, name, image, karma, rank, evidence, freshness }`
  - `FoldSet { conformations }`
  - `Emotion { border_color }` — from karma
  - `TapAction { route: Neuron(subject_ref), network }` — inspection only
- System: `NeuronCardSystem` renders supplied subject projections; no key operation or attachment change
