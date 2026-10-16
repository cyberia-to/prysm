---
tags: prysm, cyb, core
crystal-type: pattern
crystal-domain: cyber
status: proposed-ui
---

Proposed UI contract under [composition](../../system/specs/composition.md). Layouts, ECS records and interactions below specify intended behavior, not shipped coverage.

the interaction protocol in [[prysm]]

interaction is how user input changes the element tree $\mathcal{T}$. the layout protocol (§4) produces coordinates. the [[prysm/emotion]] function produces colors. the interaction protocol produces state transitions. three independent systems, one interface

## input events

six primitive events. every interaction in [[cyb]] decomposes into these

| event | trigger | data |
|-------|---------|------|
| tap | finger down + up within $4g$ radius, < $300\text{ms}$ | $(p_x, p_y)$ |
| long-press | finger down, held $\geq 500\text{ms}$, no movement | $(p_x, p_y)$ |
| drag | finger down + movement $> g/2$ | $(p_{x_0}, p_{y_0}, p_{x_t}, p_{y_t}, \delta_x, \delta_y)$ |
| hover | pointer enters organelle bounds (pointer devices only) | $(p_x, p_y)$ |
| key | keyboard key pressed | $(keycode, modifiers)$ |
| scroll | wheel or two-finger gesture | $(\delta_x, \delta_y)$ |

tap and long-press are mutually exclusive: if hold exceeds $500\text{ms}$, tap is cancelled and long-press fires. drag cancels both if movement exceeds $g/2$ before release

## hit testing

given input coordinates $(p_x, p_y)$, find the target organelle:

$$\text{hit}(p_x, p_y) = e_i \quad \text{where} \quad z(e_i) = \max\{z(e_j) : (p_x, p_y) \in \text{bounds}(e_j)\}$$

the organelle with the highest $z$ containing the point receives the event. this follows from the urgency function $\mathcal{U}$ (§6.3): a modal at $z = 50$ captures input before the space zone at $z = 0$

when no interactive organelle contains the point: event is discarded

ECS: `HitTestSystem` reads `Position`, `OccupiedSize`, input coordinates. writes `EventTarget { entity }` 

## event propagation

events do not bubble. the target organelle handles the event or ignores it. no capture phase, no propagation chain

rationale: bubbling introduces implicit coupling between membrane and organelle. the layout protocol (§4.0) guarantees one-directional information flow (constraints down, sizes up). interaction follows the same principle — events arrive at target, state changes propagate through explicit systems, not implicit bubbling

exception: scroll. when the target organelle does not handle scroll, the event passes to the nearest ancestor with `Overflow::Scroll`. this is the only propagation — it follows the containment hierarchy, not an event chain

## focus

exactly one organelle holds focus at any time. focus determines which organelle receives key events

$$\text{focus}: \mathcal{T} \to e_i \quad |\quad \exists!\; e_i : \text{focused}(e_i) = \text{true}$$

focus transitions:
- tap on a focusable organelle → that organelle gains focus
- tab key → focus moves to the next organelle in `NavigationOrder` (§17.2 of layout protocol)
- shift+tab → previous in `NavigationOrder`
- escape → focus moves to commander ([Com](../../chroma/specs/com.md))

the commander is the focus root — pressing `/` from anywhere focuses the commander. this is the universal entry point

ECS: `Focus { active: bool }` component. `FocusSystem` reads input events, `NavigationOrder`, writes `Focus`

## state machine

every interactive organelle defines a finite state machine:

$$\mathcal{S} = (S,\; E,\; \delta,\; s_0)$$

$S$ — finite set of states. $E$ — set of input events the organelle handles. $\delta: S \times E \to S$ — transition function. $s_0 \in S$ — initial state

### common state patterns

three patterns recur across all [[prysm]] components:

### pattern 1: pressable

used by: [[prysm/button]], [[prysm/tabs]], [[prysm/pill]], [[prysm/toggle]], [[prysm/launcher]], [[prysm/menu]] items

```
states: { default, hover, active, disabled }
transitions:
  default  + hover   → hover
  hover    + leave   → default
  hover    + tap     → active → default (fires action)
  default  + tap     → active → default (fires action)
  disabled + *       → disabled (absorbs all events)
```

active is transient: $50\text{ms}$ visual feedback, then returns to default/hover. the action fires on transition to active, not on exit

### pattern 2: focusable

