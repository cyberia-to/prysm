// ---
// tags: prysm, rust, cli
// crystal-type: source
// crystal-domain: comp
// ---
//! prysm — read a tade stream as prysm reads it: what each frame *becomes*.
//!
//!   prysm render [file]    decode a stream → the molecule each frame renders as
//!   prysm molecules        the routing table: (sigil, render) → molecule
//!
//! prysm proper renders meaning to a GPU surface (Bevy). This CLI is its terminal
//! shadow: it applies the same `(sigil, render) → molecule` routing and shows the
//! content as text — text as text, a log as a log line, a table as a note — so
//! you can read a stream's *meaning* where `tade inspect` shows its envelope.
//!
//! SYNC: the routing table below mirrors `prysm/system/rs/scrollback.rs`. The
//! test `the_printed_table_holds_exactly_the_renderers_arms` reads that file's
//! own dispatch arms, so an arm added there and forgotten here fails the build
//! rather than printing `raw`. The copy is local on purpose: step 3 of the
//! tade·nox·spark·prysm proposal ("catalog → spark: prysm reads spark; it does
//! not hold a vocabulary of its own") moves the `(sigil, render)` catalog into
//! spark, and no spark crate exists to read yet. When one does, this table and
//! the scrollback dispatch both read it instead of carrying rows. Step 5 of the same proposal
//! (tty → ANSI, pipe → tade chunks ending in STATUS) is not implemented here:
//! piped output is plain text.

use std::io::{self, IsTerminal, Read};

use tade::{render, sigil, Chunk, ReadResult, Reader};

fn tty() -> bool {
    io::stdout().is_terminal()
}
fn paint(code: &str, s: &str) -> String {
    if tty() { format!("\x1b[{code}m{s}\x1b[0m") } else { s.to_string() }
}
fn dim(s: &str) -> String {
    paint("90", s)
}
fn cyan(s: &str) -> String {
    paint("36", s)
}
fn green(s: &str) -> String {
    paint("32", s)
}
fn yellow(s: &str) -> String {
    paint("33", s)
}
fn bold(s: &str) -> String {
    paint("1", s)
}
fn red(s: &str) -> String {
    paint("31", s)
}

