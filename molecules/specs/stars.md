---
tags: prysm, cyb
crystal-type: pattern
crystal-domain: cyber
status: proposed-ui
---

Proposed UI contract under [composition](../../system/specs/composition.md). Layouts, ECS records and interactions below specify intended behavior, not shipped coverage.

pinned items molecule in [[prysm]]

a circular ring of [[prysm/vector]] icons arranged around the [[prysm/graph]] minimap. each icon = a pinned destination or other item bookmarked in the robot's configuration. Quick-access favorites are configured through settings/Soul; Avatar supplies visualization, not bookmark ownership or authority

## protocol role

molecule in the element tree $\mathcal{T}$. membrane = bottom-l zone of [[prysm/grid]]. stars + graph form a combined unit: graph in the center, star icons orbiting around it

## sizing

| viewport | sizing |
|----------|--------|
| desktop | fix($25g$) × auto — circular layout around graph |
| mobile ($\square_w \leq 96g$) | auto × fix($6g$) — linear horizontal row, max 4 |

$s_{min} = (4g, 4g)$ — one icon visible

## structure

desktop — circular ring around graph:
```
layer [fix(25g) × fix(25g)]
  --- graph minimap at center ---
  graph [fix(12g) × fix(12g), center]
  --- star icons arranged in circle around graph ---
  vector [4g, destination 1, position: angle 0°, radius 10g from center]
  vector [4g, destination 2, position: angle 30°, radius 10g]
  vector [4g, destination 3, position: angle 60°, radius 10g]
  vector [4g, destination 4, position: angle 90°, radius 10g]
  ...
```

icons are distributed evenly around the circle. radius = distance from graph center to icon center. as more icons are pinned, they spread around the full 360°

mobile — horizontal row:
```
stack horizontal [auto × fix(6g), gap g/2]
  vector [4g, pinned 1]
  vector [4g, pinned 2]
  vector [4g, pinned 3]
  vector [4g, pinned 4]
```

## fold

$\mathcal{F}$:
- $l_1$ ($w_{min} = 25g$): circular ring around graph, all icons visible
- $l_2$ ($w_{min} = 10g$): smaller circle, icons overlap at high count
- $l_3$ ($w_{min} = 0$, mobile): horizontal stack, max 4 icons, no graph

## interaction

- tap star icon → navigates to that destination
- long-press → unpin (remove from stars)
- drag to reorder on desktop (changes angle position in circle)
- graph minimap in center is independently interactive (tap to navigate)

## emotion

star icons inherit [[emotion]] from their destination: sense icon glows when unread messages, sigma when balance change. idle icons are neutral

## states

| state | visual change | trigger |
|-------|-------------|---------|
| default | icons at standard color in circle | — |
| hover | hovered icon scale 1.1×, label appears | pointer over star |
| active | icon scale 0.95× | tap |
| dragging | icon follows pointer along circular track | long-press + drag (desktop) |

state transitions: $150\text{ms}$ ease

## where in [[prysm/grid]]

| viewport | grid zone | position |
|----------|-----------|----------|
| desktop | bottom-l | circular layout combining stars + graph |
| mobile | stars area in bottom row | left of commander (linear) |

## 3D

stars render at frame $p_z$ ($\mathcal{U} = 10$). in 3D, star icons orbit the graph minimap as a ring in the xz-plane. icons face the viewer (billboard)

## ECS

- Entity: stars organelle
- Components:
  - `Sizing { width, height }`
  - `CircularLayout { radius, center }` — desktop: positions icons in circle
  - `Stack { direction, gap }` — mobile: linear fallback
  - `FoldSet { conformations }`
  - `PinnedItems { list of (route, icon_name, label, angle) }`
- Systems:
  - `StarsSystem` reads the robot's pinned typed destinations, computes angle positions
  - `StarsDragSystem` handles reorder on desktop (updates angle)

## Destination contract

`Route` is defined by [neuron navigation](../../../neuron/specs/navigation.md).
It carries View, Particle, Neuron or Prog plus an optional explicit network.
A view/tab has no subject key. Inspecting a neuron or prog does not attach, select
or execute it. Commander actions separately capture the controlled attachment,
binding revision and network; late responses retain that capture.
Legacy `cell://` targets require an exact mapping; the built-in `landing` target
is the robot view. An unknown origin never becomes a neuron or loaded program.