used by: [[prysm/input]], [Com](../../chroma/specs/com.md)

```
states: { idle, focus, disabled }
transitions:
  idle     + tap     → focus (gain focus, show cursor)
  focus    + escape  → idle (lose focus)
  focus    + tab     → idle (focus moves to next)
  focus    + key     → focus (input character)
  disabled + *       → disabled
```

### pattern 3: draggable

used by: [[prysm/slider]], [[prysm/stars]] (reorder), [[prysm/brain]] (graph nodes)

```
states: { idle, hover, dragging }
transitions:
  idle     + hover       → hover
  hover    + leave       → idle
  hover    + drag-start  → dragging
  dragging + drag-move   → dragging (update value/position)
  dragging + drag-end    → idle (commit value)
```

### composition

a component can compose multiple patterns. a [[prysm/slider]] is draggable (handle) + pressable (track tap to jump). composition is union of state machines running in parallel — no conflicts because they operate on different sub-organelles (handle vs track)

ECS: `InteractionState { current: State }` component per organelle. `InteractionSystem` reads input events, `InteractionState`, writes new `InteractionState` + fires `Action` events

## actions

state transitions produce actions — side effects outside the interaction protocol:

| action | produced by | effect |
|--------|-------------|--------|
| navigate | tap on link, menu item, star, tab | router inspects a typed View, Particle, Neuron or Prog destination |
| submit | tap confirm button, enter key in input | Com emits a query or captured action intent; host policy governs effects |
| toggle | tap on toggle, pill (on/off mode) | boolean state flips |
| expand | tap on context, avatar, collapsible group | fold/unfold reveals/hides content |
| drag-commit | drag-end on slider, star reorder | value or position written to state |
| focus-change | tap on input, tab key, escape | focus moves between organelles |

Actions are events emitted by `InteractionSystem` and consumed by explicit host adapters. Navigation carries a Route, not signing authority. Before an effect the host binds subject, attachment revision, network, payload and grant; Ward evaluates current permission and Vault performs only the scoped key operation. A tap, focus change, model suggestion or callback cannot bypass that boundary. Selection changes cannot reauthor an outstanding operation.

## visual feedback

state changes produce visual feedback through the [[prysm/emotion]] function and motion convention (§16 of layout protocol):

| state change | visual | timing |
|-------------|--------|--------|
| → hover | scale 1.1× or glow increase | $150\text{ms}$ ease |
| → active | scale 0.95× (pressed feel) | $50\text{ms}$ ease |
| → focus | saber underline changes to blue (#00acff) | $150\text{ms}$ ease |
| → dragging | element follows pointer | immediate (0ms, locked to input) |
| → disabled | opacity 0.5, gray color | $150\text{ms}$ ease |

drag feedback is immediate (no easing) — the element must track the pointer with zero perceived latency. all other feedback uses the standard motion convention ($150\text{ms}$ ease)

## hotkeys

global keyboard shortcuts — handled before focus-based key dispatch:

| key | page | action |
|-----|------|--------|
| / | all | focus commander |
| tab | all | next focusable organelle |
| shift+tab | all | previous focusable organelle |
| enter | all | activate focused organelle (equivalent to tap) |
| escape | all | lose focus → commander |
| f | brain | toggle fullscreen |

These proposed hotkeys are configured through the [settings view](../../chroma/specs/settings.md). They do not assert that each binding is currently implemented.

ECS: `HotkeyMap { list of (key, modifier, action) }`. `HotkeySystem` reads key events before `FocusSystem` — if a hotkey matches, the event is consumed and does not reach focus dispatch

## the interaction function

$$\text{interact}(\mathcal{T},\; e_{input}) \;\to\; \{(e_i,\; s'_i,\; a_i)\}$$

$\mathcal{T}$ — element tree (with current states). $e_{input}$ — input event. output: for each affected organelle $e_i$, its new state $s'_i$ and optional action $a_i$

the interaction function is deterministic: same tree + same states + same input → same output. this follows from $\delta$ being a pure function and hit testing being deterministic (highest $z$ wins)

## ECS system order

interaction systems execute after layout, before render:

```
HitTestSystem → HotkeySystem → FocusSystem → InteractionSystem → ActionSystem
```

all five run in sequence within one frame. total cost: $\mathcal{O}(n)$ for hit test (spatial index), $\mathcal{O}(1)$ for the rest (single event, single target)
