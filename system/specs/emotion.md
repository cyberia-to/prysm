---
tags: prysm, cyb, core
crystal-type: pattern
crystal-domain: cyber
status: proposed-ui
---

Proposed UI contract under [composition](../../system/specs/composition.md). Layouts, ECS records and interactions below specify intended behavior, not shipped coverage.

the emotion function in [[prysm]]

emotion is a computed color signal that encodes protocol state as affect. grounded in the [[color-emotion spectrum]]: seven wavelengths, seven feelings, one function. Components accept emotion as input under declared data/presentation rules. A green accent is not an authentication, validity or permission proof.

## the palette

9 values: 7 spectrum colors + neutral + inactive

| emotion | color | hex | signal domain |
|---------|-------|-----|---------------|
| anger | red | #ff0000 | danger, critical failure, overload |
| disgust | orange | #ff5b00 | contamination, invalid data, rejection |
| surprise | yellow | #fcf000 | attention, sudden change, caution |
| joy | green | #00fe00 | confidence, success, growth, life |
| interest | blue | #00acff | exploration, focus, discovery |
| sadness | indigo | #304ffe | withdrawal, loss, inactivity |
| fear | violet | #d500f9 | unknown threat, radiation, boundary |
| neutral | white | #ffffff | no signal, idle, default |
| inactive | gray | #4b4b4d | disabled, off, muted |

## the emotion function

$$\varepsilon: (v,\; \theta,\; d) \;\to\; C$$

$v$ — signal value (numeric or categorical). $\theta$ — reference threshold (context-dependent). $d$ — signal domain (determines which evaluation rule applies). $C$ — one of the 9 palette values

four evaluation rules, selected by domain $d$:

### polarity

binary: value is positive or negative relative to zero

$$\varepsilon_{polarity}(v) = \begin{cases} \text{green} & v > 0 \\ \text{white} & v = 0 \\ \text{red} & v < 0 \end{cases}$$

used by: [[prysm/counter]] (balance change), [[prysm/widget]] Σ (balance delta), [[prysm/table]] (row-level accent)

### threshold

value compared to reference with three zones

$$\varepsilon_{threshold}(v, \theta_{low}, \theta_{high}) = \begin{cases} \text{green} & v \geq \theta_{high} \\ \text{yellow} & \theta_{low} \leq v < \theta_{high} \\ \text{red} & v < \theta_{low} \end{cases}$$

used by: subject-card borders (sourced [[karma]] thresholds), the [Sphere view proposal](../../../aos/sphere.md) validator pills, the [Reactor view proposal](../../../aos/reactor.md) E-Ratio, and labeled transaction outcomes

generalizes to more zones:

$$\varepsilon_{threshold}(v, \vec{\theta}) = C_i \quad \text{where} \quad i = \max\{j : v \geq \theta_j\}$$

$\vec{\theta} = (\theta_1, \theta_2, \ldots)$ — ordered thresholds. $C_i$ — color for zone $i$. this allows mapping to any subset of the palette, not only green/yellow/red

### semantic

categorical: action type determines emotion directly

$$\varepsilon_{semantic}(a) = \text{lookup}(a, M)$$

$M$ — a finite map from action categories to colors:

| category | color | examples |
|----------|-------|----------|
| navigate | white | open, browse, expand |
| confirm | green | sign, stake, claim, delegate, send |
| danger | red | delete key, burn tokens, undelegate |
| caution | yellow | large transfer, irreversible action |
| explore | blue | search, discover, ask |
| guide | green/red/yellow/blue | adviser messages — by message type |

used by: [[prysm/button]], [[prysm/adviser]], [Com](../../chroma/specs/com.md)

### continuous

numeric value mapped to spectrum position. used when emotion encodes magnitude, not polarity

$$\varepsilon_{continuous}(v, v_{min}, v_{max}) = \text{spectrum}\left(\frac{v - v_{min}}{v_{max} - v_{min}}\right)$$

