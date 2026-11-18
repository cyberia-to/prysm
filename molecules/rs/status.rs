use bevy::prelude::*;
use tade::Chunk;
use crate::theme;

pub fn spawn(commands: &mut Commands, parent: Entity, chunk: &Chunk) -> Entity {
    let m = tade::read_kv(&chunk.payload);
    let code: i32 = m.get("code")
        .and_then(|c| String::from_utf8_lossy(&c.payload).parse().ok())
        .unwrap_or(0);

    let color = if code == 0 {
        Color::srgba(1.0, 1.0, 1.0, 0.06)
    } else {
        Color::srgba(theme::ACID_RED.to_srgba().red, 0.1, 0.1, 0.3)
    };
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(1.0),
            margin: UiRect::vertical(Val::Px(4.0)),
            ..default()
        },
        BackgroundColor(color),
        ChildOf(parent),
    )).id()
}
