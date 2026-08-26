//! stream_demo — visual sanity check for prysm particle rendering.
//!
//! Runs a Bevy window, spawns a StreamScrollback, and feeds every v0 chunk type
//! into the channel from a Startup system. Run with:
//!
//!   cargo run -p prysm --example stream_demo

use bevy::prelude::*;
use prysm::{
    theme, atoms::{glass, GlassDepth},
    layout::scrollback::{StreamPlugin, StreamChannel, StreamScrollback},
    tape::{Chunk, sigil, render, encode_nested, table_chunk},
};
use prysm::tape::bytes::Bytes;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "prysm particle demo".into(),
                resolution: (900u32, 700u32).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(StreamPlugin)
        .add_systems(Startup, (setup, send_demo_chunks).chain())
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            ..default()
        },
        BackgroundColor(theme::DARK_BASE),
    ))
    .with_children(|root| {
        root.spawn((
            Node {
                width: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(theme::G)),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            glass(GlassDepth::Background),
        ))
        .with_children(|bar| {
            bar.spawn((
                Text::new("prysm particles · v0 demo"),
                TextFont { font_size: theme::CAPTION, ..default() },
                TextColor(theme::TEXT_DIM),
            ));
        });

        root.spawn((
            StreamScrollback::default(),
            Node {
                flex_direction: FlexDirection::Column,
                flex_grow: 1.0,
                padding: UiRect::all(Val::Px(theme::G * 2.0)),
                row_gap: Val::Px(2.0),
                overflow: Overflow::clip_y(),
                ..default()
            },
        ));
    });
}

fn send_demo_chunks(channel: Res<StreamChannel>) {
    let tx = &channel.tx;

    tx.send(Chunk::text("Hello from tape. Every chunk type below is a native prysm particle.")).ok();
    tx.send(Chunk::annotation("~/cyber/cyb  ·  main")).ok();
    tx.send(Chunk::new(sigil::PAT, render::TEXT, Bytes::from_static(b"@alice"))).ok();

    tx.send(table_chunk(
        &["name", "size", "modified"],
        vec![
            vec![Chunk::text("Cargo.toml"), Chunk::text("512 B"),   Chunk::text("2m ago")],
            vec![Chunk::text("src/lib.rs"), Chunk::text("18.3 KB"), Chunk::text("4m ago")],
            vec![Chunk::text("Makefile"),   Chunk::text("2.1 KB"),  Chunk::text("1h ago")],
        ],
    )).ok();

    tx.send(Chunk::log("info",  "compiler", "Compiling prysm v0.1.0")).ok();
    tx.send(Chunk::log("warn",  "linker",   "unused variable `x` in src/lib.rs:42")).ok();
    tx.send(Chunk::log("error", "compiler", "mismatched types: expected `u8`, found `u32`")).ok();

    tx.send(Chunk::progress(1, "Building cyb-shell", 72, 100)).ok();
    tx.send(Chunk::progress(1, "Building cyb-shell", 100, 100)).ok();

    tx.send(Chunk::error("cannot borrow `engine` as mutable more than once")).ok();

    let action_payload = encode_nested(&[Chunk::annotation("retry build"), Chunk::text("cmd:cargo_build")]);
    tx.send(Chunk::new(sigil::ZAP, render::COMPONENT, action_payload)).ok();

    let inner = encode_nested(&[
        Chunk::text("composed:"),
        Chunk::annotation("three chunks inside one container"),
        Chunk::new(sigil::PAT, render::TEXT, Bytes::from_static(b"@builder")),
    ]);
    tx.send(Chunk::compose(inner)).ok();

    let breadcrumb = encode_nested(&[
        Chunk::annotation("~/cyber/cyb"),
        Chunk::annotation("shell"),
        Chunk::annotation("worlds"),
    ]);
    tx.send(Chunk::scope(breadcrumb)).ok();

    tx.send(Chunk::status(0)).ok();

    tx.send(Chunk::text("cargo build --release")).ok();
    tx.send(Chunk::log("error", "cargo", "could not compile `cyb-shell`")).ok();
    tx.send(Chunk::status(101)).ok();
}
