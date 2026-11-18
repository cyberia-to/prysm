---
tags: prysm, audit, cli
crystal-type: record
crystal-domain: comp
---

# the CLI's routing against the scrollback dispatch — 2026-11-18

Subject: `cli/src/main.rs` (`prysm render`, `prysm molecules`) against
`system/rs/scrollback.rs` `dispatch`, the renderer it mirrors.

Method: read both tables and compared them arm by arm; the comparison is also a
test, `cli/src/main.rs` `tests::the_printed_table_holds_exactly_the_renderers_arms`,
which no longer holds a copy of anything. It reads `system/rs/scrollback.rs`
through `include_str!` and parses the `(sigil::X, render::Y)` pair of every arm
of `dispatch` up to the `_` fallthrough, then compares that sequence against
`ROUTES` named through `tade::sigil::name` and `tade::render::name` — the names
tade returns are the constants' own idents in lower case. An arm added to the
renderer and forgotten in the CLI now fails the test instead of printing `raw`.
`tests::every_scrollback_arm_has_its_molecule` resolves each parsed arm back to
its bytes and asserts the CLI draws it as something other than `raw`.

Limit of that method: the parse reads one pair per arm line. An arm written as
an or-pattern (`(a, b) | (c, d) =>`) or behind a guard would be read as its
first pair only, and a renderer that stopped matching on
`(chunk.sigil, chunk.render)` would fail the test with a parse message rather
than a drift message. The dispatch has carried one pair per arm since it was
written; if that changes, the parse changes with it.

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

Corroboration of the tade API the CLI calls — read in
`tade/impl/rust/src/lib.rs` at this commit, not compiled: `Reader`
(`lib.rs:274`), `ReadResult` (`lib.rs:261`), `Chunk` (`lib.rs:102`),
`Chunk::encode` (`lib.rs:180`), `sigil::name` (`lib.rs:36`), `sigil::is_valid`
(`lib.rs:45`), `render::name` (`lib.rs:74`), `render::is_valid` (`lib.rs:85`),
`decode_nested` (`lib.rs:383`). Every symbol the binary names exists in the
dependency it names.

Not covered by this record: no build and no run of the binary is recorded here.
The crate is a member of the root workspace, so `cargo build` and `cargo test`
at the repository root cover it; the output of `prysm render` and
`prysm molecules` on a live stream has not been captured yet. Nothing in this
branch was compiled on the owner's desk — the build gate's run on the pushed
commit is the only evidence of compilation, and it is still outstanding.

Also not covered: step 5 of the tade·nox·spark·prysm proposal — a CLI emits a
tade chunk stream ending in a `STATUS` chunk when piped. `prysm render` writes
plain text on both channels today.
