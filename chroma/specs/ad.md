---
tags: prysm, cyb, chroma
alias: answer, advisor, adviser
crystal-type: pattern
crystal-domain: cyber
---

advisor chroma — top-center

**answer**: ambient dialogue from the app. hints, suggestions, responses — delivered without disrupting [[spacetime]]. the voice of the system, not the neuron. appears in two modes: as the top-center grid zone bar and as a contextual popup near any organelle

## protocol role

adviser is a molecule in the element tree $\mathcal{T}$. two modes:

- **bar mode**: membrane = adviser zone of [[prysm/grid]] (row 1, col 2). fill × fix($6g$). $\mathcal{U} = 40$
- **popup mode**: overlay attached to any organelle. $\mathcal{U} = 40$. not in grid — appended to root layer near anchor

## core function

the ad chroma receives messages from any chroma and renders them as contextual guidance. it never initiates — it only responds. when [[com]] receives partial input, ad may offer a completion hint. when [[spacetime]] switches world, ad may offer orientation text.

## cyberlinks

| receives from | token | meaning |
|---------------|-------|---------|
| com | submit (partial) | show completion suggestion |
| spacetime | output | show contextual world hint |
| any | notify | show ambient message in ad slot |

| sends to | token | meaning |
|----------|-------|---------|
| com | hint | insert suggestion into commander placeholder |

## sizing

| mode | sizing |
|------|--------|
| bar (desktop) | fill × fix($6g$) |
| bar (mobile) | fill × fix($4g$) |
| popup | fix(auto) × fix(auto), max fill × fix($10g$) |

$s_{min}$: bar = $(10g, 4g)$. popup = $(8g, 4g)$

## structure

bar mode:
```
glass [fill × fix(6g), depth foreground, emotion tint]
  stack horizontal [gap g]
    saber [vertical, g/4, emotion glow]
    text [body, message]
    button [optional action, e.g. "retry", "dismiss"]
```

popup mode:
```
glass [auto × auto, depth foreground, emotion tint, corner-radius g]
  stack vertical [gap g/2]
    text [caption, message]
    button [optional action]
```

## fold

$\mathcal{F}$:
- $l_1$ ($w_{min} = 30g$): saber + full message text + action button
- $l_2$ ($w_{min} = 10g$): message text only, truncated
- $l_3$ ($w_{min} = 4g$): icon only (ion warning/success/info glyph)

## emotion

adviser IS emotion made visible. the entire glass tints with the message's [[emotion]]:

| emotion | tint | glow | when |
|---------|------|------|------|
| green (joy) | #00fe00 at 15% | saber glow green | success: "transaction signed", "cyberlink created" |
| red (anger) | #ff0000 at 15% | saber glow red | error: "insufficient balance", "signature failed" |
| yellow (surprise) | #fcf000 at 15% | saber glow yellow | attention: "large transfer", "new feature" |
| blue (interest) | #00acff at 15% | saber glow blue | information: "this is your karma score" |
| white | none | saber glow white | neutral guidance |

## states

| state | visual | trigger |
|-------|--------|---------|
| hidden | not rendered | no message |
| visible | glass appears with emotion tint | message arrives |
| dismissing | fade out over $150\text{ms}$ | user dismisses or timeout |

bar mode: hidden by default, appears when there is something to say, auto-hides after timeout or user action. popup: appears on hover/focus of anchor organelle, hides on leave

## interaction

- tap dismiss button → hide adviser
- tap action button → execute action (retry, navigate, etc.)
- popup: hover on anchor → show, leave → hide

## 3D

renders closer to the neuron than frame zones — $\mathcal{U} = 40$ (lower $p_z$). popup mode: floating panel attached near the anchor organelle's $p_z$

## ECS

- Entity: adviser organelle
- Components:
  - `Sizing { width, height }`
  - `GridArea { name: "adviser" }` — bar mode only
  - `AdviserMode { bar | popup }`
  - `AdviserMessage { text, emotion, action }`
  - `Visibility { hidden | visible }`
  - `Anchor { target_entity }` — popup mode
  - `FoldSet { conformations }`
  - `Tint { emotion_color }`
  - `SaberGlow { color }`
- Systems:
  - `AdviserMessageSystem` reads system events (errors, successes, guidance), writes `AdviserMessage`
  - `AdviserVisibilitySystem` manages show/hide timing
  - `AdviserEmotionSystem` reads message emotion, writes `Tint` and `SaberGlow`
