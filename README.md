# prysm

The visual protocol for [[cyb]]: compose typed data into atoms, molecules and views.

This repository contains renderer code and a broader proposed design. The design
pages specify target behavior; their ECS sketches, chrome arrangements and proof
obligations do not establish that every feature is implemented. Implementation
and validation evidence belongs in `audit/`.

## The model

Atoms are rendering primitives. Molecules compose them into widgets. Views compose
molecules into navigable experiences. These are UI categories, independent of the
protocol's subject hierarchy. A view has no signing key or CellId. A neuron may
execute several progs and appear in several views; opening a view creates neither
a neuron nor a running program.

The proposed layout uses a spatial quantum `g = 8` logical units, sizing with
`fix`, `fill`, or `scale`, and `stack`, `grid`, or `layer` containers. Atoms and
molecules expose constraints and fold conformations to the layout engine.
Renderer mappings determine pixels, world units or terminal cells.

## Chroma

[Cyb's anatomy](../cyb/anatomy.md) defines the robot's 21 organs. Chroma arranges
their UI projections: Now supplies context, Avatar visualizes the robot, Com
accepts intent, Sense presents messages, and Sigma presents assets and attached
neurons. Time composes Log ← Now → Plan. Stars, launcher, adviser and the minimap
are widgets, not additional organs. Soul configures behavior, Soma supplies
cognition, and Body supplies physical resources.

Prysm receives typed projections and emits intent. The host captures the acting
neuron, attachment revision, network and payload; Ward checks authority and Vault
performs permitted key operations. No component reads secret keys from its props
or infers authority from an avatar, selected tab or destination.

## Example: graph search view

Illustrative composition notation, not an executable Rune declaration:

```text
view search [grid]:
  inhabits the space zone of chroma
  stack vertical [gap g]:
    input [search, placeholder "ask"]       canvas × bar
    table [results]                         canvas × canvas
      neuron-card                           col per row, folds on narrow
      pill [rank, source evidence]           col per row
```

Navigation uses [typed destinations](../neuron/specs/navigation.md), such as
`cyb://view/brain`; inspection and authored actions have separate contracts.
The [composition specification](system/specs/composition.md) defines the shared
boundary for all design pages. See also [[prysm/layout]], [[prysm/emotion]],
[[prysm/interaction]] and [[prysm/proof]].
