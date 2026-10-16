use bevy::prelude::*;
use tade::Chunk;
use crate::theme;

#[derive(Component)]
pub struct ProgressBar;

#[derive(Component)]
pub struct ProgressFill;

#[derive(Component)]
pub struct ProgressLabel;

pub fn spawn(commands: &mut Commands, parent: Entity, chunk: &Chunk) -> Entity {
    let m = tade::read_kv(&chunk.payload);
    let label   = kv_str(&m, "label", "");
    let current = kv_u64(&m, "current");
    let total   = kv_u64(&m, "total");
    let fraction = if total == 0 { 0.0 } else { current as f32 / total as f32 };

    let container = commands.spawn((
        ProgressBar,
        Node {
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.0),
            padding: UiRect::vertical(Val::Px(2.0)),
            ..default()
        },
        ChildOf(parent),
    )).id();

    commands.spawn((
        ProgressLabel,
        Text::new(label),
        TextFont { font_size: theme::MICRO, ..default() },
        TextColor(theme::TEXT_DIM),
        ChildOf(container),
    ));

    let track = commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(4.0),
            margin: UiRect::top(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.08)),
        ChildOf(container),
    )).id();

    commands.spawn((
        ProgressFill,
        Node {
            width: Val::Percent((fraction * 100.0).clamp(0.0, 100.0)),
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(theme::ACID_BLUE),
        ChildOf(track),
    ));

    container
}

fn kv_str(m: &std::collections::HashMap<String, Chunk>, key: &str, default: &str) -> String {
    m.get(key)
        .map(|c| String::from_utf8_lossy(&c.payload).into_owned())
        .unwrap_or_else(|| default.to_string())
}

fn kv_u64(m: &std::collections::HashMap<String, Chunk>, key: &str) -> u64 {
    m.get(key)
        .and_then(|c| String::from_utf8_lossy(&c.payload).parse().ok())
        .unwrap_or(0)
}
