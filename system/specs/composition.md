---
tags: prysm, cyb
crystal-type: pattern
crystal-domain: cyber
status: proposed-ui
---

the composition model of [[prysm]]

```text
atom → molecule → view
```

These are three composition categories, not a signing hierarchy or a limit of
three tree depths. Molecules may contain molecules. The former UI term `cell`
means a view here; it must not be reintroduced as a runtime subject.

[Cyb anatomy](../../../cyb/anatomy.md) and [robot/neuron/prog architecture](../../../cyb/specs/architecture.md)
govern ownership. Layouts, ECS names and interactions in this design catalog are
proposed UI contracts. Code coverage, renderer parity and performance measurements
require separate audit evidence; design examples alone make no release claim.

## Atom

An atom supplies one rendering or input primitive. It accepts typed data, emotion
and a constraint, returns occupied size, renders a surface and emits input events.
It has no children and no ambient access to stores, keys or network services.
“Capability” in a rendering description means a visual ability, not a Ward grant.

A text atom formats a string. An address atom uses an explicit display profile
over retained subject bytes; Bech32 is one possible display profile, not
the definition of identity. An image atom renders bounded decoded pixels. Atoms
do not infer the meaning, authorship or authority of the data they display.

## Molecule

A molecule composes atoms or other molecules into a shaped widget: a button,
table, neuron-card or timeline. It validates its typed props, emits typed intent,
and declares fold conformations. Geometry is local to the widget; view-specific
data arrives through props rather than a global store.

| input | output |
|-------|--------|
| typed projection and evidence | rendered subtree |
| emotion and constraint | occupied size and selected conformation |
| local input events | selection, navigation or action intent |

A neuron-card knows the subject's domain, display profile and supplied evidence.
It does not own the neuron or authenticate a subject merely by rendering its
address. Molecule IDs and ECS entity IDs identify UI objects only.

## View

A view is a navigable composition of molecules. It owns presentation state,
filters and subscriptions through host-provided adapters. It can be full-screen
or embedded. Pages, tabs and panels are view/widget arrangements, not subjects.

A view may inspect several neurons and networks. The robot may attach zero, one
or many neurons in observation, control or delegated modes. Opening a view or
inspecting a neuron/prog never creates, attaches, selects or executes that subject.
Another tab or independent program lifecycle does not require another key.

| input | output |
|-------|--------|
| viewport and typed Route | rendered molecule tree and navigation intent |
| authorized data projection, origin and freshness | display state and emotion |
| separately captured action context, without secrets | exact action intent for the host |

Views may share projections and local navigation state through the host. Durable
behavior belongs to a prog executed by a neuron; its state, continuations and
effect records belong to Cybergraph/BBG. View closure, prog cancellation and
neuron detachment are different operations. Log renders retained history.

## Navigation and authority

[Route](../../../neuron/specs/navigation.md) distinguishes View, Particle,
Neuron and Prog. Subject/prog references retain their identity domain, original
identifier and optional explicit network. No network means unqualified inspection,
not an implicit choice of the currently selected network. Example view routes are
`cyb://view/brain` and `cyb://view/sigma`; internal tab state is not an arbitrary
extension of the canonical URI grammar.

Legacy `cell://ID` needs an exact resolver mapping. Cyb's built-in mapping is only
`cell://landing` → `cyb://view/robot`; unknown IDs stay unresolved. This alias does
not authorize code loading. Legacy landing source lives at
[cyb/pages/landing.rune](../../../cyb/pages/landing.rune).

Before dispatch, the host captures the controlled attachment, binding revision,
subject, network, prog/invocation or native caller, payload, grant and resource
reservation. Ward evaluates the current grant and revocation; Vault carries out
the permitted key operation. Props and checkpoints contain references, not keys.
Changing the UI selection cannot reauthor an outstanding action or its response.
Actions through another attachment need an explicit permitted binding.

UI events such as navigate, notify or record are local adapter messages. They are
not automatically signed cyberlinks. A profile that persists or publishes them
must specify authorship, bytes, disclosure, ordering and failure handling. Local
commit, transport delivery, remote acceptance and network finality remain distinct.

## The composition tree

```text
view
├── molecule (com projection)
├── molecule (tabs)
│   ├── atom (text)
│   └── atom (saber)
└── molecule (table)
    └── molecule (neuron-card)
        ├── atom (address)
        └── atom (image)
```

Biological cells remain a metaphor in layout research. Grid/table cells and
terminal character cells remain geometric/rendering units. None adds a CellId,
subject key or authorization layer to the robot → neurons model.