the spectrum function maps $[0, 1]$ to the ROYGBIV gradient: $0 \to$ red, $0.5 \to$ green, $1.0 \to$ violet. intermediate values interpolate between adjacent spectrum colors

used by: [[prysm/pill]] progress fill (cyberank weight), [[prysm/slider]] (position-dependent color), [[prysm/glass]] tint (when driven by continuous metric)

## application

emotion applies to three visual properties:

| property | how | opacity |
|----------|-----|---------|
| text color | direct replacement of text fill | 100% |
| saber glow | glow-color parameter of [[prysm/saber]] | 100% |
| glass tint | color overlay on [[prysm/glass]] surface | 15% |

a single component can carry emotion on multiple properties simultaneously: a [[prysm/button]] applies emotion to both text color and saber glow. a [[prysm/counter]] applies emotion to text and its saber change indicator

## inheritance

emotion flows through the element tree $\mathcal{T}$:

1. a view computes emotion from sourced protocol data (chain state, [[cyberank]], [[karma]])
2. the view passes emotion to its molecules as a parameter
3. molecules pass emotion to their atoms

an atom never computes emotion. it receives and renders. a molecule may compute emotion for its sub-atoms (e.g. [[prysm/counter]] computes polarity from value delta). a view computes emotion from supplied graph projections

## concrete thresholds

threshold values for every component that uses $\varepsilon_{threshold}$:

### karma (qualified subject / neuron-card)

