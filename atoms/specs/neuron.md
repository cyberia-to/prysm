---
tags: prysm, cyb
crystal-type: pattern
crystal-domain: cyber
---

identity and agency atom in [[prysm]]

the primitive for any actor in the [[cybergraph]] — human, AI, sensor, autonomous agent. renders a domain-qualified public subject reference. a leaf in the element tree (leaf type: address). the neuron atom is pure identity — it knows the address and nothing else. molecules (neuron-card, avatar, address) compose it into richer displays

## protocol role

neuron is a leaf in the element tree $\mathcal{T}$ (§7 of [[prysm/layout]]). leaf type: address. it has no sub-organelles. its membrane constrains it, it occupies space based on address format and size, membrane places it

## sizing

all values in spatial quanta $g$

| parameter | sizing type | values | default |
|-----------|-----------|--------|---------|
| id | — | SubjectRef plus optional display profile | required |
| format | — | full, short, hash-only | short |
| verified | — | bool | false |

$s_{min}$: short $(6g, 2g)$, full $(20g, 2g)$, hash-only $(6g, 2g)$

height is always fix $2g$ — neuron renders as a single line at body size

## address formats

| format | rendering | example |
|--------|-----------|---------|
| full | complete bech32 string | `bostrom1abc...xyz` |
| short | prefix + "..." + 4-char suffix | `bostrom1...vug5` |
| hash-only | 8 chars from middle of address | `4f8a2c91` |

format selection is a parameter — the membrane or molecule determines which format is appropriate for the context. short is the safe default across all zones

## occupy

$$s_w = \text{chars}(\text{address at format}) \cdot \text{char\_width}(\text{body})$$
$$s_h = 2g$$

neuron is always single-line. it does not wrap. if $s_w > c_w$, neuron switches to the next shorter format (full → short → hash-only) until it fits within the constraint. if hash-only still exceeds $c_w$, neuron truncates with ellipsis

verified = true appends vector(verified, $2g$) with $g/2$ gap after the address string, increasing $s_w$ by $2g + g/2$

## color

| condition | color |
|-----------|-------|
| default | #d7d7d7 (secondary text) |
| hover | #ffffff |
| active | [[emotion]] |
| verified | #ffffff with joy green vector |

## states

| state | visual change | trigger |
|-------|-------------|---------|
| default | address in #d7d7d7 | — |
| hover | underline, cursor pointer, color #ffffff | pointer over neuron |
| active | copy address to clipboard, color flashes [[emotion]] for $150\text{ms}$ | tap/click |
| disabled | color dims to #4b4b4d, no interaction | membrane disabled |

state transitions: $150\text{ms}$ ease

## 3D

in the 3D extension (§11 of [[prysm/layout]]):

- neuron renders on a plane at the same $p_z$ as its membrane
- neuron always faces the actor (billboard)
- size in quanta is constant in world space

## ECS

- Entity: neuron organelle
- Components:
  - `Sizing { width, height }` — computed from address string at format
  - `NeuronId { bech32 }` — the full bech32 address string
  - `NeuronFormat { full | short | hash_only }` — resolved format after constraint check
  - `NeuronVerified { bool }` — appends verified ion when true
  - `TextColor { color }` — from palette, #d7d7d7 default
- System: neuron participates in `OccupySystem` as a leaf — computes size from formatted address string; falls back to shorter format if width exceeds $c_w$

## Identity boundary

The native ID remains H(compressed pubkey), provided by the owning identity
profile. A foreign address remains exact bytes qualified by its domain; a Bech32
label is presentation and is never hashed into a native ID. `verified` requires
a separately supplied verification result for the precise claim/network; text
parsing or an identicon is not evidence of control. The atom needs only
`neuron-model` without its records feature, never keys or the execution engine.
