use bevy::prelude::*;
use tade::Chunk;
use crate::theme;

pub fn spawn(commands: &mut Commands, parent: Entity, chunk: &Chunk) -> Entity {
    let content = String::from_utf8_lossy(&chunk.payload).into_owned();
    commands.spawn((
        Text::new(content),
        TextFont { font_size: theme::BODY, ..default() },
        TextColor(theme::TEXT_PRIMARY),
        Node { margin: UiRect::vertical(Val::Px(1.0)), ..default() },
        ChildOf(parent),
    )).id()
}

pub fn spawn_annotation(commands: &mut Commands, parent: Entity, chunk: &Chunk) -> Entity {
    let content = String::from_utf8_lossy(&chunk.payload).into_owned();
    commands.spawn((
        Text::new(content),
        TextFont { font_size: theme::CAPTION, ..default() },
        TextColor(theme::TEXT_DIM),
        Node { margin: UiRect::vertical(Val::Px(1.0)), ..default() },
        ChildOf(parent),
    )).id()
}
