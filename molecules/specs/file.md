---
tags: prysm, file, particle
crystal-type: pattern
crystal-domain: cyber
status: proposed-ui
---

Proposed UI contract under [composition](../../system/specs/composition.md). Layouts, ECS records and interactions below specify intended behavior, not shipped coverage.

# file molecule

`file = particle + data`. The frame is the particle card; the data slot is to
hold whatever a spark resolves for the file. No spark crate exists in this tree
yet, so nothing fills the slot: `molecules/rs/file.rs` spawns the card and the
frame around it, and the data surface is proposed.

Spacetime is to mount this when a `cyb://file/<hex>` address is standing and a
spark has resolved. Neither the address router nor the spark is in this
repository; the mount is the intended behavior, not a shipped one.
