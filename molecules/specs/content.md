---
tags: prysm, cyb
crystal-type: pattern
crystal-domain: cyber
status: proposed-ui
---

Proposed UI contract under [composition](../../system/specs/composition.md). Layouts, ECS records and interactions below specify intended behavior, not shipped coverage.

[[particle]] renderer molecule in [[prysm]]

the universal content display. any [[particle]] — text, image, video, audio — passes through content to become visible. adapts rendering to the particle format while keeping a consistent frame

## protocol role

molecule in $\mathcal{T}$. lives inside space zone, [[prysm/display]], the [graph-search view proposal](../../../aos/oracle.md)

## sizing

fill × auto (content-determined height)

$s_{min} = (8g, 2g)$

## structure

depends on particle format:

| format | structure |
|--------|----------|
| heading | text [h1/h2/h3] + optional ion |
| text + icon-L | ion left + text right |
| text + icon-R | text left + ion right |
| text + icon-LR | ion + text + ion |
| number + change | counter molecule |
| date | text [micro, machine time format: "56.03.29" or "2h ago"] |
| value-change | text [number] + saber [change indicator] + pill [delta, emotion] |
| star-indicator | vector [star] × fill level |
| picture | raster leaf [image, scale to fit] |
| video | raster leaf [video frame] + button [play] |
| audio | vector [waveform] + button [play] + slider [progress] |

## fold

$\mathcal{F}$:
- $l_1$ ($w_{min} = 25g$): full format with icons and details
- $l_2$ ($w_{min} = 10g$): text only, truncated
- $l_3$ ($w_{min} = 4g$): icon indicating particle type

## emotion

Content may carry an accent from sourced cyberank. Rank describes the declared graph computation, not truth, authorship, content integrity or action authority. Preserve source/evidence and freshness independently of the accent.

## states

| state | visual | trigger |
|-------|--------|---------|
| loading | skeleton placeholder | fetching particle |
| rendered | content visible | loaded |
| error | red adviser message | load failed |
| seed | particle visible as [[prysm/address]] hash bars | no cyberlinks to this particle yet — it exists as a seed |

## 3D

renders at membrane's $p_z$. image/video particles face the viewer (billboard)

## ECS

- Entity: content organelle
- Components:
  - `Sizing { width: Fill, height: auto }`
  - `Particle { particle }` — content address
  - `ParticleFormat { heading | text | image | video | audio | ... }`
  - `FoldSet { conformations }`
  - `Emotion { color }` — from cyberank
- System: `ContentRenderSystem` reads `Particle`, determines format, spawns appropriate leaf organelles
