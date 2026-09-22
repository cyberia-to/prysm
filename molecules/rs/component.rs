use bevy::prelude::*;
use tade::{Chunk, decode_nested};
use crate::theme;

pub fn spawn(commands: &mut Commands, parent: Entity, chunk: &Chunk) -> Entity {
    let container = commands.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.0),
            padding: UiRect::vertical(Val::Px(theme::G * 0.5)),
            ..default()
        },
        ChildOf(parent),
    )).id();

    for child in decode_nested(&chunk.payload) {
        crate::layout::scrollback::dispatch(commands, container, &child);
    }
    container
}

pub fn spawn_scope(commands: &mut Commands, parent: Entity, chunk: &Chunk) -> Entity {
    let container = commands.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.0),
            ..default()
        },
        ChildOf(parent),
    )).id();

    for child in decode_nested(&chunk.payload) {
        crate::layout::scrollback::dispatch(commands, container, &child);
    }
    container
}
