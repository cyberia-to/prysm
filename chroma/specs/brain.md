---
tags: prysm, cyb, chroma
alias: map
crystal-type: pattern
crystal-domain: cyber
status: proposed-ui
---

Proposed UI contract under [composition](../../system/specs/composition.md). Layouts, ECS records and interactions below specify intended behavior, not shipped coverage.

map chrome — bottom-left

Brain is the rendered graph: browse particles, traverse cyberlinks and inspect neighborhoods. Memory is the separate filesystem/table/tile projection of particles. Both share Now's context; neither owns a signing identity.

The graph visualization molecule renders particles and links in several views, including network search and the robot's authorized personal projection. Subject and network filters retain their meaning across views.

## protocol role

molecule in the element tree $\mathcal{T}$. in chrome: bottom-left grid zone. as a view: fills the space zone or embeds in search/robot views

## core function

Brain presents Cybergraph data available through the host, including retained local data. It does not duplicate graph storage, authority or execution. Filesystem navigation belongs to Memory.

## Proposed host events

| receives from | token | meaning |
|---------------|-------|---------|
| com | navigate | inspect a typed graph destination; filesystem paths go to Memory |
| spacetime | locate | sync navigation to active world |

| sends to | token | meaning |
|----------|-------|---------|
| brain | map | graph navigation self-feed |
| spacetime | switch-renderer | open brain in space zone |
| com | context | Now's inspected graph context for command preparation |

## Proposed graph and memory capabilities

The historical JS-era file-manager list below is design input, not a current engine/API guarantee. Supported adapters must declare their query, content and publication profile.

- [[cyb/offline]] first
- localhost interface
- renders: [[space]] (3D), [[shadow]] (depth), [[heap]] (2D map), [[list]] (table)
- graph queries through declared host adapters; historical Datalog/Cozo examples do not require a second storage owner
- behavior: neuron-executed progs, including supported Rune programs
- static and dynamic [[linking]], private and public [[linking]]
- publishing to [[ipfs]] and [[cybergraph]]
- particle formats: text, video, audio, image, pdf, epub, web2

## Destinations

Use [typed Route values](../../../neuron/specs/navigation.md): View, Particle, Neuron or Prog, with an optional explicit network. A Memory path is a projection path, not a subject address. Historical `#`, `!`, `@`, `~` and `/` shorthand needs an explicit parser/adapter mapping; it must not silently create a subject, select an attachment or load code.

## graph visualization (molecule)

renders the [[cybergraph]] as visual space — particles as circles, cyberlinks as lines

### sizing

fill × fill. $s_{min} = (20g, 20g)$

### structure

```
glass [fill × fill, depth background]
  tabs [3d graph | 2d graph | last cyberlinks]
  text [caption, "Limit is: 500"]
  --- render area ---
```

### tabs

**3d graph** — interactive 3D. particles = blue spheres. cyberlinks = green lines. camera orbits. starfield background. commander: "Change limit" + fullscreen toggle

**2d graph** — force-directed 2D. particles = blue circles (size = focus). cyberlinks = green lines. draggable, zoomable. commander: "select 2 particles" + "Change limit" + fullscreen toggle

**last cyberlinks** — most recent cyberlinks as particle pairs side by side

```
glass [fill × fill, depth background, overflow scroll]
  stack horizontal [gap 2g, wrap]
    glass [fix(20g) × auto, depth midground] — source particle
    glass [fix(20g) × auto, depth midground] — target particle
```

### fullscreen

tap fullscreen → brain covers entire viewport, all grid zones hidden. only render + minimal commander visible

### context

| context | what renders |
|---------|-------------|
| network search | graph data within the declared network, availability and disclosure scope |
| robot brain | authorized graph projection across selected attachment filters; authors/domains stay separate |

### emotion

- particles: blue
- cyberlinks: green (#00fe00)
- selected particle: highlighted, connected links brighten
- starfield: ambient white dots

### states

| state | visual | trigger |
|-------|--------|---------|
| loading | "loading..." | fetching graph data |
| rendered | particles + links visible | data loaded |
| selecting | highlight mode — "select 2 particles" | commander prompt |
| fullscreen | chrome hidden, graph fills viewport | fullscreen toggle |

### interaction

- 3d: orbit (drag), zoom (scroll)
- 2d: pan (drag), zoom (scroll), tap particle → navigate
- select 2 particles → prepare cyberlink intent with a separately captured acting attachment and network
- Change limit → input + Confirm in commander

## 3D

Brain can render a 3D projection. A proposed layout may use sourced tri-kernel focus to influence positions; rank and proximity do not prove truth, authorship or permission.

## ECS

- Entity: brain organelle
- Components:
  - `Sizing { width: Fill, height: Fill }`
  - `BrainTab { graph_3d | graph_2d | last_cyberlinks }`
  - `BrainContext { route, subject_filters, network, disclosure_scope }`
  - `RenderLimit { count }`
  - `Fullscreen { bool }`
  - `SelectedParticles { list of particle, max 2 }`
- Systems:
  - `BrainFetchSystem` fetches graph data within limit
  - `BrainRender3dSystem` renders 3D scene
  - `BrainRender2dSystem` renders 2D force-directed layout
  - `BrainCyberlinkSystem` handles particle selection and emits authorized host action intent
