use bevy::prelude::*;
use tade::{Chunk, sigil, render, decode_nested};
use crate::theme;

#[derive(Component)]
pub struct TableRoot;

pub fn spawn(commands: &mut Commands, parent: Entity, chunk: &Chunk) -> Entity {
    let rows = decode_nested(&chunk.payload);
    let mut headers: Vec<String>       = Vec::new();
    let mut data_rows: Vec<Vec<String>> = Vec::new();

    for row in &rows {
        if row.sigil == sigil::FAS && row.render == render::STRUCT {
            for h in decode_nested(&row.payload) {
                headers.push(String::from_utf8_lossy(&h.payload).into_owned());
            }
        } else if row.sigil == sigil::COL && row.render == render::STRUCT {
            let cells = decode_nested(&row.payload);
            data_rows.push(cells.iter()
                .map(|c| String::from_utf8_lossy(&c.payload).into_owned())
                .collect());
        }
    }

    let root = commands.spawn((
        TableRoot,
        Node {
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.0),
            margin: UiRect::vertical(Val::Px(4.0)),
            ..default()
        },
        ChildOf(parent),
    )).id();

    if !headers.is_empty() {
        let header_row = commands.spawn((
            Node {
                flex_direction: FlexDirection::Row,
                width: Val::Percent(100.0),
                padding: UiRect::axes(Val::Px(4.0), Val::Px(3.0)),
                border: UiRect::bottom(Val::Px(1.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.06)),
            BorderColor::all(Color::srgba(1.0, 1.0, 1.0, 0.12)),
            ChildOf(root),
        )).id();

        for h in &headers {
            spawn_cell(commands, header_row, h, true);
        }
    }

    for (i, row) in data_rows.iter().enumerate() {
        let bg = if i % 2 == 0 { Color::NONE } else { Color::srgba(1.0, 1.0, 1.0, 0.03) };
        let data_row = commands.spawn((
            Node {
                flex_direction: FlexDirection::Row,
                width: Val::Percent(100.0),
                padding: UiRect::axes(Val::Px(4.0), Val::Px(2.0)),
                ..default()
            },
            BackgroundColor(bg),
            ChildOf(root),
        )).id();

        for cell in row {
            spawn_cell(commands, data_row, cell, false);
        }
    }

    root
}

fn spawn_cell(commands: &mut Commands, row: Entity, text: &str, is_header: bool) {
    let (size, color) = if is_header {
        (theme::CAPTION, Color::srgba(0.75, 0.75, 0.80, 1.0))
    } else {
        (theme::BODY, theme::TEXT_PRIMARY)
    };
    commands.spawn((
        Text::new(text.to_string()),
        TextFont { font_size: size, ..default() },
        TextColor(color),
        Node {
            flex_grow: 1.0,
            overflow: Overflow::clip(),
            ..default()
        },
        ChildOf(row),
    ));
}
