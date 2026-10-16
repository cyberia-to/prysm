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
//! SYNC: the routing table below mirrors `prysm/system/rs/scrollback.rs` — keep
//! them in step.

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

/// The `(sigil, render) → molecule` routing — mirrors prysm's scrollback dispatch.
fn molecule(sig: u8, ren: u8) -> &'static str {
    match (sig, ren) {
        (sigil::HAX, render::TEXT) => "text",
        (sigil::SIG, render::TEXT) => "annotation",
        (sigil::PAT, render::TEXT) => "neuron",
        (sigil::DOT, render::LOG) => "log",
        (sigil::ZAP, render::ERROR) => "error",
        (sigil::DOT, render::STATUS) => "status",
        (sigil::DOT, render::PROGRESS) => "progress",
        (sigil::ZAP, render::COMPONENT) => "action",
        (sigil::BAR, render::COMPONENT) => "component",
        (sigil::FAS, render::COMPONENT) => "scope",
        (sigil::HAX, render::TABLE) => "table",
        _ => "raw",
    }
}

const ROUTES: &[(&str, &str, &str)] = &[
    ("# hax", "t text", "text"),
    ("~ sig", "t text", "annotation"),
    ("@ pat", "t text", "neuron"),
    (". dot", "l log", "log"),
    ("! zap", "e error", "error"),
    (". dot", "x status", "status"),
    (". dot", "p progress", "progress"),
    ("! zap", "c component", "action"),
    ("| bar", "c component", "component"),
    ("/ fas", "c component", "scope"),
    ("# hax", "T table", "table"),
];

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
    loop {
        match reader.next_chunk() {
            ReadResult::Chunk(c) => {
                let mol = molecule(c.sigil, c.render);
                let name = if mol == "raw" { dim(&format!("{mol:<11}")) } else { cyan(&format!("{mol:<11}")) };
                println!("  {:>2}   {}  {}", yellow(&n.to_string()), name, content(&c));
                n += 1;
            }
            ReadResult::Pending | ReadResult::Eof => break,
        }
    }
    if n == 0 {
        println!("  {}", dim("(no frames)"));
    } else {
        println!("  {}", dim(&format!("{n} frame(s)")));
    }
}

fn cmd_molecules() {
    println!("{}", dim("(sigil, render) → molecule"));
    println!("  {}  {}  {}", dim(&format!("{:<8}", "sigil")), dim(&format!("{:<14}", "render")), dim("molecule"));
    for (sig, ren, mol) in ROUTES {
        println!("  {}  {}  {}", cyan(&format!("{sig:<8}")), green(&format!("{ren:<14}")), bold(mol));
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
