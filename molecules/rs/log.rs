use bevy::prelude::*;
use tade::Chunk;
use crate::theme;

pub fn spawn(commands: &mut Commands, parent: Entity, chunk: &Chunk) -> Entity {
    let m = tade::read_kv(&chunk.payload);
    let level   = kv_str(&m, "level",   "info");
    let source  = kv_str(&m, "source",  "");
    let message = kv_str(&m, "message", "");

    let (prefix, color) = level_style(&level);
    let full = if source.is_empty() {
        format!("{prefix} {message}")
    } else {
        format!("{prefix} [{source}] {message}")
    };
    commands.spawn((
        Text::new(full),
        TextFont { font_size: theme::MICRO, ..default() },
        TextColor(color),
        Node { margin: UiRect::vertical(Val::Px(0.5)), ..default() },
        ChildOf(parent),
    )).id()
}

fn level_style(level: &str) -> (&'static str, Color) {
    match level {
        "error" | "err"     => ("E", theme::ACID_RED),
        "warn"  | "warning" => ("W", theme::ACID_ORANGE),
        "debug" | "dbg"     => ("D", theme::TEXT_DIM),
        "trace"             => ("T", Color::srgba(0.3, 0.3, 0.35, 1.0)),
        _                   => ("I", theme::TEXT_DIM),
    }
}

fn kv_str(m: &std::collections::HashMap<String, Chunk>, key: &str, default: &str) -> String {
    m.get(key)
        .map(|c| String::from_utf8_lossy(&c.payload).into_owned())
        .unwrap_or_else(|| default.to_string())
}
