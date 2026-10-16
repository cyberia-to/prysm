---
tags: prysm, cyb
crystal-type: pattern
crystal-domain: cyber
status: proposed-ui
---

Proposed UI contract under [composition](../../system/specs/composition.md). Layouts, ECS records and interactions below specify intended behavior, not shipped coverage.

data grid molecule in [[prysm]]

structured rows and columns. renders the [[cybergraph]] as ordered information — neurons by [[karma]], particles by [[cyberank]], tokens by balance. a grid IS a table (§6.2 of [[prysm/layout]])

## protocol role

molecule in $\mathcal{T}$. container type = grid (2D lattice). lives inside space zone. supports filter and sort operations before layout pass

## sizing

fill × auto (rows determine height)

$s_{min} = (15g, 6g)$ — header + one row

## structure

```
grid [fill × auto, col-gap g, row-gap 0]
  --- header row ---
  stack horizontal
    text [caption, column 1 label]
    text [caption, column 2 label, right-aligned if numeric]
    ...
  saber [horizontal, g/8, full width]
  --- data rows ---
  stack horizontal [per row]
    text/ion/counter [cell values]
  saber [horizontal, g/8] — row separator
  ...
```

## fold

$\mathcal{F}$:
- $l_1$ ($w_{min} = 40g$): all columns visible
- $l_2$ ($w_{min} = 20g$): primary + secondary columns
- $l_3$ ($w_{min} = 10g$): primary column only. if horizontal scroll needed: left column and top row stay fixed (pinned) — never lose reference

## alignment rules

- numeric columns: right-aligned. digits stack by place value
- text columns: left-aligned
- icon columns: center-aligned
- headers inherit alignment from data column

## filter and sort

from §6.2 of [[prysm/layout]]:

filter: predicate $f: row \to \{\text{true}, \text{false}\}$. hidden rows get $s_h = 0$
sort: ordering $\sigma$ on rows. changes placement, not sizing

both applied before layout pass. ECS: `GridFilter`, `GridSort`, `FilterSortSystem`

Rows retain their subject, identity domain, network and source evidence even when
columns are folded or sorted. A shared display address or token ticker is not a
row identity; aggregation must disclose its scope and preserve the original rows.

## variants

| variant | structure | use |
|---------|----------|-----|
| line | single-line rows, saber separators | simple lists |
| row-L | icon/label left, value right | key-value displays |
| row-R | value left, action right | transaction lists |
| row-8gutter | compact $g$ gutter | dense data |
| row-16gutter | standard $2g$ gutter | normal data |

## emotion

row-level accent: green for positive balance changes, red for negative. header: neutral

## states

| state | visual | trigger |
|-------|--------|---------|
| default | data visible | — |
| sorted | sort icon on active column | tap column header |
| loading | skeleton rows | fetching |
| empty | dim text "no data" | empty result set |

## 3D

renders at membrane's $p_z$

## ECS

- Entity: table organelle
- Components:
  - `Sizing { width: Fill, height: auto }`
  - `Grid { cols, rows, areas, col_gap, row_gap }`
  - `TableColumns { list of (label, width, align, sortable) }`
  - `TableRows { list of row data }`
  - `GridFilter { predicate }`
  - `GridSort { ordering }`
  - `FoldSet { conformations }`
- Systems:
  - `FilterSortSystem` applies filter + sort before layout
  - `TableRenderSystem` spawns row organelles from data
