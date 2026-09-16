use crate::atoms::glass::{GlassDepth, glass};
use crate::molecules::action::ActionButton;
use crate::theme;
use bevy::prelude::*;
use tape::{Chunk, decode_nested, render, sigil};

#[derive(Component)]
pub struct TableRoot;

pub fn spawn(commands: &mut Commands, parent: Entity, chunk: &Chunk) -> Entity {
    let rows = decode_nested(&chunk.payload);
    let mut headers: Vec<String> = Vec::new();
    let mut data_rows: Vec<(Vec<String>, Option<String>)> = Vec::new();

    for row in &rows {
        if row.sigil == sigil::FAS && row.render == render::STRUCT {
            for h in decode_nested(&row.payload) {
                headers.push(String::from_utf8_lossy(&h.payload).into_owned());
            }
        } else if row.sigil == sigil::COL && row.render == render::STRUCT {
            let cells: Vec<String> = decode_nested(&row.payload)
                .iter()
                .map(|c| String::from_utf8_lossy(&c.payload).into_owned())
                .collect();
            let (visible, target) = split_target(cells);
            data_rows.push((visible, target));
        }
    }

    let (fill, hair) = glass(GlassDepth::Midground);
    let root = commands
        .spawn((
            TableRoot,
            Node {
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            fill,
            hair,
            ChildOf(parent),
        ))
        .id();

    if !headers.is_empty() {
        let header_row = commands
            .spawn((
                Node {
                    flex_direction: FlexDirection::Row,
                    width: Val::Percent(100.0),
                    padding: UiRect::axes(Val::Px(theme::G * 1.5), Val::Px(theme::G)),
                    border: UiRect::bottom(Val::Px(1.0)),
                    column_gap: Val::Px(theme::G),
                    ..default()
                },
                BorderColor::all(theme::BORDER),
                ChildOf(root),
            ))
            .id();
        for (i, h) in headers.iter().enumerate() {
            spawn_cell(commands, header_row, h, true, i, headers.len());
        }
    }

    for (i, (cells, target)) in data_rows.iter().enumerate() {
        let mut row = commands.spawn((
            Node {
                flex_direction: FlexDirection::Row,
                width: Val::Percent(100.0),
                padding: UiRect::axes(Val::Px(theme::G * 1.5), Val::Px(theme::G)),
                column_gap: Val::Px(theme::G),
                border: UiRect::bottom(Val::Px(1.0)),
                ..default()
            },
            BorderColor::all(Color::srgba(
                0.13,
                0.92,
                0.51,
                if i % 2 == 0 { 0.06 } else { 0.12 },
            )),
            ChildOf(root),
        ));
        if let Some(t) = target {
            row.insert((
                Button,
                ActionButton {
                    label: cells.first().cloned().unwrap_or_default(),
                    target_ref: t.clone(),
                },
            ));
        }
        let data_row = row.id();
        let n = cells.len().max(headers.len());
        for (j, cell) in cells.iter().enumerate() {
            spawn_cell(commands, data_row, cell, false, j, n);
        }
    }

    root
}

fn split_target(mut cells: Vec<String>) -> (Vec<String>, Option<String>) {
    if cells
        .last()
        .is_some_and(|c| c.starts_with("particle:") || c.starts_with("cyb://"))
    {
        let t = cells.pop();
        (cells, t)
    } else {
        (cells, None)
    }
}

fn spawn_cell(
    commands: &mut Commands,
    row: Entity,
    text: &str,
    is_header: bool,
    col: usize,
    cols: usize,
) {
    let numeric = col > 0;
    let grow = if col == 0 && cols > 1 { 2.2 } else { 1.0 };
    let (size, color) = if is_header {
        (theme::MICRO, theme::TEXT_DIM)
    } else if col == 0 {
        (theme::BODY, theme::TEXT_PRIMARY)
    } else {
        (theme::CAPTION, theme::TEXT_DIM)
    };
    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                flex_grow: grow,
                flex_basis: Val::Px(0.0),
                overflow: Overflow::clip(),
                justify_content: if numeric {
                    JustifyContent::FlexEnd
                } else {
                    JustifyContent::FlexStart
                },
                align_items: AlignItems::Center,
                ..default()
            },
            ChildOf(row),
        ))
        .with_children(|cell| {
            cell.spawn((
                Text::new(text.to_string()),
                TextFont {
                    font_size: size,
                    ..default()
                },
                TextColor(color),
            ));
        });
}
