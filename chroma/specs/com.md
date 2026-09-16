---
tags: prysm, cyb, chroma
alias: command, commander
crystal-type: pattern
crystal-domain: cyber
status: proposed-ui
---

Proposed UI contract under [composition](../../system/specs/composition.md). Layouts, ECS records and interactions below specify intended behavior, not shipped coverage.

commander chrome — bottom-center

Com is the user's input surface: buttons, decisions, questions and commands. Scheduled and autonomous work can enter through Plan/Soma as well; every effect still passes the same authority boundary.

## protocol role

com is a molecule in the element tree $\mathcal{T}$. membrane = commander zone of [[prysm/grid]] (row 3, col 2). fill × fix($6g$). $\mathcal{U} = 20$ (active)

## core function

com emits typed intent. Before dispatch the host captures the acting attachment, subject, binding revision, network, prog/invocation or native caller, exact payload, grant and resource reservation. Ward checks current authority and Vault performs scoped key operations. Input focus, suggested text and button color are not authorization.

com adapts to context: the active world determines which commands are available, which suggestions appear in [[ad]], and what the placeholder text says.

## command palette

Proposed shortcut: `⌘K` opens the palette. Type anything:

- a query → requests graph search or Soma assistance
- a page name → resolves a typed view/particle destination
- a command → prepares a typed action for current policy evaluation
- a CID → resolves the [[particle]]
- a qualified subject address → inspects the neuron; ambiguous domains require explicit resolution

fuzzy matching. recent commands. context-aware suggestions from current view.

## actions

| action | what happens |
|--------|-------------|
| link | create a [[cyberlink]] between two [[particles]] |
| send | transfer [[tokens]] via [Sigma](sigma.md) |
| sign | request scoped signing of a captured, authorized payload |
| publish | push a [[particle]] to the [[cybergraph]] |
| stake | delegate to a subnet |
| ask | submit a read/query or Soma task under its declared policy |
| navigate | inspect a typed destination without attaching or executing it |

actions may compose as search → select → link → publish. Each effect retains its captured authority, result and failure state; a chain of UI steps is not an atomic transaction or a blanket grant.

## Proposed keyboard bindings

- `⌘K` command palette
- `/` focus Com/search
- `⌘L` new [[cyberlink]]
- `⌘S` request the signing flow for the captured pending transaction
- `⌘P` publish current [[particle]]
- `Tab` cycle between view controls/tabs according to focus rules
- `Esc` back / close / cancel

## voice and text

Com accepts natural-language intent. Soma interprets it under Soul configuration; Voice supplies speech input. For example, “stake 100 CYB on subnet 3” can produce a delegation proposal, which still needs an explicit subject/network/asset profile and the host authorization path. Model text does not sign or select an identity.

## Proposed host events

These events are local adapter messages unless a declared persistence/publication profile turns them into authored records.

| sends to | token | meaning |
|----------|-------|---------|
| spacetime | submit | user submitted text or command |
| spacetime | switch-renderer | replace active renderer |
| ad | submit (partial) | partial text → suggestion request |
| sigma | send | token transfer initiated |
| log/time | record | display the retained command and effect outcome from Cybergraph/BBG |

| receives from | token | meaning |
|---------------|-------|---------|
| ad | hint | insert suggestion into placeholder |
| spacetime | context | current world for palette adaptation |

## sizing

| viewport | sizing |
|----------|--------|
| desktop | fill × fix($6g$) |
| mobile ($\square_w \leq 96g$) | fill × fix($6g$) (same) |

$s_{min} = (20g, 6g)$

## structure

```
glass [fill × fix(6g), depth midground]
  stack horizontal [gap g/2]
    vector [2g, back arrow]
    vector [2g, forward arrow]
    input [fill × fix(4g), with saber underline]
    button [sign action, with saber frame]
```

## fold

$\mathcal{F}$:
- $l_1$ ($w_{min} = 40g$): back + forward + input + sign button + context actions
- $l_2$ ($w_{min} = 20g$): input + sign button only
- $l_3$ ($w_{min} = 10g$): sign button only

## contextual adaptation

| active destination | placeholder | primary action |
|------------|-------------|----------------|
| graph search | "ask the cybergraph" | Search |
| terminal | "enter command" | Run |
| sense/@ | "send message" | Send |
| sense/llm | "ask the model" | Ask |
| sigma/send | "enter recipient" | Send |
| sigma/neurons | "attach, observe, or explicitly create" | Review |

content replacement within the same conformation — structure stays, labels and available actions change per destination

## emotion

saber underline of the input field carries [[emotion]]:

| state | glow-color | trigger |
|-------|-----------|---------|
| idle | #ffffff (white) | no activity |
| typing | #00fe00 (green) | user is entering text |
| error | #ff0000 (red) | invalid input, failed transaction |
| signing | #00fe00 (green, pulsing) | [[cyberlink]] being signed |
| success | #00fe00 (green, fades) | action completed |

## states

| state | visual | trigger |
|-------|--------|---------|
| default | input empty, saber white | — |
| focus | cursor visible, saber glow intensifies | tap input |
| typing | text appears, saber green | keystrokes |
| submitting | loading state on sign button | action in progress |
| disabled | all dims to #4b4b4d | no connection |

state transitions: $150\text{ms}$ ease

## 3D

commander renders at fixed $p_z$ ($\mathcal{U} = 20$) — does not recede with gravity. always within reach

## ECS

- Entity: com organelle
- Components:
  - `Sizing { width: Fill, height: Fix(6) }`
  - `GridArea { name: "commander" }`
  - `FoldSet { conformations }`
  - `ActiveDestination { route }`
  - `InputState { text, focused, emotion }`
  - `SaberGlow { color, spread }`
- Systems:
  - `ComContextSystem` reads `ActiveDestination`, updates placeholder and available actions
  - `ComInputSystem` handles focus, typing, submission
  - `ComEmotionSystem` reads input state and action result, writes `SaberGlow`

## Destination contract

`Route` is defined by [neuron navigation](../../../neuron/specs/navigation.md).
It carries View, Particle, Neuron or Prog plus an optional explicit network.
A view/tab has no subject key. Inspecting a neuron or prog does not attach, select
or execute it. Commander actions separately capture the controlled attachment,
binding revision and network; late responses retain that capture.
Legacy `cell://` targets require an exact mapping; the built-in `landing` target
is the robot view. An unknown origin never becomes a neuron or loaded program.
