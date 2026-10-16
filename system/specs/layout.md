---
tags: prysm, cyb, article, core
crystal-type: pattern
crystal-domain: cyber
status: proposed-ui
crystal-size: deep
---

Proposed UI contract under [composition](../../system/specs/composition.md). Layouts, ECS records and interactions below specify intended behavior, not shipped coverage.

# prysm layout protocol

a formal spatial composition system for the interface of [[superintelligence]]

---

## 0. abstract

This paper proposes a layout composition algebra for user interfaces. the protocol is a pure function: element tree × viewport → exact coordinates for every element. three axioms generate the entire system: a membrane-organelle protocol ($\Pi$: constrain → occupy → place), three sizing primitives ($\Phi$: fix, fill, scale — the three relations between part and whole), and three container topologies ($\mathcal{K}$: stack, grid, layer — 1D chain, 2D lattice, depth). all sizes are expressed in quanta of a spatial quantum $g = 8$, derived from visual acuity, Fitts's law, and quantum alignment. the protocol is renderer-independent, deterministic, and completes in $\mathcal{O}(n)$ time

14 theorems with proofs: linear time (T1), determinism (T2), sizing irreducibility (T3) and completeness (T4), container completeness (T5), optimal fold derivation (T6), urgency-gravity composition (T7), multimodal generalization (T8), branching fold via Pareto front (T9), rewrite termination (T10), confluence (T11), algebra completeness (T12), amortized $\mathcal{O}(1)$ fold selection (T13), multi-node decomposition (T14). 10 invariants. The theorem statements and proof sketches are research candidates, not a claim of completed machine verification. the protocol generalizes to any bounded measurable domain: audio, haptic, neural (T8)

Companion specifications define [[prysm/emotion]], [[prysm/interaction]] and [[prysm/responsive]] adaptation. Acceptance requires the evidence in §16 and §19; examples of Cyb layouts do not establish renderer coverage or proof completion.

---

## 1. the problem

the [[cybergraph]] is a directed authenticated multigraph where [[neurons]] are protocol subjects that author links between [[particles]]. Humans, robots, programs and sensory devices can contribute through explicitly attributed subjects; each UI element or device is not automatically a neuron. the structural record is append-only. the active computation evolves through the [[tri-kernel]]. this graph will outlive every rendering technology that exists today

this graph needs a spatial protocol with four properties:

- renderer-independent — outputs coordinates, not instructions for a specific engine. the same protocol feeds any renderer that can draw rectangles
- deterministic — same tree + same viewport → same coordinates. provably, not by convention
- linear time — layout completes in $\mathcal{O}(n)$ where $n$ is the organelle count. provably, not by claim
- formally specified — every rule has a formula. every property has a proof or is honestly marked as conjecture

This paper explores a layout algebra from three axioms.

---

## 2. notation

| symbol | meaning |
|--------|---------|
| $\Pi$ | the membrane-organelle protocol (constrain → occupy → place) |
| $\Phi$ | the set of sizing primitives: {fix, fill, scale} |
| $\mathcal{K}$ | the set of container types: {stack, grid, layer} |
| $g$ | the spatial quantum |
| $\mathcal{T}$ | element tree — hierarchy of membranes (containers), organelles (elements), and leaves (terminal organelles with no sub-organelles). input to layout function |
| $\square = (\square_w, \square_h)$ | viewport size |
| $c = (c_w, c_h)$ | constraint imposed by membrane on organelle |
| $s = (s_w, s_h)$ | occupied size returned by organelle to membrane |
| $p = (p_x, p_y)$ | position assigned by membrane to organelle |
| $z$ | depth index (layer ordering) |
| $r$ | scale coefficient, $r \in (0, 1]$ |
| $s_{min}$ | minimum viable size of an organelle |
| $n_f$ | count of fill-type sibling organelles in a membrane |
| $\mathcal{F}$ | ordered set of fold layouts for a molecule |
| $\mathcal{U}$ | urgency function mapping organelles to z-levels |
| $e$ | organelle (entity in ECS) |
| $n$ | count of organelles in a membrane |

---

## 3. three-layer specification

following the [[crystal]] framework: axioms generate, conventions configure, invariants constrain

### 3.1 axioms

change any axiom and the entire system reconfigures

| axiom | symbol | value |
|-------|--------|-------|
| protocol | $\Pi$ | constrain → occupy → place (§4) |
| sizing | $\Phi$ | {fix, fill, scale} (§5) |
| containers | $\mathcal{K}$ | {stack, grid, layer} (§6) |

derived — the layout function:

$$\text{layout}(\mathcal{T},\; \square) \;\to\; \{(e_i,\; p_{x_i},\; p_{y_i},\; s_{w_i},\; s_{h_i},\; z_i)\}$$

$\mathcal{T}$ — the element tree: a hierarchy of membranes and organelles. $\square = (\square_w, \square_h)$ — viewport size. output: for each organelle $e_i$, its position $p$, size $s$, depth $z$

### 3.2 derived constants and conventions

$g$ is derived from three physical constraints (§8): visual acuity at arm's length, Fitts's law touch target, and integer divisibility of reference screen width. $g^* = 8$ is the unique solution. the protocol works for any $g > 0$ — the derivation selects the optimal value for current human-device interaction

| parameter | value | source |
|-----------|-------|--------|
| spatial quantum | $g = 8$ | derived: visual acuity + Fitts's law + quantum alignment (§8) |
| desktop viewport | $\square = (240g, 135g)$ | convention: reference display dimensions |
| mobile viewport | $\square = (49g, 106g)$ | convention: reference display dimensions |
| structural breakpoint | $\square_w = 96g$ | convention: fold threshold analysis |
| side column | fix($25g$) | convention: min width fitting context/avatar content |
| commander height | fix($6g$) | derived: min touch target ($4g$) rounded up to comfortable ($6g$) |
| top bar | fix($6g$) desktop, fix($4g$) mobile | convention: content requirement per fold |
| menu overlay | fix($25g$) desktop, fix($35g$) mobile | convention: content-driven |

| $k$ | $k \cdot g$ | use |
|-----|-----------|-----|
| 1 | $g$ | minimum gap |
| 2 | $2g$ | mobile padding |
| 3 | $3g$ | desktop padding |
| 4 | $4g$ | side buttons (S, Σ) |
| 6 | $6g$ | commander, button, top bar |
| 25 | $25g$ | side columns |

#### named sections

human work is not in raw quanta — it is in semantic chunks. these are the named sizes every prysm component snaps to. seven names cover most layouts; raw quanta remain available for the rest

| name | size | role |
|------|------|------|
| `col` | $25g$ | one column. side panel, single card, mobile half-width |
| `wide` | $50g$ | two columns. dialog, content + sidebar split, wide card |
| `band` | $75g$ | three columns. article, hero, full modal |
| `canvas` | fill | everything available. main work area, scrollable content |
| `line` | $2g$ | one line of body text, baseline grid row |
| `bar` | $6g$ | one row of action. commander, top bar, button, input |
| `row` | $12g$ | one row of content. table row, neuron-card, list item |

horizontal scale is derived from the cyb side-column ($25g$) by integer multiplication ($1 \cdot 25g$, $2 \cdot 25g$, $3 \cdot 25g$). vertical scale is derived from text metrics ($2g$ = body line) and Fitts's law touch target ($6g$ = comfortable interactive height)

new sections are added only if a size repeats across three or more molecules without semantic match to an existing name. otherwise: raw quanta. the discipline of seven names is itself a kind of beauty

### 3.3 invariants

testable constraints. the layout is not ready until all pass

| # | invariant | formal statement |
|---|-----------|-----------------|
| I1 | determinism | $\forall \mathcal{T}, \square: \text{layout}(\mathcal{T}, \square)_1 = \text{layout}(\mathcal{T}, \square)_2$ |
| I2 | single-pass | each organelle measured exactly once per layout computation |
| I3 | linear time | $\exists\, k:\; t(\text{layout}) \leq k \cdot |\mathcal{T}| \cdot f_{max}$ where $f_{max}$ = max fold set size |
| I4 | constraint respect | $\forall e_i:\; s_{w_i} \leq c_{w_i} \;\wedge\; s_{h_i} \leq c_{h_i}$ |
| I5 | quantum alignment | $\forall e_i:\; g \mid p_{x_i} \;\wedge\; g \mid p_{y_i} \;\wedge\; g \mid s_{w_i} \;\wedge\; g \mid s_{h_i}$ (except text glyph metrics) |
| I6 | z monotonicity | $\mathcal{U}(a) > \mathcal{U}(b) \;\Rightarrow\; z(a) > z(b)$ |
| I7 | fold legibility | $\forall$ molecule $m$ with fold set $\mathcal{F}_m$: $l_k$ renders legibly at $w_{min}(l_k)$ |
| I8 | renderer independence | output identical across all target renderers |
| I9 | semantic completeness | $\forall e_i:\; \text{role}(e_i) \in \{\text{interactive}\} \;\Rightarrow\; \text{label}(e_i) \neq \emptyset$ |
| I10 | frame budget | $t_{layout}(\mathcal{T}) \leq 2\text{ms}$ for $|\mathcal{T}| \leq 10{,}000$ |

### 3.4 motion convention

motion is convention, not axiom. the protocol computes static coordinates — motion is how the renderer transitions between two successive layout computations

the motion function:

$$\mu(e,\; s_0,\; s_1,\; t) = s_0 + (s_1 - s_0) \cdot \alpha(t)$$

$s_0$ — previous state (position + size). $s_1$ — target state (new layout output). $t$ — time elapsed since layout change. $\alpha(t)$ — easing function, $\alpha: [0, T] \to [0, 1]$, $\alpha(0) = 0$, $\alpha(T) = 1$

| parameter | value | rationale |
|-----------|-------|-----------|
| $T$ (duration) | $150\text{ms}$ | below perception threshold for causality (200ms) |
| $\alpha$ | ease (cubic-bezier 0.25, 0.1, 0.25, 1.0) | matches natural deceleration |

all motion in prysm uses the same $T$ and $\alpha$. one duration, one curve. uniformity is legibility — the viewer learns one rhythm

motion applies only when $s_0 \neq s_1$ for the same entity $e$. motion does not affect layout computation. the protocol always outputs $s_1$ (target). the renderer interpolates. this preserves I2 (single-pass) and I1 (determinism)

