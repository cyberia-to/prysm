---
tags: prysm, cyb
crystal-type: pattern
crystal-domain: cyber
status: proposed-ui
---

Proposed UI contract under [composition](../../system/specs/composition.md). Layouts, ECS records and interactions below specify intended behavior, not shipped coverage.

formal notation atom in [[prysm]]

renders a particle that is formal notation — programming source code, mathematical expressions (LaTeX/MathML), query languages, configuration. a leaf in the element tree (leaf type: code). math is code: both are formal languages parsed by a machine, both produce deterministic output, both can be executed or rendered

## protocol role

code is a leaf in the element tree $\mathcal{T}$ (§7 of [[prysm/layout]]). leaf type: code. it has no sub-organelles. its membrane constrains it, it occupies fill × auto (grows with content), membrane places it

## sizing

all values in spatial quanta $g$

| parameter | sizing type | values | default |
|-----------|-----------|--------|---------|
| src | — | string or particle CID | required |
| language | — | rust, python, js, latex, sql, nu, ... | auto-detect |
| line-numbers | — | bool | false |
| wrap | — | bool | false |
| max-height | fix | any $k \cdot g$ or unbounded | unbounded |

width: fill. height: auto (line count × line height), clipped at max-height with overflow scroll

$s_{min} = (10g, 2g)$ — one visible line

## rendering modes

| language class | rendering | engine |
|----------------|-----------|--------|
| programming (rust, python, js, nu, ...) | syntax-highlighted monospace | token colorizer |
| math (latex, mathml) | typeset notation | Vello path render |
| query (sql, cyql, sparql) | syntax-highlighted monospace | token colorizer |
| config (toml, json, yaml) | syntax-highlighted monospace | token colorizer |
| plain | monospace, no highlighting | — |

LaTeX and MathML render as paths via Vello — the output is a vector expression, not text. all other modes render as monospace text with token-based color mapping

## syntax highlighting palette

| token class | color |
|-------------|-------|
| keyword | #00acff (blue) |
| string | #00fe00 (green) |
| number literal | #fcf000 (yellow) |
| comment | #4b4b4d (gray) |
| function/method | #ffffff |
| type | #d7d7d7 |
| operator | #d7d7d7 |
| error | #ff0000 (red) |

colors are fixed — not driven by [[emotion]]. they are the semantic color language of code

## execution

The code atom renders source and supplied execution state. A view may compose it with a run button that emits intent to the host. Execution belongs to a prog under a captured neuron attachment, network, current grant and resource budget; a Hacklab view or selected source file is not itself a runtime or signer. Rendering, previewing or navigating to code must not execute it.

## states

| state | visual change | trigger |
|-------|-------------|---------|
| default | rendered with syntax highlight | — |
| loading | pulse overlay | CID src resolving |
| error | red underline at parse error position | syntax error |
| selected | 15% white highlight on selected lines | drag selection |
| executing | progress indicator in gutter | runtime active |

state transitions: $150\text{ms}$ ease

## 3D

in the 3D extension (§11 of [[prysm/layout]]):

- code renders on a plane at the same $p_z$ as its membrane
- code always faces the viewer (billboard)
- math typeset output renders as vector paths on the plane

## ECS

- Entity: code organelle
- Components:
  - `Sizing { width: Fill, height: Auto }`
  - `CodeSrc { Inline(String) | Particle(cid) }`
  - `CodeLanguage { language_id }` — for tokenizer selection
  - `CodeLineNumbers { bool }`
  - `CodeWrap { bool }`
  - `CodeMaxHeight { Option<f32> }`
  - `ExecutionState { idle | executing | done | error }`
- System: code participates in `OccupySystem` as a leaf — returns fill × line-count height
