use bevy::prelude::*;
use tade::Chunk;
use crate::theme;

pub fn spawn(commands: &mut Commands, parent: Entity, chunk: &Chunk) -> Entity {
    let raw = String::from_utf8_lossy(&chunk.payload).into_owned();
    let name = if raw.starts_with('@') { raw } else { format!("@{raw}") };
    commands.spawn((
        Text::new(name),
        TextFont { font_size: theme::BODY, ..default() },
        TextColor(theme::ACID_BLUE),
        Node { margin: UiRect::vertical(Val::Px(1.0)), ..default() },
        ChildOf(parent),
    )).id()
}
