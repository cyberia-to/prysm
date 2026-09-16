---
tags: prysm, cyb
crystal-type: pattern
crystal-domain: cyber
status: proposed-ui
---

Proposed UI contract under [composition](../../system/specs/composition.md). Layouts, ECS records and interactions below specify intended behavior, not shipped coverage.

typography atom in [[prysm]]

the only way text appears in [[cyb]]. Play (google fonts) — a single font family across the entire interface. hierarchy comes from size and weight, never from decoration

## protocol role

text is a leaf in the element tree $\mathcal{T}$ (§7 of [[prysm/layout]]). leaf type: text. it has no sub-organelles. its membrane constrains it, it occupies space based on content and font size, membrane places it

## sizing

all values in spatial quanta $g$

| parameter | sizing type | values | default |
|-----------|-----------|--------|---------|
| content | — | any string | required |
| size | fix | h1($4g$), h2($3g$), h3($5g/2$), body($2g$), caption($7g/4$), micro($3g/2$) | body($2g$) |
| weight | — | regular (400), medium (500) | regular |
| align | — | left, center, right | left |
| line-height | — | 1.0, 1.2, 1.4 | 1.4 for paragraph, 1.0 for labels |
| max-lines | — | 1, 2, 3, unlimited | unlimited |

### size scale

| token | size | line-height | where in [[prysm/grid]] |
|-------|------|-------------|------------------------|
| h1 | $4g$ | 1.2 | space: page titles, hero numbers |
| h2 | $3g$ | 1.2 | space: section headers |
| h3 | $5g/2$ | 1.2 | space: subsection headers. context, avatar: zone labels |
| body | $2g$ | 1.4 | space: content text. commander: input text |
| caption | $7g/4$ | 1.4 | stars, time: labels, metadata. S, Σ: counts |
| micro | $3g/2$ | 1.0 | time: timestamps. graph: annotations |

### font

- family: Play (single family everywhere)
- no bold. no italic. no underline. no decoration
- hierarchy only through size and weight

### font metrics derivation

the relationship between spatial quantum $g$ and font rendering:

$$\text{font\_size}(token) = \text{size}(token) \cdot (g_{px} / g)$$

$g_{px}$ — the renderer's interpretation of $g$ in pixels (convention: $g_{px} = 8\text{px}$). for body($2g$): $\text{font\_size} = 2 \cdot 8 = 16\text{px}$

| token | size | font-size at $g_{px}=8$ | cap-height (Play) | line-height (computed) |
|-------|------|------------------------|-------------------|----------------------|
| h1 | $4g$ | 32px | ~23px | $4g \cdot 1.2 = 4.8g$ = 38.4px |
| h2 | $3g$ | 24px | ~17px | $3g \cdot 1.2 = 3.6g$ = 28.8px |
| h3 | $5g/2$ | 20px | ~14px | $2.5g \cdot 1.2 = 3g$ = 24px |
| body | $2g$ | 16px | ~11px | $2g \cdot 1.4 = 2.8g$ = 22.4px |
| caption | $7g/4$ | 14px | ~10px | $1.75g \cdot 1.4 = 2.45g$ = 19.6px |
| micro | $3g/2$ | 12px | ~8px | $1.5g \cdot 1.0 = 1.5g$ = 12px |

character width for Play at body(16px): average ~8.7px for Latin, ~9.2px for Cyrillic. since Play is not strictly monospace, text width computation uses the renderer's font shaping — this is why text glyph metrics are the exception to quantum alignment (I5)

the minimum legible size: micro($3g/2$) at $g_{px} = 8$ = 12px. below this, Play glyphs lose legibility on standard DPI (96-144). on high-DPI (288+), micro remains legible at smaller $g_{px}$

**renderer independence.** the protocol outputs size in quanta ($2g$, $3g$). the renderer translates: $\text{font\_size} = \text{size\_quanta} \cdot g_{px}$. different renderers may use different $g_{px}$:

| renderer | $g_{px}$ | body font-size |
|----------|---------|---------------|
| Portal (web) | 8px | 16px |
| Terminal (Sugarloaf) | 1 cell height | 1 cell × 2 = 2 cells |
| Bevy UI | 8 logical px | 16 logical px (scales with DPI) |
| 3D (Ren) | 8 world units | 16 world units |

## occupy

text occupies space based on content length and font size:

$s_w = \text{chars} \cdot \text{char\_width}(size)$
$s_h = \text{lines} \cdot size \cdot \text{line\_height}$

when $s_w > c_w$ (text exceeds membrane constraint): text wraps to next line, increasing $s_h$. when max-lines reached and content still exceeds: truncate with ellipsis

$s_{min} = (\text{char\_width}(size),\; size \cdot \text{line\_height})$ — one character at current size. below this, text is illegible. for body($2g$) at line-height 1.4: $s_{min} \approx (g, 3g)$

note: text glyph metrics are the one exception to quantum alignment (invariant I5) — character widths are determined by the font, not by $g$

## color

| level | color | use |
|-------|-------|-----|
| primary | #ffffff | default on dark backgrounds |
| secondary | #d7d7d7 | less important text |
| dim | #777777 | placeholders, disabled |
| muted | #4b4b4d | ghost text |
| [[emotion]] | acid palette | overrides color when signaling state |

## states

| state | visual change | trigger |
|-------|-------------|---------|
| default | color from palette | — |
| hover | underline appears (for linked text) | pointer over text |
| active | color shifts to [[emotion]] | tap/click |
| disabled | color dims to #4b4b4d | membrane disabled |
| selected | background highlight at 15% opacity of [[emotion]] | text selection |

state transitions: $150\text{ms}$ ease

## adaptation

text size tokens do not change between desktop and mobile. the same body($2g$) renders on both. adaptation happens through the membrane: narrower membrane → text wraps to more lines or molecule folds to a conformation that uses smaller text token

## 3D

in the 3D extension (§11 of [[prysm/layout]]):

- text renders on a plane at the same $p_z$ as its membrane
- text always faces the viewer (billboard orientation) — legibility requires frontal view
- text size in quanta is constant in world space. at greater distance from the viewer, text appears smaller but remains sharp

## ECS

- Entity: text organelle
- Components:
  - `Sizing { width, height }` — computed from content + font size
  - `TextContent { string }`
  - `TextSize { token }` — h1, h2, h3, body, caption, micro
  - `TextWeight { regular | medium }`
  - `TextAlign { left | center | right }`
  - `TextColor { color }` — from palette or [[emotion]] override
  - `MaxLines { n }` — truncation limit
- System: text participates in `OccupySystem` — computes size from content metrics, respects $c_w$ for wrapping