| zone | condition | color |
|------|-----------|-------|
| high | karma $\geq \mu + \sigma$ | green (#00fe00) |
| medium | $\mu - \sigma \leq$ karma $< \mu + \sigma$ | white (#ffffff) |
| low | karma $< \mu - \sigma$ | red (#ff0000) |

$\mu$ — network mean karma, $\sigma$ — network standard deviation. Thresholds are relative to a declared network distribution, not absolute. Avatar visualizes the robot and has no implicit neuron identity: a robot-level accent must disclose any aggregation of attached subjects. Missing or stale evidence cannot become a control/verification badge.

### validator pills (Sphere view)

each metric (APR, power, commission, self-stake) evaluated independently:

| zone | condition | color |
|------|-----------|-------|
| above average | $v \geq \mu_{network}$ | green (#00fe00) |
| around average | $\mu_{network} \cdot 0.8 \leq v < \mu_{network}$ | yellow (#fcf000) |
| below average | $v < \mu_{network} \cdot 0.8$ | red (#ff0000) |

### E-Ratio (Reactor view)

| zone | condition | color |
|------|-----------|-------|
| healthy | $e \geq 0.5$ | green (#00fe00) |
| low | $0.2 \leq e < 0.5$ | yellow (#fcf000) |
| depleted | $e < 0.2$ | red (#ff0000) |

$e = \text{available energy} / \text{max energy}$, range $[0, 1]$

### transaction status

| state | color |
|-------|-------|
| complete | green (#00fe00) |
| pending | yellow (#fcf000) |
| failed | red (#ff0000) |

Categorical, not threshold. “Complete” requires the declared operation profile's outcome evidence; local commit, transport delivery, remote acceptance and network finality have separate labels. Color does not prove successful execution or grant authority.

### cyberank (content, pill, search-view particle rank)

uses $\varepsilon_{continuous}$: $v = \log_{10}(\text{rank})$, $v_{min} = 0$, $v_{max} = \log_{10}(\text{max\_rank})$. The declared continuous rule maps low to red, midpoint to green and high to violet. The logarithmic scale prevents a few high-rank particles from compressing the entire spectrum. Nonpositive/absent rank or a degenerate scale uses the unavailable-data fallback; rank is not a truth score.

## default

when no signal drives emotion: $\varepsilon = \text{neutral}$ (white). when a component is disabled: $\varepsilon = \text{inactive}$ (gray)

## fallback

when `EmotionSource` data is unavailable (chain offline, RPC timeout, fetching):

| condition | emotion | rationale |
|-----------|---------|-----------|
| data fetching (loading) | neutral (white) | no information — no signal |
| chain unreachable | neutral (white) | no data to evaluate — show absence of signal, not false signal |
| stale data (> 60s old) | last computed emotion, opacity 50% | fade indicates staleness — the viewer sees that information is aging |
| error in computation | neutral (white) + adviser red message | emotion system fails gracefully — never show wrong color |

the rule: emotion must never lie. if the system cannot compute a truthful signal, it shows no signal (neutral). showing a false green on stale data is worse than showing white

ECS: `EmotionSource` includes `freshness: Instant`. `EmotionSystem` checks freshness: if $t_{now} - t_{fresh} > 60s$, apply 50% opacity to `Emotion`. if source is `None`, emit neutral

## ECS

- Component: `Emotion { color: EmotionColor }` — the computed color, attached to any entity
- Component: `EmotionSource { domain: Domain, value: f64, thresholds: Vec<f64> }` — the input data
- System: `EmotionSystem` — reads `EmotionSource`, writes `Emotion` using the evaluation rules above
- System: `EmotionPropagateSystem` — propagates `Emotion` from views → molecules → atoms through the element tree

`EmotionSystem` runs before render systems, after data-fetch systems. emotion is recomputed every frame when the source value changes

## the binding

The color-emotion mapping is a proposed design convention inspired by evolutionary metaphors, not proof of a universal innate response. It is a presentation channel between the viewer and supplied graph data. See [[color-emotion spectrum]] for the evolutionary framework.

---

## the full palette

every color in [[cyb]] comes from this section. no hardcoded hex values outside this palette. renderers reference tokens by name

### background

| token | hex | use |
|-------|-----|-----|
| void | #000000 | screen background, the absence of signal |

void is the energetically correct default — on OLED, #000000 draws zero power. every other color exists on void

### text luminance

four levels of white on void

| token | hex | use |
|-------|-----|-----|
| primary | #ffffff | default text, active labels |
| secondary | #d7d7d7 | supporting text, descriptions |
| dim | #777777 | placeholders, metadata, timestamps |
| muted | #4b4b4d | ghost text, disabled labels |

text levels are pure white at varying opacity — no warm/cool tint

### glass

translucent surfaces over void. glass fill is white at varying opacity. emotion tint overlays at 12% when present

| token | opacity | hex (effective on void) | use |
|-------|---------|------------------------|-----|
| foreground | 0.70 | ~#b3b3b3 | modals, tooltips |
| midground | 0.38 | ~#616161 | cards, panels, commander |
| background | 0.15 | ~#262626 | space-level containers |
| subtle | 0.07 | ~#121212 | ambient regions |

glass borders are white at lower opacity than fill

| token | opacity |
|-------|---------|
| foreground-border | 0.15 |
| midground-border | 0.10 |
| background-border | 0.06 |
| subtle-border | 0.03 |

### saber

| token | hex | use |
|-------|-----|-----|
| saber-default | #ffffff | neutral separator, structural lines |
| saber-emotion | (from emotion) | glow carries action/state emotion |
| saber-dim | rgba(255,255,255,0.12) | structural dividers, inactive separators |

### sigil colors (token logos)

token sigils carry their brand color — the only non-palette colors allowed in prysm. these are immutable (the emotion system does not override them)

| sigil | color | hex |
|-------|-------|-----|
| BOOT | green | #00fe00 |
| HYDROGEN | gray | #777777 |
| ATOM | blue | #6f7390 |
| OSMO | purple | #5e12a0 |

## rules

- void is always the base. every other color exists on void
- emotion colors are max saturation / max brightness — acid, not pastel
- text levels are pure white at varying opacity (no warm/cool tint)
- glass fill is pure white at varying opacity (no color shift)
- the only colored elements are emotion signals (driven by the emotion function) and sigil colors (immutable brand identity)
- if a color is not in this palette, it does not belong in [[cyb]]
