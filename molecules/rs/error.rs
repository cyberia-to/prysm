use bevy::prelude::*;
use tade::Chunk;
use crate::theme;

pub fn spawn(commands: &mut Commands, parent: Entity, chunk: &Chunk) -> Entity {
    let m = tade::read_kv(&chunk.payload);
    let level   = kv_str(&m, "level",   "error");
    let source  = kv_str(&m, "source",  "");
    let message = kv_str(&m, "message", "unknown error");

    let prefix = match level.as_str() {
        "warn" | "warning" => "⚠ ",
        _ => "✕ ",
    };
    let color = match level.as_str() {
        "warn" | "warning" => theme::ACID_ORANGE,
        _ => theme::ACID_RED,
    };

    let container = commands.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(theme::G)),
            margin: UiRect::vertical(Val::Px(2.0)),
            border: UiRect::left(Val::Px(2.0)),
            ..default()
        },
        BorderColor::all(color),
        BackgroundColor(Color::srgba(color.to_srgba().red, 0.05, 0.05, 0.15)),
        ChildOf(parent),
    )).id();

    commands.spawn((
        Text::new(format!("{prefix}{message}")),
        TextFont { font_size: theme::BODY, ..default() },
        TextColor(color),
        ChildOf(container),
    ));

    if !source.is_empty() {
        commands.spawn((
            Text::new(source),
            TextFont { font_size: theme::MICRO, ..default() },
            TextColor(theme::TEXT_DIM),
            ChildOf(container),
        ));
    }

    container
}

fn kv_str(m: &std::collections::HashMap<String, Chunk>, key: &str, default: &str) -> String {
    m.get(key)
        .map(|c| String::from_utf8_lossy(&c.payload).into_owned())
        .unwrap_or_else(|| default.to_string())
}
