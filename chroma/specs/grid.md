---
tags: prysm, cyb
crystal-type: pattern
crystal-domain: cyber
status: proposed-ui
---

Proposed UI contract under [composition](../../system/specs/composition.md). Layouts, ECS records and interactions below specify intended behavior, not shipped coverage.

screen layout grid for [[prysm]]

The proposed coordinate system of [[cyb]]. Every element has a grid address. The CSS below is a web mapping of the layout, not the native runtime contract: Cyb's root runtime is Bevy, and WebView is reserved for Portal web content. Chroma slots are views of the [21 organs](../../../cyb/anatomy.md), not new organs.

## viewports

| mode | baseline | breakpoint |
|------|----------|------------|
| desktop | 1920 × 1080 | > 768px |
| mobile | 390 × 844 | <= 768px |

## desktop

```
+----------+-------------------------+----------+
| context  |        adviser          |  avatar  |
+          +                         +          +
| menu     |                         | menu     |
| context  |                         | avatar   |
|          |                         |          |
|          |         space           |          |
|S         |                         |         Σ|
|          |                         |          |
|          |                         |          |
|          |                         |          |
+----------+-------------------------+----------+
|  stars   |       | button |        |   time   |
|  graph   |        commander        |          |
+----------+-------------------------+----------+
```

```css
.cyb {
  display: grid;
  grid-template-areas:
    "context  adviser   avatar"
    "space    space     space"
    "bottom-l commander bottom-r";
  grid-template-columns: 200px 1fr 200px;
  grid-template-rows: 48px 1fr auto;
  height: 100dvh;
}

.context   { grid-area: context; }
.adviser   { grid-area: adviser; }
.avatar    { grid-area: avatar; }
.space     { grid-area: space; overflow-y: auto; padding: 24px; }
.bottom-l  { grid-area: bottom-l; display: flex; flex-direction: column; }
.commander { grid-area: commander; height: 48px; }
.bottom-r  { grid-area: bottom-r; }
```

### zones

| zone | grid-area | size | description |
|------|-----------|------|-------------|
| context | context | 200 × 48 | Now's displayed context. what is being viewed; independent of the acting attachment. tap → context menu |
| adviser | adviser | 1fr × 48 (fills remaining width) | voice of [[cyb]]. highlights danger, shows errors, gives suggestions. hidden when empty |
| avatar | avatar | 200 × 48 | the robot's visualization. attachment selection belongs to Sigma, not this image |
| space | space | spans all 3 columns × 1fr (fills remaining height) | everything exists in space. [[particle]] views, profiles, search, transactions. scrollable |
| S (sense) | — (position: fixed) | 32 × 48 | button-widget on left edge, vertical center. unread count. tap → navigates to [[cyb/sense]] page |
| Σ (sigma) | — (position: fixed) | 32 × 48 | button-widget on right edge, vertical center. current balance. tap → navigates to [[cyb/sigma]] page |
| stars | bottom-l (top) | 200 × auto | pinned typed destinations, configured through the robot's settings/Soul |
| graph | bottom-l (bottom) | 200 × auto | navigation minimap. position in [[cybergraph]] |
| commander | commander | 1fr × 48 (fills remaining width) | [Com](com.md). input field with buttons inside; emits search or authorized action intent |
| time | bottom-r | 200 × auto | Log projection with a link to Time (Log ← Now → Plan); timestamps retain their source |

### S and Σ positioning

```css
.sense {
  position: fixed;
  left: 0;
  top: 50%;
  transform: translateY(-50%);
  width: 32px;
  height: 48px;
  z-index: 10;
}

.sigma {
  position: fixed;
  right: 0;
  top: 50%;
  transform: translateY(-50%);
  width: 32px;
  height: 48px;
  z-index: 10;
}
```

### overlays (slide from edges)

```css
.menu-context {
  position: fixed;
  left: 0; top: 0;
  width: 200px; height: 100dvh;
  transform: translateX(-100%);    /* hidden by default */
  transition: transform 200ms;
  z-index: 30;
}
.menu-context.open { transform: translateX(0); }

.menu-avatar {
  position: fixed;
  right: 0; top: 0;
  width: 200px; height: 100dvh;
  transform: translateX(100%);
  transition: transform 200ms;
  z-index: 30;
}
.menu-avatar.open { transform: translateX(0); }
```

## mobile

```
+---------+-------------------+---------+
| context |      adviser      | avatar  |
+---------+                   +---------+
|  menu   |                   |  menu   |
| context |                   | avatar  |
|         |                   |         |
|         |                   |         |
|S        |       space       |        Σ|
|         |                   |         |
|         |                   |         |
|         |                   |         |
+---------+-------------------+---------+
| S| S|S|S|    commander      |  time   |
+-+--+--+-+-------------------+---------+
```

```css
@media (max-width: 768px) {
  .cyb {
    grid-template-areas:
      "context  adviser  avatar"
      "space    space    space"
      "stars    commander time";
    grid-template-columns: auto 1fr auto;
    grid-template-rows: 32px 1fr 48px;
  }

  .space { padding: 16px; }
  .stars { display: flex; gap: 4px; max-width: 128px; } /* 4 icons × 32px */

  .menu-context { width: 280px; }
  .menu-avatar  { width: 280px; }
}
```

| zone | mobile size | difference from desktop |
|------|------------|------------------------|
| context | 32 × 32 | icon only |
| adviser | 1fr × 32 | narrower |
| avatar | 32 × 32 | icon only |
| space | spans all 3 columns, 16px padding | same logic, tighter padding |
| S, Σ | 32 × 32, fixed on edges | same |
| stars | auto × 48, max 4 pinned destination icons | inline in bottom row, no graph |
| commander | 1fr × 48 | shares bottom row with stars and time |
| time | auto × 48 | compressed column |
| graph | inside menu context | not visible on main grid |
| menus | slide from edges, 280px wide | wider than desktop 200px |

## 8px grid

all spacing snaps to 8px

| px | use |
|----|-----|
| 8 | minimum gap, icon spacing |
| 16 | mobile padding |
| 24 | desktop padding |
| 32 | S/Σ buttons, mobile context/avatar |
| 48 | commander height, button height, row height, top row height |
| 200 | desktop side columns, desktop menu overlay width |
| 280 | mobile menu overlay width |

## z-index

| z | layer |
|---|-------|
| 0 | space |
| 10 | grid frame (context, avatar, S, Σ, stars, graph, time) |
| 20 | commander |
| 30 | menus (slide-out overlays) |
| 40 | [[prysm/tooltip]], [[prysm/adviser]] |
| 50 | modal (confirmations, destructive actions) |
