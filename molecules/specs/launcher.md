---
tags: prysm, cyb
crystal-type: pattern
crystal-domain: cyber
status: proposed-ui
---

Proposed UI contract under [composition](../../system/specs/composition.md). Layouts, ECS records and interactions below specify intended behavior, not shipped coverage.

destination launcher molecule in [[prysm]]

the destination icon in the top-left corner of [[prysm/grid]] (inside the [Now/context widget](../../chroma/specs/space.md)). by default shows the active destination icon. tap opens a dropdown menu listing available destinations — each item is an icon + text label. menu extends down from the icon, max length to the middle of the screen

## protocol role

launcher is a molecule in the element tree $\mathcal{T}$. lives inside the [Now/context widget](../../chroma/specs/space.md). the icon is always visible. the menu is an overlay at $\mathcal{U} = 30$ (interrupting)

## sizing

| element | sizing |
|---------|--------|
| icon (collapsed) | fix($4g$) × fix($4g$) |
| menu (expanded) | fix($25g$) × auto, max height = $\square_h / 2$ |

$s_{min} = (4g, 4g)$ — icon only

## structure

collapsed (default):
```
vector [4g, active destination icon]
```

expanded (on tap):
```
glass [fix(25g) × auto, depth foreground, max height square_h/2, overflow scroll]
  stack vertical [gap g/2]
    stack horizontal [gap g]
      vector [2g, oracle icon]
      text [body, "Oracle"]
    stack horizontal [gap g]
      vector [2g, brain icon]
      text [body, "Brain"]
    stack horizontal [gap g]
      vector [2g, portal icon]
      text [body, "Portal"]
    stack horizontal [gap g]
      vector [2g, sense icon]
      text [body, "Sense"]
    stack horizontal [gap g]
      vector [2g, sigma icon]
      text [body, "Sigma"]
    ...
```

## fold

launcher does not fold — icon is always $4g$ × $4g$. menu appears/disappears, does not fold

## emotion

active destination icon can carry [[emotion]] reflecting the destination's state (e.g. sense icon glows when unread messages). inactive items in menu are neutral

## states

| state | visual | trigger |
|-------|--------|---------|
| collapsed | icon only | default |
| expanded | menu drops down from icon | tap on icon |
| hover (menu item) | item text brightens | pointer over item |
| active (menu item) | navigate to selected destination | tap on item |

state transitions: menu slide down $150\text{ms}$ ease

## interaction

- tap icon → toggle menu (expand/collapse)
- tap menu item → navigate to that destination, menu closes
- tap outside menu → close menu
- active destination is highlighted in menu list

## where in [[prysm/grid]]

inside the [Now/context widget](../../chroma/specs/space.md) (row 1, col 1). icon is part of context. menu is overlay at z: 30, drops down from context zone

## 3D

icon renders at frame $p_z$ ($\mathcal{U} = 10$). menu renders at interrupting $p_z$ ($\mathcal{U} = 30$) — closer to the viewer than the frame

## ECS

- Entity: launcher organelle
- Components:
  - `Sizing { width: Fix(4), height: Fix(4) }` — icon
  - `DestinationList { list of (route, icon, label) }` — available destinations
  - `ActiveDestination { route }` — currently active
  - `Visibility { collapsed | expanded }` — menu state
  - `Emotion { color }` — from active destination state
- Systems:
  - `LauncherSystem` handles tap to toggle, tap on item to navigate
  - `LauncherEmotionSystem` reads destination states, writes icon emotion

## Destination contract

`Route` is defined by [neuron navigation](../../../neuron/specs/navigation.md).
It carries View, Particle, Neuron or Prog plus an optional explicit network.
A view/tab has no subject key. Inspecting a neuron or prog does not attach, select
or execute it. Commander actions separately capture the controlled attachment,
binding revision and network; late responses retain that capture.
Legacy `cell://` targets require an exact mapping; the built-in `landing` target
is the robot view. An unknown origin never becomes a neuron or loaded program.
