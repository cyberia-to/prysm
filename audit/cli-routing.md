---
tags: prysm, audit, cli
crystal-type: record
crystal-domain: comp
---

# the CLI's routing against the scrollback dispatch — 2026-11-18

Subject: `cli/src/main.rs` (`prysm render`, `prysm molecules`) against
`system/rs/scrollback.rs` `dispatch`, the renderer it mirrors.

Method: read both tables and compared them arm by arm; the comparison is also a
test, `cli/src/main.rs` `tests::the_printed_table_holds_exactly_the_routing`,
which holds an independent copy of the dispatch arms and fails when the two
drift apart.

Observed: the dispatch carries fifteen arms. `ROUTES` carried eleven and is now
the same fifteen, in the dispatch's order:

| sigil | render | molecule |
|-------|--------|----------|
| `# hax` | `t text` | text |
| `~ sig` | `t text` | annotation |
| `@ pat` | `t text` | neuron |
| `. dot` | `l log` | log |
| `! zap` | `e error` | error |
| `. dot` | `x status` | status |
| `. dot` | `p progress` | progress |
| `! zap` | `c component` | action |
| `$ buc` | `c component` | stat |
| `+ lus` | `c component` | row |
| `\| bar` | `c component` | component |
| `/ fas` | `c component` | scope |
| `# hax` | `T table` | table |
| `/ fas` | `s struct` | scope |
| `: col` | `s struct` | pair |

Not covered by this record: no build and no run of the binary is recorded here.
The crate is a member of the root workspace, so `cargo build` and `cargo test`
at the repository root cover it; the output of `prysm render` and
`prysm molecules` on a live stream has not been captured yet.

Also not covered: step 5 of the tade·nox·spark·prysm proposal — a CLI emits a
tade chunk stream ending in a `STATUS` chunk when piped. `prysm render` writes
plain text on both channels today.