choreography: when multiple properties change simultaneously (fold + emotion + position), they transition in parallel. exception: overlay entrance/exit uses two-phase (fade $T/2$ then slide $T/2$) to soften the interruption at high $z$

ECS: `MotionState { s_0, s_1, t_start }`. `MotionSystem` runs after layout, before render

### 3.5 semantic layer

the protocol outputs spatial coordinates. accessibility requires semantic annotation: what each organelle means, not where it is

every organelle carries an optional semantic role:

| role | meaning | examples |
|------|---------|----------|
| navigation | changes the inspected destination | commander input, menu items, tabs, stars |
| action | triggers a state change | buttons, toggles |
| display | presents data (read-only) | counters, address, pill, content |
| input | accepts user input | text input, slider, token amount |
| landmark | structural anchor for orientation | context, avatar, commander, adviser |
| group | semantically related organelles | validator list, token table, filter bar |

ECS: `Semantic { role, label, description }`, `NavigationOrder { index }` — derived from element tree (depth-first, same order as layout traversal), `LiveRegion { politeness }` for dynamic content

renderer mapping: Portal → ARIA roles, Terminal → focus ring, Bevy UI → AccessibilityNode, 3D → spatial audio cues

I9 enforces: every interactive organelle has a non-empty label (see invariants table above)

---

## 4. the protocol (axiom $\Pi$)

### 4.0 why this protocol

the fundamental question of spatial layout: who decides how much space an organelle gets?

three approaches:

membrane dictates. the membrane assigns exact size and position. the organelle has no input. fast ($\mathcal{O}(1)$ per decision), but the membrane lacks information about the organelle's intrinsic needs — a text block cannot communicate how tall it needs to be

organelle dictates. each organelle declares its size, the membrane must accommodate. flexible, but organelles can overflow, conflict, or demand contradictory space. no coordination guarantee

negotiation. membrane and organelle exchange information iteratively until agreement. maximally expressive. but requires multiple passes. convergence not guaranteed

this is not a strategic interaction. there is no conflict of interest. membrane and organelle are parts of the same tree — like a cell wall and a mitochondrion. the membrane does not compete with the organelle. it defines the physical boundary within which the organelle exists. the organelle does not negotiate — it truthfully reports its intrinsic size because there is no advantage to misreporting (exceeding the constraint is physically impossible, underreporting makes it illegible)

prysm chooses: membrane constrains, organelle occupies, membrane places

the membrane constrains — imposing a boundary. the organelle occupies — growing to fill what it needs within that boundary. the membrane places — positioning each organelle in space

information flows one direction: constraints down, sizes up, positions down. no upward influence from organelle to membrane constraint. this eliminates backtracking and guarantees single-pass $\mathcal{O}(n)$

the tradeoff. prysm cannot express content-dependent membrane sizing — a membrane cannot ask "how tall is my tallest organelle?" and resize itself before constraining others, because that requires measuring organelles before deciding membrane size (two passes). this is accepted. organelles that need unbounded space use scroll. the guarantee of linear time and determinism is worth more than multi-pass expressiveness

### 4.1 constrain

membrane computes a constraint for the organelle:

$$c = (c_w,\; c_h)$$

$c_w$ — maximum width the organelle may occupy (in quanta $g$). $c_h$ — maximum height. the organelle sees only $c$. it has no access to $\square$, siblings, or state outside its subtree. the root membrane receives $c_{root} = \square$

ECS: Entity = organelle. Component = `Constraint { c_w, c_h }`. System = `ConstrainSystem` — membrane reads own size + container params, writes `Constraint` to each organelle

### 4.2 occupy

organelle computes occupied size:

$$s = (s_w,\; s_h) \quad \text{subject to} \quad s_w \leq c_w \;\wedge\; s_h \leq c_h$$

$s_w$ — width actually occupied. $s_h$ — height occupied. invariant I4 enforced

when intrinsic content exceeds $c$, two adaptation rules:

shrink — organelle compresses proportionally to minimum viable size:

$$s_w = \max(s_{min},\;\; r \cdot c_w)$$

$s_{min}$ — smallest size at which this organelle functions legibly (defined per organelle). when $s_w = s_{min}$ and content still exceeds, fold threshold reached (§4.3)

scroll — organelle's layout size equals constraint, internal content extends beyond:

$$s_{visible} = (c_w, c_h) \quad;\quad s_{content} = (s'_w, s'_h) \quad \text{where} \quad s'_w > c_w \;\vee\; s'_h > c_h$$

scroll creates a viewport within an organelle. used for the space zone and data lists

ECS: Component = `Sizing { width: SizeType, height: SizeType, min_w, min_h }`. System = `OccupySystem` — reads `Sizing` + `Constraint`, writes `OccupiedSize { s_w, s_h }`

### 4.3 fold

when $r \cdot c_w < s_{min}$, the organelle hits its fold threshold and must reorganize — like a protein changing conformation to fit available space

each molecule defines $\mathcal{F}$ — an ordered set of conformations — as a required parameter:

$$\mathcal{F} = \{l_1, l_2, \ldots, l_k\} \quad \text{where} \quad w_{min}(l_1) > w_{min}(l_2) > \cdots > w_{min}(l_k)$$

$l_1$ — widest conformation (all sub-organelles visible). $l_k$ — narrowest (icon only). $w_{min}(l_j)$ — minimum width at which conformation $l_j$ is legible

selection:

$$l^* = l_i \quad \text{where} \quad i = \min\{j : w_{min}(l_j) \leq c_w\}$$

fold is a discrete transition between conformations. the layout computation is instantaneous (step function), but the renderer animates the visual transition over $150\text{ms}$ ease — motion serves state communication, not decoration. $\mathcal{F}$ is mandatory: a molecule without a fold set violates I7

ECS: Component = `FoldSet { conformations: Vec<(min_width, SubTree)> }`. System = `FoldSystem` — reads `FoldSet` + `Constraint`, writes `ActiveFold { index }`

### 4.3a fold derivation

Theorem 6 (fold derivation for stacks). for a stack container with $m$ organelles, each with importance $\phi^*_i$ and width $w_i$, the optimal fold set $\mathcal{F}$ can be computed in $\mathcal{O}(m \log m)$

Definition. a fold set $\mathcal{F}$ is optimal if at every constraint width $c_w$, the active conformation retains the maximum total importance among all conformations that fit in $c_w$

Algorithm.

1. assign each organelle $i$ an importance score $\phi^*_i$ (from [[cyberank]], [[focus]], or manual priority)
2. sort organelles by $\phi^*_i$ ascending: $\phi^*_{\sigma(1)} \leq \phi^*_{\sigma(2)} \leq \cdots \leq \phi^*_{\sigma(m)}$
3. initialize $l_1$ = full set, $w_{min}(l_1) = \sum_{i=1}^{m} w_i + (m-1) \cdot gap$
4. for $j = 1$ to $m - 1$:
   - $l_{j+1} = l_j \setminus \{e_{\sigma(j)}\}$ — remove the least important remaining organelle
   - $w_{min}(l_{j+1}) = w_{min}(l_j) - w_{\sigma(j)} - gap$
5. $\mathcal{F} = \{l_1, l_2, \ldots, l_m\}$, ordered by decreasing $w_{min}$

Proof of optimality. by greedy exchange argument. suppose an alternative fold set $\mathcal{F}'$ has a conformation $l'$ at some width $c_w$ that achieves higher total importance than the conformation $l^*$ selected by the algorithm. then $l'$ contains an organelle $e_j$ that $l^*$ excluded, and excludes some organelle $e_k$ that $l^*$ included. since the algorithm removes by ascending importance, $\phi^*_j < \phi^*_k$. replace $e_j$ with $e_k$ in $l'$: importance increases (contradiction) or widths change. since both have width $\leq c_w$ and $w_j$ was removed before $w_k$ (lower importance), the exchange produces a conformation with $\geq$ importance that still fits. the exchange can be repeated until $l' = l^*$. ∎

Complexity. step 2 is $\mathcal{O}(m \log m)$. steps 3-4 are $\mathcal{O}(m)$. total: $\mathcal{O}(m \log m)$

Grid fold. for grids, fold means hiding columns or rows — a 1D problem on tracks. apply the same algorithm to tracks instead of organelles: sort tracks by importance, remove least important first. each removal hides an entire column/row, reducing $w_{min}$ by the track width + gap. same $\mathcal{O}(m \log m)$

Branching fold sets. Theorem 6 produces linear chains. when importance is vector-valued $\vec{\phi^*}_i \in \mathbb{R}^d$ (e.g. readability importance vs interaction importance), different organelle subsets may be Pareto-optimal at the same width

Theorem 9 (branching fold derivation). for $m$ organelles with $d$-dimensional importance vectors, the Pareto-optimal fold set can be computed in $\mathcal{O}(m^d \cdot m \log m)$

Algorithm.

1. enumerate all $\binom{m}{k}$ subsets of size $k$ for each $k = m, m{-}1, \ldots, 1$
2. for each subset $A$: compute $w_{min}(A) = \sum_{i \in A} w_i + (|A| - 1) \cdot gap$ and $\vec{\phi^*}(A) = \sum_{i \in A} \vec{\phi^*}_i$
3. within each width class (subsets with the same $w_{min}$), compute the Pareto front: $A$ dominates $B$ iff $\vec{\phi^*}(A) \geq \vec{\phi^*}(B)$ componentwise with at least one strict inequality
4. the fold set $\mathcal{F} = \bigcup_k \text{ParetoFront}(w_{min} = k)$, ordered by $w_{min}$

Proof. the Pareto front at each width contains all non-dominated importance vectors — by definition, no other subset at that width is strictly better in all dimensions. the union across widths is the complete set of conformations worth considering

Complexity. step 1 generates $2^m$ subsets (dominated by step 3). for fixed $w_{min}$, the Pareto front of $n$ points in $\mathbb{R}^d$ is computable in $\mathcal{O}(n \log^{d-2} n)$ for $d \geq 2$ (Kung et al., 1975). the total number of Pareto-optimal conformations is $\mathcal{O}(m^{d-1})$ per width class (upper bound on Pareto front size in $\mathbb{R}^d$). for $d = 1$ (scalar importance): the front is a single point — Theorem 6 recovers. for $d = 2$: at most $\mathcal{O}(m)$ Pareto-optimal conformations per width, $\mathcal{O}(m^2)$ total. specification-time computation, not runtime