const LOGO: &str = "\
\x1b[31m██████╗ ██████╗ ██╗   ██╗███████╗███╗   ███╗\x1b[0m
\x1b[33m██╔══██╗██╔══██╗╚██╗ ██╔╝██╔════╝████╗ ████║\x1b[0m
\x1b[32m██████╔╝██████╔╝ ╚████╔╝ ███████╗██╔████╔██║\x1b[0m
\x1b[36m██╔═══╝ ██╔══██╗  ╚██╔╝  ╚════██║██║╚██╔╝██║\x1b[0m
\x1b[34m██║     ██║  ██║   ██║   ███████║██║ ╚═╝ ██║\x1b[0m
\x1b[35m╚═╝     ╚═╝  ╚═╝   ╚═╝   ╚══════╝╚═╝     ╚═╝\x1b[0m";

/// The `(sigil, render) → molecule` routing — one row per arm of prysm's
/// scrollback dispatch, in its order. `molecule()` looks a frame up here and
/// `prysm molecules` prints these rows, so the printed table cannot drift from
/// the routing, and the labels come from tade's own names rather than by hand.
/// The rows themselves are checked against the renderer's source in `tests`.
const ROUTES: &[(u8, u8, &str)] = &[
    (sigil::HAX, render::TEXT, "text"),
    (sigil::SIG, render::TEXT, "annotation"),
    (sigil::PAT, render::TEXT, "neuron"),
    (sigil::DOT, render::LOG, "log"),
    (sigil::ZAP, render::ERROR, "error"),
    (sigil::DOT, render::STATUS, "status"),
    (sigil::DOT, render::PROGRESS, "progress"),
    (sigil::ZAP, render::COMPONENT, "action"),
    (sigil::BUC, render::COMPONENT, "stat"),
    (sigil::LUS, render::COMPONENT, "row"),
    (sigil::BAR, render::COMPONENT, "component"),
    (sigil::FAS, render::COMPONENT, "scope"),
    (sigil::HAX, render::TABLE, "table"),
    (sigil::FAS, render::STRUCT, "scope"),
    (sigil::COL, render::STRUCT, "pair"),
];

/// A frame prysm has no molecule for: the scrollback shows it as a debug label.
const RAW: &str = "raw";

fn molecule(sig: u8, ren: u8) -> &'static str {
    ROUTES
        .iter()
        .find(|(s, r, _)| *s == sig && *r == ren)
        .map_or(RAW, |&(_, _, mol)| mol)
}

/// `# hax`, `t text` — the byte as it appears in a stream, with tade's name.
fn label(byte: u8, name: &str) -> String {
    format!("{} {name}", byte as char)
}

fn banner() {
    if !tty() {
        return;
    }
    println!("{LOGO}");
    println!("{}", paint("37", "    the visual protocol"));
    println!("{}", dim("\n    meaning, not pixels · molecules decoded from tade\n"));
}

fn help() {
    banner();
    println!("{}", dim("commands"));
    println!("  {}   {}", bold("render [file]"), dim("decode a stream → the molecule each frame becomes (stdin if no file)"));
    println!("  {}       {}", bold("molecules"), dim("the routing table: (sigil, render) → molecule"));
}

/// A payload preview: text-bearing renders decode to a string, the rest to hex.
fn content(c: &Chunk) -> String {
    let textual = matches!(
        c.render,
        render::TEXT | render::LOG | render::ERROR | render::STATUS | render::INPUT | render::TOKEN
    );
    if textual {
        String::from_utf8_lossy(&c.payload).chars().take(60).collect::<String>().replace('\n', "⏎")
    } else if c.payload.is_empty() {
        "·".into()
    } else {
        let hex: String = c.payload.iter().take(10).map(|b| format!("{b:02x}")).collect();
        format!("{hex}{}", if c.payload.len() > 10 { "…" } else { "" })
    }
}

fn cmd_render(path: Option<&str>) {
    let mut bytes = Vec::new();
    let read = match path {
        Some(p) => std::fs::File::open(p).and_then(|mut f| f.read_to_end(&mut bytes)),
        None => io::stdin().read_to_end(&mut bytes),
    };
    if let Err(e) = read {
        eprintln!("  {}: {e}", red("error"));
        std::process::exit(1);
    }

    let mut reader = Reader::new();
    reader.feed(&bytes);
    println!("  {}   {}  {}", dim("#"), dim(&format!("{:<11}", "molecule")), dim("content"));
    let mut n = 0u64;
    let mut framed = 0usize;
    // Pending is not Eof: the reader stopped on bytes that do not form a frame.
    let mut truncated = false;
    loop {
        match reader.next_chunk() {
            ReadResult::Chunk(c) => {
                let mol = molecule(c.sigil, c.render);
                let name = if mol == RAW { dim(&format!("{mol:<11}")) } else { cyan(&format!("{mol:<11}")) };
                println!("  {:>2}   {}  {}", yellow(&n.to_string()), name, content(&c));
                n += 1;
                framed += c.encode().len();
            }
            ReadResult::Pending => {
                truncated = true;
                break;
            }
            ReadResult::Eof => break,
        }
    }
    if n == 0 {
        println!("  {}", dim("(no frames)"));
    } else {
        println!("  {}", dim(&format!("{n} frame(s)")));
    }
    if truncated {
        let loose = bytes.len().saturating_sub(framed);
        eprintln!(
            "  {}: {loose} byte(s) outside a complete frame — the input does not end on a frame boundary",
            red("truncated")
        );
        std::process::exit(1);
    }
}

fn cmd_molecules() {
    println!("{}", dim("(sigil, render) → molecule"));
    println!("  {}  {}  {}", dim(&format!("{:<8}", "sigil")), dim(&format!("{:<14}", "render")), dim("molecule"));
    for &(sig, ren, mol) in ROUTES {
        let s = label(sig, sigil::name(sig));
        let r = label(ren, render::name(ren));
        println!("  {}  {}  {}", cyan(&format!("{s:<8}")), green(&format!("{r:<14}")), bold(mol));
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("render" | "read" | "cat") => cmd_render(args.get(1).map(String::as_str)),
        Some("molecules" | "routes") => cmd_molecules(),
        Some("help" | "--help" | "-h") | None => help(),
        Some(other) => {
            eprintln!("  {}: {other}  (try: prysm help)", dim("unknown"));
            std::process::exit(2);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The renderer's own dispatch, read out of its source at compile time —
    /// not a second copy to keep in step. An arm reads as the pair of names
    /// tade gives its bytes: `(sigil::HAX, render::TEXT)` is `("hax", "text")`,
    /// which is what `sigil::name`/`render::name` return for a route's bytes.
    /// So an arm added to the renderer and forgotten in ROUTES fails here.
    ///
    /// The parse reads one `(sigil::X, render::Y)` pair per arm line, up to the
    /// `_` fallthrough. An arm written as an or-pattern or behind a guard would
    /// be read as its first pair only; the dispatch has had one pair per arm
    /// since it was written, and this limit is recorded in
    /// `audit/cli-routing.md`.
    const SCROLLBACK: &str = include_str!("../../system/rs/scrollback.rs");

    fn dispatch_arms() -> Vec<(String, String)> {
        let body = SCROLLBACK
            .split_once("match (chunk.sigil, chunk.render) {")
            .expect("dispatch matches on (chunk.sigil, chunk.render)")
            .1
            .split_once("\n        _ =>")
            .expect("the dispatch ends in a fallthrough arm")
            .0;
        body.lines()
            .filter_map(|line| {
                let pair = line.trim().strip_prefix("(sigil::")?.split_once(')')?.0;
                let (sig, ren) = pair.split_once(", render::")?;
                Some((sig.trim().to_lowercase(), ren.trim().to_lowercase()))
            })
            .collect()
    }

    fn byte_named(name: &str, all: &[u8], namer: fn(u8) -> &'static str) -> u8 {
        *all.iter()
            .find(|&&b| namer(b) == name)
            .unwrap_or_else(|| panic!("tade names no sigil or render {name}"))
    }

    #[test]
    fn the_printed_table_holds_exactly_the_renderers_arms() {
        let arms = dispatch_arms();
        assert!(
            !arms.is_empty(),
            "no arm was read out of system/rs/scrollback.rs — the parse, not the routing, is broken"
        );
        let routes: Vec<(String, String)> = ROUTES
            .iter()
            .map(|&(sig, ren, _)| (sigil::name(sig).to_string(), render::name(ren).to_string()))
            .collect();
        assert_eq!(
            routes, arms,
            "ROUTES and the scrollback dispatch drifted apart, in content or in order"
        );
    }

    #[test]
    fn every_scrollback_arm_has_its_molecule() {
        for (sig, ren) in dispatch_arms() {
            let s = byte_named(&sig, &sigil::ALL, sigil::name);
            let r = byte_named(&ren, &render::ALL, render::name);
            assert_ne!(
                molecule(s, r),
                RAW,
                "({sig} {ren}) is drawn by the renderer but prysm prints it as {RAW}"
            );
        }
    }

    #[test]
    fn a_frame_outside_the_routing_is_raw() {
        assert_eq!(molecule(sigil::KET, render::AUDIO), RAW);
        assert_eq!(molecule(sigil::HAX, render::MOVIE), RAW);
    }

    #[test]
    fn every_route_names_a_valid_sigil_and_render() {
        for &(sig, ren, _) in ROUTES {
            assert!(sigil::is_valid(sig), "{} is not a sigil", sig as char);
            assert!(render::is_valid(ren), "{} is not a render", ren as char);
            assert_ne!(sigil::name(sig), "unknown");
            assert_ne!(render::name(ren), "unknown");
        }
    }

    #[test]
    fn a_label_is_the_byte_then_its_name() {
        assert_eq!(label(sigil::HAX, sigil::name(sigil::HAX)), "# hax");
        assert_eq!(label(render::TABLE, render::name(render::TABLE)), "T table");
    }
}
