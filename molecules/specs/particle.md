---
tags: prysm, particle
crystal-type: pattern
crystal-domain: cyber
status: proposed-ui
---

Proposed UI contract under [composition](../../system/specs/composition.md). Layouts, ECS records and interactions below specify intended behavior, not shipped coverage.

# particle molecule

Identity of a protocol particle: name, short hash, optional rank. No bytes, no
spark.

Intended callers: now / space chroma, the name cell of a memory table, the file
header, axon neighbors. Of these only the file header has a renderer in this
tree — `molecules/rs/file.rs` calls `particle_card::spawn`. The rest are
proposed.