Fold selection with branching. at runtime, user configuration or display context determines a weight vector $\vec{w} \in \mathbb{R}^d$ (how much to value each importance dimension). the fold function becomes:

$$l^* = \arg\max_{l \in \mathcal{F},\; w_{min}(l) \leq c_w} \vec{w} \cdot \vec{\phi^*}(l)$$

the scalarized importance $\vec{w} \cdot \vec{\phi^*}(l)$ reduces branching to linear selection: given $\vec{w}$, exactly one conformation is optimal at each width. this preserves determinism (Theorem 2) as long as $\vec{w}$ is deterministic

practical scope. $d = 1$ covers all current prysm molecules (Theorem 6). $d = 2$ is the realistic maximum: readability (how much information the conformation shows) vs interactivity (how many interactive elements it retains). $d \geq 3$ is theoretically supported but no practical use case exists

### 4.4 place

membrane assigns position to the organelle:

$$p = (p_x,\; p_y)$$

$p_x$ — horizontal offset from membrane's origin. $p_y$ — vertical offset. the organelle does not choose its position

ECS: Component = `Position { x, y, z }`. System = `PlaceSystem` — reads `OccupiedSize` of all organelles + container params, writes `Position`

### 4.5 proofs

Theorem 1 (linear time). for a tree $\mathcal{T}$ with $n$ organelles where each molecule has a fold set of at most $f_{max}$ conformations, the protocol completes in $\mathcal{O}(n \cdot f_{max})$

Proof. the protocol performs a single depth-first traversal of $\mathcal{T}$. at each node $e_i$:
- constrain: membrane reads its own `OccupiedSize` and container parameters. both already computed (membrane visited before organelle in DFS). cost: $\mathcal{O}(1)$
- occupy: for a leaf, size computed from `Sizing` type and $c$. cost: $\mathcal{O}(1)$. for a molecule with fold, scan $\mathcal{F}$ — at most $f_{max}$ comparisons. cost: $\mathcal{O}(f_{max})$
- place: membrane reads occupied sizes of all organelles. for a stack with $m$ organelles, summing widths costs $\mathcal{O}(m)$. each organelle contributes to exactly one membrane's sum, so total placement across all nodes: $\mathcal{O}(n)$

total: $\sum_{i=1}^{n} \mathcal{O}(f_{max}) = \mathcal{O}(n \cdot f_{max})$

no backtracking: organelle cannot access membrane state (§4.1). ∎

Theorem 2 (determinism). for any tree $\mathcal{T}$ and viewport $\square$, $\text{layout}(\mathcal{T}, \square)$ produces a unique output

Proof. by structural induction on $\mathcal{T}$

*base case.* a leaf $e$ with sizing type $\phi$ and constraint $c$. the sizing function (§5.1-5.3) is a pure function of $(\phi, c)$ — no randomness, no external state. occupied size $s$ is unique. position $p$ assigned by membrane. output for $e$ is unique

