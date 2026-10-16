---
tags: prysm, cyb
crystal-type: pattern
crystal-domain: cyber
status: proposed-ui
---

Proposed UI contract under [composition](../../system/specs/composition.md). Layouts, ECS records and interactions below specify intended behavior, not shipped coverage.

3D geometry atom in [[prysm]]

renders a particle that is a 3D scene. a leaf in the element tree (leaf type: mesh). the rendering pipeline is entirely separate from the 2D compositor — mesh requires a GPU mesh pipeline with depth buffer, PBR materials, and a lighting model. the primary format is glTF

for raster images see [[prysm/image]]. for 2D paths see [[prysm/vector]]

## protocol role

mesh is a leaf in the element tree $\mathcal{T}$ (§7 of [[prysm/layout]]). leaf type: mesh. it has no sub-organelles. its membrane constrains the 2D viewport into which the 3D scene is projected. the scene itself is infinite

## glTF structure

a glTF particle delivers:
- **scene graph** — hierarchical node transforms
- **meshes** — vertex buffers, index buffers, primitive topology
- **materials** — PBR (metallic/roughness), base color, normal map, emissive
- **textures** — embedded raster maps for material channels
- **skeleton** — bones and bind poses for skinned meshes
- **animations** — keyframe tracks (translation, rotation, scale, morph weights)
- **lights** — point, spot, directional (glTF extension)
- **cameras** — perspective and orthographic views

## sizing

all values in spatial quanta $g$

| parameter | sizing type | values | default |
|-----------|-----------|--------|---------|
| src | — | particle CID | required |
| width | fix or fill | any $k \cdot g$ or fill | fill |
| height | fix or fill | any $k \cdot g$ or fill | fill |
| camera | — | scene default or override | scene default |
| background | — | transparent or color | transparent |

$s_{min} = (8g, 8g)$

## camera and interaction

the default camera comes from the glTF scene. if no camera is defined, a perspective camera is placed at $(0, 0, 5)$ looking at origin. the membrane can expose orbit interaction (drag to rotate, scroll to zoom) — this is a property of the membrane, not the mesh atom

## animations

if the glTF contains animation tracks, mesh exposes an `AnimationState`:

| state | behavior |
|-------|----------|
| idle | first frame of default animation |
| playing | advancing animation clock |
| paused | animation clock stopped |

animation playback is driven at the molecule/view level — mesh atom exposes the clock

## states

| state | visual change | trigger |
|-------|-------------|---------|
| default | rendered at current animation frame | src loaded |
| loading | glass placeholder, pulse $1\text{s}$ | src not yet resolved |
| error | vector [broken-mesh, $4g$, anger] | src failed |

state transitions: $150\text{ms}$ ease

## emotion

mesh carries [[emotion]] as an emissive overlay — a color tint added to the material's emissive channel at 12% intensity. this is equivalent to the 12% glass tint on image and glass atoms, adapted to 3D surface shading

## 3D

mesh is natively 3D. in the [[prysm]] 3D extension:

- mesh renders into its own render pass with full depth buffer
- the membrane defines the 2D viewport rect into which the projection is composited
- mesh does not billboard — it has its own orientation from the scene graph
- scale from $g$ quanta applies to the viewport size, not the internal 3D scale

## ECS

- Entity: mesh organelle
- Components:
  - `Sizing { width, height }`
  - `MeshSrc { cid }` — glTF particle CID
  - `MeshCamera { scene_default | Override(transform) }`
  - `MeshBackground { transparent | Color }`
  - `AnimationState { idle | playing | paused }`
  - `AnimationClock { f32 }` — seconds
  - `LoadState { loading | loaded | error }`
  - `EmissiveTint { emotion: Option<Color> }` — 12% emissive overlay
- System: `MeshSystem` drives render pass, advances animation clock, composites into 2D layout
