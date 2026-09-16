use crate::theme;
use bevy::prelude::*;
use tape::{Chunk, decode_nested};

pub fn spawn(commands: &mut Commands, parent: Entity, chunk: &Chunk) -> Entity {
    let container = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                padding: UiRect::vertical(Val::Px(theme::G * 0.5)),
                ..default()
            },
            ChildOf(parent),
        ))
        .id();

    for child in decode_nested(&chunk.payload) {
        crate::layout::scrollback::dispatch(commands, container, &child);
    }
    container
}

pub fn spawn_scope(commands: &mut Commands, parent: Entity, chunk: &Chunk) -> Entity {
    let container = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                ..default()
            },
            ChildOf(parent),
        ))
        .id();

    for child in decode_nested(&chunk.payload) {
        crate::layout::scrollback::dispatch(commands, container, &child);
    }
    container
}

/// Horizontal pack of child chunks — `row(...)` in rune.
pub fn spawn_row(commands: &mut Commands, parent: Entity, chunk: &Chunk) -> Entity {
    let container = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                width: Val::Percent(100.0),
                column_gap: Val::Px(theme::G),
                padding: UiRect::new(
                    Val::Px(theme::G * 3.0),
                    Val::Px(theme::G * 1.5),
                    Val::Px(theme::G * 3.0),
                    Val::Px(theme::G * 2.0),
                ),
                ..default()
            },
            ChildOf(parent),
        ))
        .id();

    for child in decode_nested(&chunk.payload) {
        crate::layout::scrollback::dispatch(commands, container, &child);
    }
    container
}

/// A `(key, value)` pair on one line — the row of a struct tree.
///
/// The children are laid across, not stacked: a record's field and its value
/// belong on the same line, and stacking them doubles the height of every
/// structured result.
pub fn spawn_pair(commands: &mut Commands, parent: Entity, chunk: &Chunk) -> Entity {
    let container = commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                width: Val::Percent(100.0),
                column_gap: Val::Px(theme::G),
                ..default()
            },
            ChildOf(parent),
        ))
        .id();

    for child in decode_nested(&chunk.payload) {
        crate::layout::scrollback::dispatch(commands, container, &child);
    }
    container
}