*inductive step.* a membrane $e$ with organelles $e_1, \ldots, e_m$. assume each organelle produces unique output given its constraint. the membrane computes constraints from its own size and container parameters — both determined (by inductive hypothesis on $e$'s own membrane). organelles are ordered (tree structure is fixed). each constraint is unique. by hypothesis, each organelle's output is unique. placement is a function of occupied sizes and container parameters — all determined. output for $e$ and all organelles is unique

*root.* the root membrane receives $c_{root} = \square$, which is given. by induction, all organelles produce unique output. ∎

---

## 5. sizing (axiom $\Phi$)

### 5.1 the three primitives

fix — intrinsic size. no dependency on membrane:

$$\text{size}(\text{fix},\; k) = k \cdot g \quad \text{where} \quad k \in \mathbb{N}^+$$

$k$ — size in quanta. $g$ — the spatial quantum. the renderer translates $k \cdot g$ into its native units. ECS: `Sizing { width: Fix(6) }` → $6g$

fill — extrinsic size. absorbs remaining space after fix and scale siblings:

$$\text{size}(\text{fill},\; s_r,\; n_f) = \frac{s_r}{n_f}$$

$s_r$ — remaining space in quanta: $s_r = c_{membrane} - \sum s_{fix} - \sum (r_j \cdot c_{membrane}) - (n - 1) \cdot gap$. $n_f$ — count of fill sibling organelles. $gap$ — spacing between organelles (in quanta)

weighted: $\text{size}(\text{fill}_w,\; s_r,\; w_i,\; W) = s_r \cdot w_i / W$ where $W = \sum w_j$

ECS: `Sizing { width: Fill { weight: 1.0 } }`

scale — proportional size. fraction of membrane:

$$\text{size}(\text{scale},\; r,\; c_{membrane}) = \max(s_{min},\;\; r \cdot c_{membrane}) \quad \text{where} \quad r \in (0, 1]$$

$r$ — fraction. $c_{membrane}$ — membrane's constraint in quanta. $s_{min}$ — minimum in quanta (triggers fold when hit)

ECS: `Sizing { width: Scale { r: 0.33, min: 15 } }` → min $15g$

### 5.2 irreducibility

Theorem 3. no proper subset of $\Phi$ can express all sizing behaviors that $\Phi$ expresses

Proof. by exhibiting a layout requiring the removed primitive

*remove fix.* an organelle must be exactly $6g$ regardless of membrane size. scale($r$, $c_{membrane}$) = $r \cdot c_{membrane}$ varies with membrane — no constant $r$ produces $6g$ for all $c_{membrane}$. fill depends on siblings and membrane. neither produces a constant. fix is necessary

*remove fill.* a stack with two organelles: one is fix($25g$), the other must absorb whatever remains. scale($r$) requires knowing $r$ in advance, but the remainder $c_{membrane} - 25g$ as a fraction of $c_{membrane}$ is $1 - 25g/c_{membrane}$, different for every $c_{membrane}$. no fixed $r$ works across viewports. fill is necessary

*remove scale.* an organelle must always be $\frac{1}{3}$ of its membrane, regardless of siblings. fix is constant — cannot track membrane. fill depends on siblings — adding a fix($12g$) sibling changes the fill result. scale is necessary. ∎

the three primitives correspond to three fundamental relations between part and whole:

| primitive | relation | meaning | formal class |
|-----------|----------|---------|-------------|
| fix | independence | part does not depend on whole | $s = k \cdot g$ (constant) |
| fill | complement | part fills what remains after other parts | $s = f(c_{membrane}, \text{siblings})$ |
| scale | similarity | part is proportional to whole | $s = r \cdot c_{membrane}$ (linear through origin) |

Theorem 4 (completeness of $\Phi$). any monotone non-negative sizing function $s: \mathbb{R}^+ \to \mathbb{R}^+$ bounded by $c_{membrane}$ can be expressed as a composition of fix, fill, and scale

Proof. decompose $s(c_{membrane})$ into three components:

$$s(c_{membrane}) = \underbrace{a}_{\text{fix}} + \underbrace{r \cdot c_{membrane}}_{\text{scale}} + \underbrace{\epsilon(c_{membrane})}_{\text{fill}}$$

where $a = \lim_{c \to \infty} (s(c) - r \cdot c)$ is the constant offset (fix), $r = \lim_{c \to \infty} s(c)/c$ is the asymptotic scale factor (scale), and $\epsilon = s - a - r \cdot c_{membrane}$ is the remainder (fill — dependent on siblings and membrane, absorbed after fix and scale are resolved)

for any practical sizing function: if $s$ is constant → pure fix. if $s$ is linear through origin → pure scale. if $s$ depends on what other organelles leave → pure fill. any combination decomposes into these three. ∎

note: this proof assumes piecewise-linear sizing. nonlinear sizing (e.g. logarithmic) is outside the protocol's scope — the fold mechanism (§4.3) handles discrete transitions between linear regions

### 5.3 resolution order

within any membrane: fix → scale → fill. fill absorbs the remainder. this order is invariant — changing it would break fill's definition

---

## 6. containers (axiom $\mathcal{K}$)

three container types define three spatial topologies. in [[cellular automata]] terms: stack is the 1D chain (elementary automaton), grid is the 2D lattice (Game of Life), layer is the multi-plane stack (coupled maps). together they span all rectangular composition — and the tree of nested containers forms a tree cellular automaton, which is computationally complete

### 6.1 stack (1D chain)

organelles along a single axis. ECS: `Stack { direction, gap, align }`

| parameter | domain | default |
|-----------|--------|---------|
| direction | {horizontal, vertical} | vertical |
| gap | $\{g \cdot k : k \in \mathbb{N}\}$ | $g$ |
| align | {start, center, end, stretch} | start |

placement for horizontal stack with organelles $e_1, \ldots, e_n$:

$$p_{x_i} = \sum_{j=1}^{i-1} (s_{w_j} + gap) \qquad p_{y_i} = \text{align}(s_{h_i},\; c_h)$$

### 6.2 grid (2D lattice)

organelles in rows and columns. ECS: `Grid { cols, rows, areas, col_gap, row_gap }`

| parameter | domain |
|-----------|--------|
| columns | list of track sizes (fix, fill, scale) |
| rows | list of track sizes (fix, fill, scale) |
| areas | named regions spanning cells |
| col-gap, row-gap | $\{g \cdot k\}$ |

track resolution: fix → scale → fill (same order as sizing)

a grid is a table. grid operations include:

filter: predicate $f: e \to \{\text{true}, \text{false}\}$ selecting visible rows. $\text{rows}_{vis} = \{e_i : f(e_i)\}$. filtered rows get $s_h = 0$, excluded from placement. applied before layout pass

sort: ordering $\sigma$ on rows. changes placement sequence, not sizing. applied before layout pass

ECS: `GridFilter { predicate }`, `GridSort { ordering }` — `FilterSortSystem` runs before layout

### 6.3 layer (depth)

organelles share the same spatial region, ordered by depth. ECS: `Layer`, `Position { z }`

$z$ is assigned by the membrane via the urgency function $\mathcal{U}$:

$$z(e) = \mathcal{U}(e)$$

| urgency | $z$ | elements | rationale |
|---------|-----|----------|-----------|
| ambient | 0 | space | background, scrollable |
| persistent | 10 | context, avatar, stars, graph, time, S, Σ | frame, always visible |
| active | 20 | commander | primary interaction |
| interrupting | 30 | menus | temporarily demands focus |
| guiding | 40 | adviser, tooltip | system guidance |
| blocking | 50 | modal | demands resolution |

on renderers without native z-axis (terminal character grid): z maps to draw order — higher z overwrites lower z at the same position

### 6.4 completeness

Theorem 5 (completeness of $\mathcal{K}$). for any finite set of axis-aligned rectangles $S = \{R_1, \ldots, R_k\}$ contained in a bounding rectangle $R$, there exists a composition of stack, grid, and layer containers that produces $S$

Proof. three cases exhaust all possibilities

Case 1: $S$ is a partition of $R$ (non-overlapping, union = $R$).

let each $R_i = [a_i, b_i] \times [c_i, d_i]$. collect all distinct x-coordinates $X = \{a_1, b_1, \ldots, a_k, b_k\}$ sorted: $x_0 < x_1 < \cdots < x_m$ where $x_0 = 0, x_m = W$. collect all distinct y-coordinates $Y = \{c_1, d_1, \ldots, c_k, d_k\}$ sorted: $y_0 < y_1 < \cdots < y_n$ where $y_0 = 0, y_n = H$

define grid $G$ with $m$ columns (column $j$ has width $x_j - x_{j-1}$) and $n$ rows (row $j$ has height $y_j - y_{j-1}$)

for each $R_i$: $a_i = x_p, b_i = x_q$ for some $p < q$ and $c_i = y_r, d_i = y_s$ for some $r < s$. map $R_i$ to a grid cell spanning columns $p{+}1$ through $q$, rows $r{+}1$ through $s$

spans are non-overlapping. suppose cell spans of $R_i$ and $R_j$ share a grid cell $(col, row)$. this cell corresponds to rectangle $[x_{col-1}, x_{col}] \times [y_{row-1}, y_{row}]$, which is contained in both $R_i$ and $R_j$. but $R_i \cap R_j = \emptyset$ (partition). contradiction

spans cover all cells. let $(col, row)$ be any grid cell. pick an interior point $p$ of $[x_{col-1}, x_{col}] \times [y_{row-1}, y_{row}]$. since $S$ partitions $R$, $p \in R_i$ for some $i$. since grid lines include all edges of $R_i$, we have $a_i \leq x_{col-1}$ and $b_i \geq x_{col}$ (otherwise an edge of $R_i$ would lie strictly between $x_{col-1}$ and $x_{col}$, contradicting that all edges are grid lines). similarly for $y$. so the cell is within $R_i$'s span

therefore a single grid-with-spans expresses any partition. grid tracks: at most $2k$ columns, $2k$ rows. construction is $\mathcal{O}(k \log k)$ (dominated by sorting coordinates)

Case 2: $S$ is non-overlapping but does not cover $R$. add empty rectangles to fill gaps, forming a partition. apply Case 1. empty cells are grid cells with no organelle (size allocated but nothing rendered)

Case 3: $S$ contains overlapping rectangles. partition $S$ into groups $S_1, S_2, \ldots$ by z-order: within each group, no two rectangles overlap ($\mathcal{U}$ assigns distinct z-levels to overlapping elements, §6.3). each group is non-overlapping — apply Case 1 or 2 to get a grid for each group. compose groups using layer. ∎

Corollary. stack is expressively redundant — a horizontal stack is a grid with 1 row, a vertical stack is a grid with 1 column. $\mathcal{K}_{min} = \{\text{grid}, \text{layer}\}$ is complete. stack is retained for ergonomics ($\mathcal{O}(1)$ per child vs grid's track resolution) and readability

### 6.5 relationship to sliceable layouts

Kozminski & Kinnen (1988) proved that not all rectangular partitions are sliceable (producible by recursive horizontal/vertical cuts). the non-sliceable layouts require T-junctions: three rectangles meeting at a point where the junction cannot be expressed as a single cut

stack (recursive slicing) cannot produce T-junctions. grid-with-spans handles them: the spanning cell crosses the T-junction boundary. Theorem 5 proves this formally — the coordinate-collection construction produces spans that naturally cross T-junctions

this resolves the open question of §6.4 in the original version of this paper

---

## 7. the leaf

terminal organelle. no sub-organelles. contains rendered data

| type | data | prysm pattern | ECS |
|------|------|---------------|-----|
| text | string at font size | [[prysm/text]] | `TextLeaf { content, size }` |
| vector | path, shape, line | [[prysm/saber]], [[prysm/vector]], [[prysm/vector]] | `VectorLeaf { path }` |
| raster | bitmap, photo, video | image/video [[particle]] | `RasterLeaf { particle }` |

### coverage

every [[particle]] that occupies space renders through one of these three types. text covers markdown, code, addresses, numbers, labels, timestamps. vector covers icons, shapes, graphs, diagrams, SVG paths, lines, dividers. raster covers images, video frames, camera feeds, 2D projections of 3D models

compound particles (a page with text + images) are not leaves — they are membranes with leaf organelles

particles without spatial dimension: audio does not occupy layout coordinates. the protocol places a visual control (play button, waveform) which is a vector or raster leaf. audio playback is outside layout scope. 3D models in 2D mode render as raster (projected frame). in 3D mode (§9) they extend to 3D constraints

three types are sufficient because every visual output is either characters (text), paths (vector), or samples (raster). there is no fourth category of visual data

a leaf is a [[particle]] made spatial. storage ([[cybergraph]]) → placement (layout protocol) → rendering (target renderer). three independent systems

---

## 8. optimal $g$

$g$ was convention ($g = 8$). this section derives it from three physical constraints, upgrading it from convention to derived constant

### 8.1 three constraints

constraint 1: font legibility. the minimum legible font size for body text on a screen at arm's length (~60cm) is determined by visual acuity. the human eye resolves ~1 arcminute. at 60cm, this is:

$$h_{min} = 2 \cdot d \cdot \tan(\theta/2) \approx d \cdot \theta = 0.6\text{m} \cdot \frac{1}{60} \cdot \frac{\pi}{180} \approx 0.175\text{mm}$$

this is the minimum stroke height. for body text, a character needs ~5 strokes vertically (cap height). minimum character height: $5 \times 0.175 \approx 0.87\text{mm}$. with line spacing (1.4×): minimum line height $\approx 1.22\text{mm}$

at standard DPI (96 DPI = 3.78 px/mm): minimum line height $\approx 4.6\text{px}$. this is the absolute floor. for comfortable reading (sustained text, not signage): double it → $\sim 10\text{px}$ line height

body text in prysm: size $= 2g$, line-height $= 1.4$. line height $= 2g \cdot 1.4 = 2.8g$ pixels. for $2.8g \geq 10$: $g \geq 3.6$

constraint 2: touch target. Fitts's law predicts movement time to a target of width $W$:

$$MT = a + b \cdot \log_2(2D/W + 1)$$

the ISO 9241-9 minimum touch target: $7\text{mm} \times 7\text{mm}$. Apple HIG: $44 \times 44$ points ($\approx 6.9\text{mm}$). Material: $48 \times 48$ dp ($\approx 7.6\text{mm}$)

taking $7\text{mm}$ as the minimum, at 96 DPI: $7\text{mm} = 26.5\text{px}$. prysm's minimum interactive element: $4g \times 4g$ (ion touch target). for $4g \geq 26.5$: $g \geq 6.6$

constraint 3: quantum alignment. $g$ must be an integer in pixels (for aliasing-free rendering on integer-pixel screens). positions and sizes are multiples of $g$, so $g$ must evenly divide common screen widths. common widths: 360, 375, 390, 393, 412, 414, 768, 1024, 1280, 1366, 1440, 1920, 2560

$g = 8$: divides 360(45), 768(96), 1024(128), 1280(160), 1440(180), 1920(240), 2560(320). fails: 375, 390, 393, 412, 414 — but these are mobile widths where sub-pixel rendering handles the remainder (at most $g-1 = 7$ pixels absorbed by the fill column)

$g = 4$: divides all of the above. but violates constraint 2: $4g = 16\text{px} = 4.2\text{mm}$ — below touch target minimum

$g = 10$: $4g = 40\text{px} = 10.6\text{mm}$ — comfortable touch. but divides fewer screen widths (360, 1280, 1920, 2560 — not 768, 1024, 1440). and body text at $2g = 20\text{px}$ is larger than needed

### 8.2 the optimization

$$g^* = \arg\min_{g \in \mathbb{N}^+} \left| g - g_{target} \right| \quad \text{subject to} \quad g \geq 3.6 \;\wedge\; g \geq 6.6 \;\wedge\; g \mid W_{ref}$$

$W_{ref} = 1920$ (reference desktop width). $g_{target}$ minimizes wasted space on mobile: $g_{target} = \arg\min_g \max_w (w \mod g)$ over common mobile widths $w$

evaluating:

| $g$ | legibility ($2.8g$ px) | touch ($4g$ px / mm) | divides 1920 | max mobile waste |
|-----|----------------------|---------------------|-------------|-----------------|
| 6 | 16.8px ✓ | 24px / 6.3mm ✗ | ✓ (320) | 6px (375→372) |
| 7 | 19.6px ✓ | 28px / 7.4mm ✓ | ✗ | 4px (375→371) |
| 8 | 22.4px ✓ | 32px / 8.5mm ✓ | ✓ (240) | 7px (375→368) |
| 10 | 28.0px ✓ | 40px / 10.6mm ✓ | ✓ (192) | 5px (375→370) |

$g = 7$: meets legibility and touch but fails quantum alignment (does not divide any standard width cleanly)

$g = 8$: meets all three constraints. smallest $g$ that satisfies touch target AND divides the reference desktop width. mobile waste (max 7px) is absorbed by the fill column — invisible to the viewer

### 8.3 the derivation

$$g^* = 8$$

derived from: visual acuity at arm's length (lower bound 3.6), Fitts's law touch target (lower bound 6.6), integer divisibility of reference width 1920 (upper bound filter). $g = 8$ is the unique integer satisfying all three constraints while minimizing body text size (avoiding waste of screen real estate on larger $g$)

$g$ is no longer convention. it is the unique solution to a constrained optimization over three physical parameters. changing any input (viewing distance, DPI standard, minimum touch target) changes $g^*$ — but the derivation method is permanent

Proposed ECS: `SpatialQuantum { g: u32 }` — set once at application init, with reference value 8. the derivation above justifies this value; future devices with different DPI/distance may compute $g^*$ dynamically

---

## 9. 3D extension

in 2D, layout has two participants: membrane and organelle. the membrane constrains, the organelle occupies, the membrane places. depth ($z$) is an integer ordering assigned by urgency $\mathcal{U}$

in 3D, depth becomes a real spatial dimension and a third participant enters the protocol: the [[cybergraph]] itself. what is important must appear close to the viewer. what is peripheral must recede. importance is not decided by membrane or organelle — it is computed by the [[tri-kernel]] as [[focus]] ($\phi^*$) and [[gravity]]

### 9.1 the gravity phase

the 2D protocol has three phases: constrain → occupy → place. the 3D protocol has four:

constrain → occupy → gravitate → place

| phase | who acts | what happens |
|-------|----------|-------------|
| constrain | membrane | imposes $(c_w, c_h)$ on xy-plane |
| occupy | organelle | returns $(s_w, s_h)$ subject to constraint |
| gravitate | [[cybergraph]] | computes $p_z$ from focus: $p_z = f(\phi^*(e))$ |
| place | membrane | assigns $(p_x, p_y)$ on xy-plane |

the gravity function:

$$p_z(e) = d_{max} \cdot (1 - \phi^*(e))$$

$\phi^*(e)$ — the focus of organelle $e$ in the [[cybergraph]], computed by the [[tri-kernel]]. $d_{max}$ — maximum depth of the 3D space (convention). when $\phi^* = 1$ (maximum focus): $p_z = 0$ — directly in front of the viewer. when $\phi^* = 0$ (no focus): $p_z = d_{max}$ — at the far edge of the space

this mirrors physical gravity: massive objects (high focus) attract the observer's attention. the [[cybergraph]] is the gravitational field. focus is mass. distance is inversely proportional to importance

### 9.2 three participants

| participant | 2D role | 3D role |
|-------------|---------|---------|
| membrane | constrains xy, places xy | constrains xy, places xy (unchanged) |
| organelle | occupies space, reports size | occupies space, reports size (unchanged) |
| [[cybergraph]] | not involved | computes $p_z$ via gravity. third participant |

in 2D, the protocol is a dialogue between membrane and organelle. in 3D, the [[cybergraph]] speaks — it determines which knowledge is near and which is far. This is a proposed rendering policy: when a UI element explicitly projects a graph particle, a sourced tri-kernel focus value can influence its distance from the viewer. Other UI elements use declared layout defaults. A layout entity does not automatically become a particle or a signing neuron

### 9.3 sizing and containers in 3D

$\Phi$ unchanged: fix, fill, scale apply per dimension. an organelle can be fix($6g$) wide, fill tall, and its depth is computed by gravity — not by sizing

$\mathcal{K}$ extends:
- stack: 1D chain along any axis (x, y, or z)
- grid: 2D lattice on any pair of axes (xy, xz, yz)
- layer: in 2D, layer = z-ordering by $\mathcal{U}$. in 3D, $\mathcal{U}$ combines with gravity via the composition rule (§9.4)

### 9.4 urgency-gravity composition

in 2D, depth is determined solely by urgency $\mathcal{U}$. in 3D, two forces compete: gravity (from [[cybergraph]] focus) and urgency (from UI semantics). the composition rule resolves them:

$$p_z(e) = d_{max} \cdot \left(1 - \max\left(\phi^*(e),\; \frac{\mathcal{U}(e)}{\mathcal{U}_{max}}\right)\right)$$

$\phi^*(e)$ — focus from [[tri-kernel]], $\phi^* \in [0, 1]$. $\mathcal{U}(e)$ — urgency level, $\mathcal{U} \in [0, 50]$. $\mathcal{U}_{max} = 50$. the $\max$ selects whichever measure assigns higher importance — the element is placed at the depth of its strongest claim to proximity

equivalently: $p_z(e) = \min(p_{z,gravity}(e),\; p_{z,urgency}(e))$ — urgency can pull closer but never push farther

Theorem 7 (composition properties).

(a) urgency dominance: $\forall e$ with $\mathcal{U}(e) = \mathcal{U}_{max}$: $p_z(e) = 0$ regardless of $\phi^*(e)$

*Proof.* $\max(\phi^*, \mathcal{U}_{max}/\mathcal{U}_{max}) = \max(\phi^*, 1) = 1$. $p_z = d_{max} \cdot (1 - 1) = 0$. ∎

(b) gravity dominance for non-urgent: $\forall e$ with $\mathcal{U}(e) = 0$: $p_z(e) = d_{max} \cdot (1 - \phi^*(e))$

*Proof.* $\max(\phi^*, 0/50) = \phi^*$. ∎

(c) monotonicity in urgency: $\mathcal{U}(a) > \mathcal{U}(b) \;\wedge\; \phi^*(a) = \phi^*(b) \;\Rightarrow\; p_z(a) \leq p_z(b)$

*Proof.* $\max(\phi^*, \mathcal{U}(a)/50) \geq \max(\phi^*, \mathcal{U}(b)/50)$ since $\mathcal{U}(a) > \mathcal{U}(b)$. $1 - \max(\ldots)$ is smaller for $a$. ∎

(d) monotonicity in focus: $\phi^*(a) > \phi^*(b) \;\wedge\; \mathcal{U}(a) = \mathcal{U}(b) \;\Rightarrow\; p_z(a) \leq p_z(b)$

*Proof.* symmetric to (c). ∎

(e) determinism: $p_z$ is a pure function of $(\phi^*, \mathcal{U})$, both deterministic inputs

the composition resolves the key scenario: a modal ($\mathcal{U} = 50$) for a low-focus entity ($\phi^* = 0.1$) must appear in front of high-focus space content ($\phi^* = 0.9, \mathcal{U} = 0$). by (a), the modal is at $p_z = 0$. the space content is at $p_z = d_{max} \cdot 0.1$. modal in front. correct

### 9.5 invariants in 3D

| invariant | 3D status | proof |
|-----------|-----------|-------|
| I1 determinism | holds | Theorem 7(e): $p_z$ is pure function of deterministic inputs |
| I2 single-pass | holds | gravitate adds $\mathcal{O}(1)$ per node (lookup $\phi^*$, compute max) |
| I3 linear time | holds | $\mathcal{O}(n \cdot f_{max})$ + $\mathcal{O}(n)$ for gravity = still $\mathcal{O}(n \cdot f_{max})$ |
| I4 constraint respect | holds for xy | $p_z$ is not constrained by membrane — determined by $\max(\phi^*, \mathcal{U}/\mathcal{U}_{max})$ |
| I5 quantum alignment | holds | $g \mid p_z$: output rounded to $g$ after composition |
| I6 z monotonicity | holds | Theorem 7(c,d): monotonic in both urgency and focus when the other is equal |
| I7 fold legibility | holds | fold on xy, gravity on z — independent axes |
| I8 renderer independence | holds | output is world coordinates in quanta $g$ |

### 9.6 the layout function in 3D

$$\text{layout}_{3D}(\mathcal{T},\; \square,\; \phi^*) \;\to\; \{(e_i,\; p_{x_i},\; p_{y_i},\; p_{z_i},\; s_{w_i},\; s_{h_i})\}$$

note the third input: $\phi^*$ — the focus distribution from the [[cybergraph]]. in 2D, layout depends only on tree and viewport. in 3D, it depends on the state of knowledge itself

ECS: `Gravity { focus: f64 }` component. `GravitateSystem` reads `Gravity`, writes `Position { z }`. runs between `OccupySystem` and `PlaceSystem`

---

## 10. the layout algebra

the element tree $\mathcal{T}$ is a term in a multi-sorted algebra. formalizing this enables algebraic simplification of trees before layout computation

### 10.1 signature

$$\Sigma = (\text{sorts},\; \text{operations})$$

sorts: Element, Size, Container

operations:

| operation | signature | meaning |
|-----------|-----------|---------|
| $\text{fix}(k)$ | $\mathbb{N}^+ \to \text{Size}$ | intrinsic size of $k$ quanta |
| $\text{fill}(w)$ | $\mathbb{R}^+ \to \text{Size}$ | absorb remaining, weight $w$ |
| $\text{scale}(r)$ | $(0,1] \to \text{Size}$ | fraction of membrane |
| $\text{leaf}(s_w, s_h)$ | $\text{Size}^2 \to \text{Element}$ | terminal organelle |
| $\text{stack}(d, g, a, \vec{e})$ | $D \times \mathbb{N} \times A \times \text{Element}^* \to \text{Element}$ | 1D chain |
| $\text{grid}(C, R, M, \vec{e})$ | $\text{Size}^* \times \text{Size}^* \times \text{Areas} \times \text{Element}^* \to \text{Element}$ | 2D lattice |
| $\text{layer}(\vec{e}, \vec{z})$ | $\text{Element}^* \times \mathbb{N}^* \to \text{Element}$ | depth composition |

$D = \{h, v\}$ (direction), $A = \{start, center, end, stretch\}$ (alignment)

### 10.2 the layout homomorphism

the layout function is the unique homomorphism from the term algebra to the coordinate algebra:

$$\text{layout}: L \to C \quad \text{where} \quad C = \{(p_x, p_y, s_w, s_h, z) \in (\mathbb{N} \cdot g)^5\}$$

"unique" because the layout function is completely determined by the axioms ($\Pi$, $\Phi$, $\mathcal{K}$) and the viewport $\square$. given an element tree (a term) and a viewport, the output is uniquely determined (Theorem 2)

### 10.3 simplification rules

algebraic identities that preserve the layout homomorphism — applying any rule produces the same coordinates:

Rule 1 (stack flattening). a fill-sized stack nested in a stack with the same direction, gap, and alignment can be flattened:

$$\text{stack}(d, g, a, [\ldots, \text{stack}(d, g, a, [e_1, \ldots, e_n])_{fill}, \ldots]) = \text{stack}(d, g, a, [\ldots, e_1, \ldots, e_n, \ldots])$$

*Proof.* the inner stack with fill sizing absorbs all remaining space, then distributes it to $e_1, \ldots, e_n$ using the same direction, gap, and alignment. flattening produces identical constraints for each $e_i$ because: (a) the inner fill-stack's constraint equals the space it would receive as a sibling, (b) direction, gap, align are identical. by Theorem 2, identical constraints → identical output. ∎

Rule 2 (identity elimination). a container with a single child where the container has fill sizing is identity:

$$\text{stack}(d, g, a, [e])_{fill} = e$$

*Proof.* the container passes its entire constraint to $e$. since $e$ is the only child, it receives the full membrane space. $e$'s position is $(0, 0)$ relative to the container, which adds nothing. ∎

Rule 3 (layer collapse). a layer with a single child is identity:

$$\text{layer}([e], [z]) = e \quad \text{(with z assigned to } e \text{)}$$

Rule 4 (dead branch elimination). if $c_w < s_{min}(e)$ for every conformation in $\mathcal{F}(e)$, the organelle is invisible — it can be removed from the tree without affecting visible output:

$$\text{stack}(\ldots, [\ldots, e_{dead}, \ldots]) = \text{stack}(\ldots, [\ldots, \ldots]) \quad \text{when} \quad \forall l \in \mathcal{F}(e): w_{min}(l) > c_w$$

### 10.4 algebraic properties

| property | holds? | reason |
|----------|--------|--------|
| $\oplus$ associative (stacking) | only for fix-sized children | fill-sized children absorb different remainders depending on grouping |
| $\oplus$ commutative | no | left-to-right order determines placement |
| $\otimes$ associative (nesting) | yes | nesting is tree composition — associativity of function composition |
| identity | exists (Rule 2) | single-child fill container |
| inverse | no | layout is not reversible — multiple trees can produce identical coordinates |

the layout algebra is a free algebra over $\Sigma$ with the simplification rules as rewrite rules. it is not a group (no inverse), not a ring (no meaningful addition distributing over nesting), but it is a multi-sorted algebra with well-defined operations and identities

the practical value: before layout computation, apply Rules 1-4 as a simplification pass. this reduces the tree size by eliminating redundant wrappers and dead branches, lowering the constant factor in $\mathcal{O}(n \cdot f_{max})$

### 10.5 termination and confluence

Theorem 10 (termination). the rewrite system $\{R_1, R_2, R_3, R_4\}$ terminates on all inputs

Proof. define a measure $\mu(\mathcal{T}) = (N, D)$ where $N$ = total node count in $\mathcal{T}$ and $D$ = sum of nesting depths across all nodes. order by lexicographic comparison

- $R_1$ (stack flattening): eliminates the inner stack node. $N$ decreases by 1. $D$ decreases (children move up one level). $\mu$ strictly decreases
- $R_2$ (identity elimination): eliminates the wrapper. $N$ decreases by 1. $\mu$ strictly decreases
- $R_3$ (layer collapse): eliminates the layer. $N$ decreases by 1. $\mu$ strictly decreases
- $R_4$ (dead branch): removes a subtree. $N$ decreases by $\geq 1$. $\mu$ strictly decreases

every rule strictly decreases $\mu$. $\mu$ is bounded below by $(1, 0)$ (a single leaf). the system terminates. ∎

Theorem 11 (confluence). the rewrite system $\{R_1, R_2, R_3, R_4\}$ is confluent — all reduction sequences reach the same normal form

Proof. by Newman's lemma, a terminating rewrite system is confluent iff it is locally confluent (every critical pair is joinable). enumerate critical pairs — cases where two rules apply to overlapping redexes:

*$R_1 / R_1$:* two nested stacks both eligible for flattening into the same parent. flattening the outer first, then the inner, vs inner first, then outer: both produce the same flat stack with all children at the same level. joinable

*$R_1 / R_2$:* a single-child fill stack nested in a same-direction stack. $R_1$ flattens (moves the single child up), $R_2$ eliminates the wrapper (replaces with the child). both produce the same result: the child placed directly in the parent. joinable

*$R_2 / R_2$:* nested single-child fill wrappers. eliminating outer vs inner first: both reduce to the innermost element. joinable

*$R_2 / R_3$:* a single-child fill container that is also a single-child layer. both rules eliminate it. same result. joinable

*$R_4 / R_{1,2,3}$:* $R_4$ removes a dead subtree. if the dead subtree is also a candidate for $R_1$, $R_2$, or $R_3$: removing it ($R_4$) produces a smaller tree. simplifying it first ($R_{1,2,3}$) then removing it ($R_4$) produces the same tree (the simplified dead branch is still dead — $c_w$ unchanged). joinable

*$R_4 / R_4$:* two independent dead branches. removal order does not matter. joinable

all critical pairs joinable. by Newman's lemma + termination (Theorem 10), the system is confluent. ∎

Corollary (unique normal form). every element tree has a unique normal form under $\{R_1, R_2, R_3, R_4\}$. two trees are layout-equivalent under these rules iff their normal forms are identical (character-by-character as terms)

### 10.6 completeness of simplification rules

Theorem 12 (completeness relative to coordinate equivalence). Rules 1-4 are complete for the class of coordinate-preserving simplifications that involve removing or merging exactly one node

Proof. enumerate all possible single-node operations that preserve coordinates:

(a) *remove a container node, keeping its children.* the container must be transparent — it does not alter the constraints passed to children or the positions assigned. a fill-sized single-child container is transparent ($R_2$, $R_3$). a same-direction, same-gap, same-align fill-sized stack is transparent for flattening ($R_1$). a container with different parameters (gap, align, direction) is not transparent — removing it changes constraints. these three rules exhaust all transparent-container cases

(b) *remove a leaf or subtree.* only valid when the removal does not affect visible output — the element is dead ($R_4$). this is the only removal that preserves coordinates for other elements (removing a visible element changes fill siblings' sizes)

(c) *merge two containers into one.* this is $R_1$ applied from the other direction — merging sibling stacks into one. but merging two sibling stacks is only valid when they share parameters AND the merged stack's children receive the same constraints. this is exactly the case when one is a single-element fill stack — which is $R_1$

no other single-node operation preserves coordinates. ∎

Limitation. Rules 1-4 are not complete for multi-node operations. example: swapping two fix-sized siblings of equal width preserves coordinates (they occupy the same space regardless of order). this is a valid simplification not captured by Rules 1-4. adding a permutation rule $R_5$ (swap equal-width fix siblings) would extend completeness, but placement order carries semantic meaning (reading direction) — permutation is coordinate-preserving but not semantics-preserving. therefore Rules 1-4 are deliberately restricted to semantics-preserving operations

### 10.7 amortized fold selection

Theorem 13 (amortized $\mathcal{O}(1)$ fold selection). if viewport changes are continuous (differ by at most $\delta g$ per frame where $\delta$ is bounded), fold selection is amortized $\mathcal{O}(1)$

Proof. cache the current conformation index $i^*$ and its boundaries $[w_{min}(l_{i^*}),\; w_{min}(l_{i^*-1}))$. on each frame:

1. if $c_w \in [w_{min}(l_{i^*}),\; w_{min}(l_{i^*-1}))$: no change. cost: 2 comparisons = $\mathcal{O}(1)$
2. if $c_w < w_{min}(l_{i^*})$: fold narrower. check $l_{i^*+1}$: is $c_w \geq w_{min}(l_{i^*+1})$? if yes, update $i^* \leftarrow i^* + 1$. cost: $\mathcal{O}(1)$. if $c_w$ drops below $w_{min}(l_{i^*+1})$ as well, continue checking — but this means the viewport shrank by more than the gap between two consecutive $w_{min}$ values in a single frame
3. if $c_w \geq w_{min}(l_{i^*-1})$: fold wider. symmetric

define the potential $\Phi = i^*$. each conformation change costs $\mathcal{O}(1)$ and changes $\Phi$ by $\pm 1$. over $F$ frames with total conformation changes $k$: total work $= F \cdot \mathcal{O}(1) + k \cdot \mathcal{O}(1)$. since each change moves the index by 1 and the index is bounded by $[1, f_{max}]$, the total number of changes is bounded by the total viewport displacement divided by the minimum gap between consecutive $w_{min}$ values. amortized cost per frame: $\mathcal{O}(1)$

for discontinuous viewport changes (e.g. rotation snap): binary search fallback in $\mathcal{O}(\log f_{max})$. amortized cost remains $\mathcal{O}(1)$ if discontinuities are rare (bounded frequency). ∎

ECS: `FoldCache { current_index: usize, lower_bound: u32, upper_bound: u32 }`. `FoldSystem` reads cache first — 2 comparisons. updates cache only on conformation change

---

## 11. multimodal extension

Theorem 8 (generality of $\Pi$). the protocol $\Pi$ (constrain → occupy → place) generalizes to any bounded measurable domain where organelles are independent

Proof. the protocol requires three properties of the domain:

1. bounded: a finite constraint $c$ exists (maximum extent in the domain). spatial: $c = (c_w, c_h)$ in quanta. audio: $c = (c_{duration}, c_{bandwidth})$ in time × frequency. haptic: $c = (c_{duration}, c_{intensity})$ in time × amplitude. neural: $c = (c_{channels}, c_{rate})$ in channel count × sample rate

2. measurable: organelles report occupied size $s \leq c$. spatial: rectangle area. audio: sound event duration × frequency range. haptic: vibration pattern duration × amplitude envelope. this is $\Phi$ — fix, fill, scale are definable whenever the domain has addition (fix = constant, fill = remainder, scale = fraction)

3. placeable: membrane assigns position $p$ within the domain. spatial: $(p_x, p_y)$. audio: $(t_{start}, f_{center})$. haptic: $(t_{start}, \text{actuator\_id})$. this is $\mathcal{K}$ — stack, grid, layer are definable whenever the domain has ordering (stack = sequential along one axis, grid = lattice on two axes, layer = depth ordering)

the determinism proof (Theorem 2) holds: structural induction on $\mathcal{T}$ uses only the pure-function property of sizing, which holds in any domain. the linear-time proof (Theorem 1) holds: one-directional information flow is a tree-traversal property, independent of domain

the tradeoff is identical. in any domain, the protocol cannot express content-dependent membrane sizing (would require two passes). organelles that exceed constraints use the domain's equivalent of scroll: audio → fade/truncate, haptic → attenuate

The spatial domain is the initial design target; extensions require their own implementation and validation. but $\Pi$ is a resource allocation protocol over trees. the "resource" can be screen area, time, frequency, or any bounded measurable quantity. ∎

| domain | constraint $c$ | size $s$ | position $p$ | $\Phi$ primitives | $\mathcal{K}$ containers |
|--------|---------------|----------|-------------|-------------------|-------------------------|
| spatial | $(c_w, c_h)$ area | $(s_w, s_h)$ | $(p_x, p_y)$ | fix: $k \cdot g$, fill: remainder, scale: $r \cdot c$ | stack: axis, grid: 2D, layer: z |
| audio | $(c_t, c_f)$ time × freq | $(s_t, s_f)$ | $(t_{start}, f_{center})$ | fix: $k$ ms, fill: remaining duration, scale: $r \cdot c_t$ | stack: sequential, grid: time × channel, layer: mix |
| haptic | $(c_t, c_a)$ time × amplitude | $(s_t, s_a)$ | $(t_{start}, \text{actuator})$ | fix: $k$ ms, fill: remaining, scale: $r \cdot c_a$ | stack: sequential, grid: time × body, layer: overlay |

---

## 12. the [[cyb]] layout

Proposed root tree $\mathcal{T}$ for [[cyb]]. Slots project the [21 organs](../../../cyb/anatomy.md); context projects Now, Avatar visualizes the robot, Com emits intent, Sigma manages assets/attachments, and Time composes Log ← Now → Plan. Widgets and ECS entities do not add organs or signing identities.

### 12.1 desktop ($\square = (240g, 135g)$)

grid: columns fix($25g$) fill fix($25g$), rows fix($6g$) fill auto. named areas: context adviser avatar / space space space / bottom-l commander bottom-r

| organelle | type | sizing | $z$ | $\mathcal{U}$ | ECS | role |
|-----------|------|--------|-----|------|-----|------|
| context | frame | fix($25g$) × fix($6g$) | 10 | persistent | `GridArea, Sizing, FoldSet, Trigger::Tap` | Now's inspected context |
| adviser | guidance | fill × fix($6g$) | 40 | guiding | `GridArea, Sizing, Visibility` | voice of [[cyb]] |
| avatar | frame | fix($25g$) × fix($6g$) | 10 | persistent | `GridArea, Sizing, FoldSet, Trigger::Tap` | robot visualization; no acting-subject selection |
| space | ambient | fill × fill, scroll | 0 | ambient | `GridArea, Overflow::Scroll` | where [[particles]] render |
| S (sense) | frame | fix($4g$) × fix($6g$), left center | 10 | persistent | `FixedEdge::Left, Sizing` | [[cyb/sense]]: unread count |
| Σ (sigma) | frame | fix($4g$) × fix($6g$), right center | 10 | persistent | `FixedEdge::Right, Sizing` | [[cyb/sigma]]: balance |
| stars | frame | fix($25g$) × auto | 10 | persistent | `Stack::Vertical, Sizing` | pinned items |
| graph | frame | fix($25g$) × auto | 10 | persistent | `Sizing` | navigation minimap |
| commander | active | fill × fix($6g$) | 20 | active | `GridArea, Sizing` | input + buttons |
| time | frame | fix($25g$) × auto | 10 | persistent | `GridArea, Sizing` | action timeline |
| menu context | overlay | fix($25g$) × fill($\square_h$) | 30 | interrupting | `SlideOverlay::Left, Trigger::Tap` | settings |
| menu avatar | overlay | fix($25g$) × fill($\square_h$) | 30 | interrupting | `SlideOverlay::Right, Trigger::Tap` | robot menu; explicit link to Sigma attachment management |

### 12.2 mobile ($\square = (49g, 106g)$, $\square_w \leq 96g$)

tree $\mathcal{T}$ restructures. grid: columns auto fill auto, rows fix($4g$) fill fix($6g$). areas: context adviser avatar / space space space / stars commander time

| change | desktop → mobile |
|--------|-----------------|
| top bar | fix($6g$) → fix($4g$). context, avatar fold to $l_k$ (icon only) |
| stars | vertical stack → horizontal, max 4 icons |
| graph | bottom-l → inside menu context |
| menus | fix($25g$) → fix($35g$) |
| space padding | $3g$ → $2g$ |

---

## 13. worked example

concrete computation for [[cyb]] desktop, demonstrating every protocol phase

### 13.1 input

$\square = (240g, 135g)$

element tree $\mathcal{T}$:

```
grid (root membrane)
  context     [fix(25g) × fix(6g)]
  adviser     [fill × fix(6g)]
  avatar      [fix(25g) × fix(6g)]
  space       [fill × fill, scroll]
  bottom-l    [fix(25g) × fix(19g)]
    stars     [fix(25g) × fix(10g)]
    graph     [fix(25g) × fix(9g)]
  commander   [fill × fix(6g)]
  time        [fix(25g) × fix(19g)]
```

grid tracks: columns = [fix(25g), fill, fix(25g)]. rows = [fix(6g), fill, auto]

### 13.2 step 1: resolve column tracks

fix columns: $25g + 25g = 50g$. fill column: $240g - 50g = 190g$

column widths: [25g, 190g, 25g]

### 13.3 step 2: resolve row tracks

fix row 1: $6g$. auto row 3: $\max(19g, 6g, 19g) = 19g$. fill row 2: $135g - 6g - 19g = 110g$

row heights: [6g, 110g, 19g]

### 13.4 step 3: constrain

| organelle | grid area | $c = (c_w, c_h)$ |
|-----------|-----------|-------------------|
| context | row 1, col 1 | $(25g, 6g)$ |
| adviser | row 1, col 2 | $(190g, 6g)$ |
| avatar | row 1, col 3 | $(25g, 6g)$ |
| space | row 2, col 1-3 | $(240g, 110g)$ |
| bottom-l | row 3, col 1 | $(25g, 19g)$ |
| commander | row 3, col 2 | $(190g, 6g)$ |
| time | row 3, col 3 | $(25g, 19g)$ |

### 13.5 step 4: occupy

- context: fix(25g) × fix(6g) → $s = (25g, 6g)$
- adviser: fill = 190g, fix(6g) → $s = (190g, 6g)$
- space: fill × fill → $s = (240g, 110g)$, overflow: scroll
- bottom-l: fix(25g) × fix(19g), organelles: stars $(25g, 10g)$, graph $(25g, 9g)$
- commander: fill = 190g, fix(6g) → $s = (190g, 6g)$
- time: fix(25g) × fix(19g) → $s = (25g, 19g)$

no fold triggered — all organelles fit within constraints

### 13.6 step 5: place

| organelle | $p_x$ | $p_y$ | $s_w$ | $s_h$ | $z$ |
|-----------|-------|-------|-------|-------|-----|
| context | $0$ | $0$ | $25g$ | $6g$ | 10 |
| adviser | $25g$ | $0$ | $190g$ | $6g$ | 40 |
| avatar | $215g$ | $0$ | $25g$ | $6g$ | 10 |
| space | $0$ | $6g$ | $240g$ | $110g$ | 0 |
| stars | $0$ | $116g$ | $25g$ | $10g$ | 10 |
| graph | $0$ | $126g$ | $25g$ | $9g$ | 10 |
| commander | $25g$ | $116g$ | $190g$ | $6g$ | 20 |
| time | $215g$ | $116g$ | $25g$ | $19g$ | 10 |

8 tuples. exact coordinates in quanta. deterministic. single pass

verification: $25g + 190g + 25g = 240g$ ✓. $6g + 110g + 19g = 135g$ ✓

---

## 14. system execution order

the layout protocol maps to a directed acyclic graph of ECS systems. the DAG defines the exact execution order — no system runs before its dependencies complete

```
DataFetchSystems ─────────────────────────────┐
  (chain state, cyberank, karma, balances)     │
                                               ▼
                                        EmotionSystem
                                        (§ emotion.md)
                                               │
                                               ▼
FilterSortSystem ──► ConstrainSystem ──► OccupySystem ──► FoldSystem
   (§6.2)               (§4.1)            (§4.2)          (§4.3)
                                                             │
                                                             ▼
                                                      GravitateSystem
                                                         (§9.1)
                                                             │
                                                             ▼
                                                       PlaceSystem
                                                         (§4.4)
                                                             │
                                                             ▼
                                                  EmotionPropagateSystem
                                                    (view → molecule → atom)
                                                             │
                                               ┌─────────────┼─────────────┐
                                               ▼             ▼             ▼
                                        CssRenderSystem  BevyUiRender  Ren3dRender
                                           (§15)          (§15)         (§15)
```

| stage | systems | reads | writes |
|-------|---------|-------|--------|
| 0 | DataFetchSystems | chain RPC, IPFS | `TokenBalances`, `ValidatorSet`, `CyberankMap`, `KarmaMap` |
| 1 | EmotionSystem | `EmotionSource` | `Emotion` |
| 1 | FilterSortSystem | `GridFilter`, `GridSort` | row visibility, row order |
| 2 | ConstrainSystem | parent `OccupiedSize`, `Stack`/`Grid`/`Layer` params | `Constraint { c_w, c_h }` |
| 3 | OccupySystem | `Sizing`, `Constraint` | `OccupiedSize { s_w, s_h }` |
| 4 | FoldSystem | `FoldSet`, `Constraint` | `ActiveFold { index }` |
| 5 | GravitateSystem | `Gravity { focus }` | `Position { z }` |
| 6 | PlaceSystem | `OccupiedSize`, container params | `Position { x, y }` |
| 7 | EmotionPropagateSystem | `Emotion`, element tree | `Emotion` on descendant entities |
| 8 | RenderSystems | `Position`, `OccupiedSize`, `Emotion`, leaf data | frame buffer |

stages 1-1 (EmotionSystem and FilterSortSystem) run in parallel — no data dependency between them. all other stages are sequential. total: 9 stages, $\mathcal{O}(n)$ per stage, $\mathcal{O}(n)$ total

---

## 15. renderers

the layout function outputs $\{(e_i, p_{x_i}, p_{y_i}, s_{w_i}, s_{h_i}, z_i)\}$. renderers consume coordinates:

| world | renderer | engine | mapping | ECS system |
|-------|----------|--------|---------|-----------|
| Portal | Leptos + CSS | browser | grid-template-areas, flexbox | `CssRenderSystem` |
| Terminal | Sugarloaf | GPU text | $(p_x / \text{cell}_w,\; p_y / \text{cell}_h)$ → char position | `TermRenderSystem` |
| Interface | Bevy UI | Taffy | Node, Style, Display::Grid | `BevyUiRenderSystem` |
| 3D | Bevy 3D | [[Ren]] | $(p_x, p_y, z)$ projected onto surfaces | `Ren3dRenderSystem` |

---

## 16. validation

Acceptance requires reproducible tests against declared renderer versions and hardware. Record observed results in `prysm/audit/`, separately from these requirements.

For each renderer claiming conformance, test these invariants against the running implementation:

- I1 (determinism): same page load → same coordinates. screenshot comparison across sessions
- I2 (single-pass): layout profiler must show no re-measurement
- I3 (linear time): layout time measured across pages with 10 to 10,000 organelles. check scaling under declared fold-set and shaping bounds
- I4 (constraint respect): automated bounds checking — no organelle overflows its membrane
- I5 (quantum alignment): inspector must check $g$-alignment on non-text organelles
- I6 (z monotonicity): modal above adviser above commander above space
- I7 (fold legibility): test every molecule at $s_{min}$ on mobile viewport
- I8 (renderer independence): same coordinates across Leptos and Bevy UI
- I9 (semantic completeness): every interactive organelle has a non-empty label. CI check against component catalog

### performance invariant

I10 (frame budget). layout computation completes within 2ms per frame on target hardware

$$t_{layout}(\mathcal{T}) \leq 2\text{ms} \quad \text{for} \quad |\mathcal{T}| \leq n_{max}$$

$n_{max}$ — maximum organelle count per frame. target: $n_{max} = 10{,}000$

derivation: frame budget at 60fps = 16.67ms. layout receives 2ms (12% of frame). remaining: data fetch (2ms), emotion (0.5ms), interaction (0.5ms), render (10ms), overhead (1.67ms)

from Theorem 1: $t_{layout} = k \cdot n \cdot f_{max}$. from Theorem 13: amortized fold is $\mathcal{O}(1)$, so effectively $t_{layout} = k \cdot n$. to hit 2ms at $n = 10{,}000$: $k \leq 200\text{ns}$ per organelle. this is achievable — each organelle operation is arithmetic (comparisons, additions) on cache-local ECS data. Bevy's archetype-based ECS processes millions of entities per frame

at $n > 10{,}000$: viewport culling reduces visible organelles. only organelles within the visible scroll region + one screen-height of buffer participate in layout. the protocol's single-pass structure means culled subtrees are skipped entirely (their constraint is computed but they are not placed)

This paper defines intended constraints. A working screen alone does not prove these invariants.

---

## 17. temporal stability

why this protocol will not become obsolete

the protocol is renderer-independent. it outputs coordinate tuples in quanta $g$. when a new rendering technology appears, it consumes the same tuples. prysm is bound to rectangles and arithmetic — both predate computing and will outlast it

the axioms are mathematical, not technological. $\Pi$ is a tree traversal protocol. $\Phi$ contains three relations between part and whole: independence, complement, similarity. $\mathcal{K}$ contains three ways to partition or overlay rectangles. none references a specific language, OS, device, or rendering API

the protocol is minimal. three axioms. Theorem 3 proves each sizing primitive is necessary. Conjecture 1 argues each container type is necessary. minimal systems survive because they have fewer parts that can become irrelevant

the inputs are universal. the protocol requires: an element tree, a viewport size in quanta, and sizing parameters per organelle. every display device provides a viewport. every interface has organelles arranged in space

what would invalidate it. a fundamentally non-rectangular display would require extending $\mathcal{K}$. the 3D extension (§9) addresses projection. a display with no spatial dimension falls outside layout scope

---

## 18. related work

| system | year | what prysm takes | what prysm rejects |
|--------|------|------------------|-------------------|
| TeX (Knuth & Plass) | 1981 | box-glue decomposition: fix ≈ box, fill ≈ glue. proven formalism | global optimization (O(n²)). penalties (line-breaking is outside scope) |
| Cassowary (Badros et al.) | 2001 | formal rigor of constraint specification | constraint solving itself — exponential worst case, prysm requires O(n) |
| Cassius (Panchekha et al.) | 2016 | proof that layout semantics can be mechanically verified (QF_LRA) | CSS complexity — prysm must be simpler than CSS to remain verifiable |
| Flutter RenderBox | 2017 | single-pass protocol: constraints down, sizes up. the direct ancestor of $\Pi$ | unproven O(n) claim. no fold. no renderer independence. no ECS |
| Taffy | 2022 | Rust implementation of CSS Grid + Flexbox. implementation target for Bevy UI | CSS semantics — prysm defines its own protocol, Taffy implements it |
| Morphorm | 2023 | ambition of a simpler single-pass layout | no grid, no fold, no formal analysis |
| VLSI floorplanning | 1988 | proof that pure stacking (slicing) is incomplete for rectangular layouts (Kozminski & Kinnen). motivates grid-with-spans in $\mathcal{K}$ | VLSI-specific constraints |
| CuTe Layout Algebra (NVIDIA) | 2024 | demonstration that layout can be formalized as algebra. closest formal precedent | GPU memory domain — different problem, but same mathematical structure |
| cellular automata | 1970 | computational completeness of simple local rules on grids. prysm's container tree is a tree cellular automaton | — |
| biological cell | — | membrane-organelle relationship. the physical metaphor: membrane constrains, organelle occupies. 3.5 billion years of validation | — |

This paper explores a common algebra for spatial composition, fold behavior and renderer mappings. The related-work table is research context, not evidence of novelty or proof completion.

---

## 19. Research and verification obligations

### Candidate results

| # | problem | resolution |
|---|---------|-----------|
| 1 | completeness of $\mathcal{K}$ | Theorem 5 (§6.4) |
| 2 | fold set derivation | Theorem 6 (§4.3a) |
| 3 | urgency-gravity composition | Theorem 7 (§9.4) |
| 4 | layout algebra | §10 |
| 5 | multimodal extension | Theorem 8 (§11) |
| 6 | branching fold sets | Theorem 9 (§4.3a) |
| 7 | algebraic normal form | Theorems 10-11 (§10.5) |
| 8 | algebra completeness | Theorems 12, 14 (§10.6, §20) |
| 9 | $\mathcal{O}(1)$ fold selection | Theorem 13 (§10.7) |
| 10 | multi-node decomposition | Theorem 14 (§19) |
| 11 | optimal $g$ | §8: derived from visual acuity + Fitts's law + quantum alignment |
| 12 | machine verification | proposed module structure below; checked artifacts required |

### formal verification roadmap

The 14 theorem statements and arguments are semi-formal research material. Before treating them as contracts, declare their exact domains and check the proofs. In particular, variable-width fold optimality, fill-dependent rewrites and general completeness need independent review; a prose proof does not settle these assumptions. Proposed Lean module structure:

```
Prysm/
  Layout/
    Protocol.lean      -- Π: constrain → occupy → place (Theorems 1, 2)
    Sizing.lean        -- Φ: fix, fill, scale (Theorems 3, 4)
    Container.lean     -- K: stack, grid, layer (Theorem 5)
    Fold.lean          -- fold derivation (Theorems 6, 9)
    Gravity.lean       -- urgency-gravity composition (Theorem 7)
    Multimodal.lean    -- domain generalization (Theorem 8)
    Algebra.lean       -- term algebra, rules, normal form (Theorems 10-13)
```

key formalization challenge: Theorems 1-2 require modeling the DFS traversal as a function on inductive trees — standard in Lean. Theorem 5 requires formalizing rectangular partitions as finite sets of axis-aligned rectangles — straightforward with `Finset` and `Prod`. Theorem 11 (confluence) requires formalizing the critical pair analysis — the `Mathlib.Order.RewriteSystem` library provides Newman's lemma

estimated effort: ~2000 lines of Lean for the core theorems. the proofs are constructive — no axiom of choice required

Proposed verification responsibilities:
- `Protocol.lean` — element tree, layout function, Theorems 1-2
- `Sizing.lean` — Φ primitives, Theorems 3-4
- `Container.lean` — coordinate-collection construction, Theorem 5
- `Fold.lean` — greedy and Pareto fold derivation, Theorems 6, 9
- `Gravity.lean` — urgency-gravity composition, Theorem 7
- `Multimodal.lean` — LayoutDomain typeclass, Theorem 8
- `Algebra.lean` — rewrite system, Theorems 10-14

Completion criteria: retain exact checked sources, compiler/library revisions and command logs in an audit; distinguish proven statements from axioms and unfinished proofs. No claim that all gaps are merely arithmetic is part of this specification.

### multi-node decomposition

Theorem 14 (multi-node decomposition). every semantics-preserving multi-node simplification decomposes into a finite sequence of single-node rules from $\{R_1, R_2, R_3, R_4\}$

Proof. let $T \to T'$ be a semantics-preserving simplification that modifies $k > 1$ nodes simultaneously. we show it decomposes into $\leq k$ single-node steps

a multi-node operation on a tree is one of:
- (a) multiple independent single-node operations at disjoint subtrees
- (b) a chain of dependent operations along an ancestor-descendant path
- (c) a combination of (a) and (b)

case (a): independent operations commute (they affect disjoint subtrees — no data dependency). apply them in any order. each is a single-node operation, hence covered by Theorem 12 and expressible as some $R_i$. sequence length: $k$

case (b): a chain of $k$ operations along a path $e_1 \succ e_2 \succ \cdots \succ e_k$ (ancestor to descendant). apply bottom-up: first simplify $e_k$, then $e_{k-1}$, etc. each step is a single-node operation on the current tree. bottom-up order ensures that when we simplify $e_i$, its subtree is already in normal form (by Theorem 11, the normal form is unique). each step is expressible as $R_i$

case (c): decompose into maximal independent chains. apply chains in any order (independence). within each chain, apply bottom-up. total: $\leq k$ steps

the set $\{R_1, R_2, R_3, R_4\}$ is finite. the decomposition produces a finite sequence from a finite set. no additional rules needed. ∎

Corollary. the rewrite system $\{R_1, R_2, R_3, R_4\}$ is complete for all semantics-preserving simplifications (single-node and multi-node). the answer to the open question: the set is finite (4 rules), and it is complete

---

## 20. scope and catalog

this paper defines spatial placement: how elements are sized and positioned. it depends on nothing above it. everything visible depends on it

companion specifications:
- [[prysm/emotion]] — the emotion function: how protocol state maps to color
- [[prysm/interaction]] — the interaction protocol: how input events produce state transitions
- [[prysm]] component catalog — atoms, molecules, views, fold sets, visual parameters

this paper does not define what elements exist, how they look, or how they behave. those definitions live in the companion specifications above