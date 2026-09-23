---
tags: prysm, cyb, chroma
alias: navigation context
crystal-type: pattern
crystal-domain: cyber
status: proposed-ui
---

Proposed UI contract under [composition](../../system/specs/composition.md). Layouts, ECS records and interactions below specify intended behavior, not shipped coverage.

location chrome — top-left

This widget presents Now's context and the inspected typed destination. Its breadcrumb describes the active view and particle; it does not select an acting neuron. Now is the context organ under [anatomy](../../../cyb/anatomy.md); space is the area in which a view renders.

## protocol role

context is a molecule in the element tree $\mathcal{T}$. membrane = context zone of [[prysm/grid]], top-left. fix($25g$) × fix($6g$) on desktop, auto × fix($4g$) on mobile

## core function

orientation — the user can inspect the current destination and context. space renders the active world name, the current path inside it, and a breadcrumb trail back to root.

receives host navigation events when the active renderer changes. A UI event is not automatically a persisted or signed cyberlink.

## Proposed host events

| receives from | token | meaning |
|---------------|-------|---------|
| spacetime | locate | renderer switched — update breadcrumb |
| brain / memory | navigate | graph destination or file projection path changed |

| sends to | token | meaning |
|----------|-------|---------|
| com | context | current world context for command palette |

## sizing

| viewport | sizing |
|----------|--------|
| desktop | fix($25g$) × fix($6g$) |
| mobile ($\square_w \leq 96g$) | auto × fix($4g$) |

$s_{min} = (4g, 4g)$ — icon only

## structure

```
glass [fix(25g) × fix(6g), depth midground]
  vector [4g, destination icon]
  text [caption, destination label / particle title / world name]
```

breadcrumb trail shows: world → section → particle (if deep)

## fold

$\mathcal{F}$:
- $l_1$ ($w_{min} = 25g$): icon + full breadcrumb
- $l_2$ ($w_{min} = 10g$): icon + world name only
- $l_3$ ($w_{min} = 4g$): icon only (mobile)

## states

| state | visual |
|-------|--------|
| world active | world icon + name |
| deep path | icon + name + breadcrumb trail |
| root | icon + "cyb" only |

## interaction

tap → opens menu context (slide-out from left edge, z: 30)

## ECS

- Entity: context organelle
- Components:
  - `Sizing { width, height }`
  - `GridArea { name: "context" }`
  - `FoldSet { conformations }`
  - `DisplayContext { route, label }`
  - `Trigger::Tap { opens: menu_context }`
- System: `ContextSystem` reads current navigation state, writes `DisplayContext`

## Destination contract

`Route` is defined by [neuron navigation](../../../neuron/specs/navigation.md).
It carries View, Particle, Neuron or Prog plus an optional explicit network.
A view/tab has no subject key. Inspecting a neuron or prog does not attach, select
or execute it. Commander actions separately capture the controlled attachment,
binding revision and network; late responses retain that capture.
Legacy `cell://` targets require an exact mapping; the built-in `landing` target
is the robot view. An unknown origin never becomes a neuron or loaded program.
