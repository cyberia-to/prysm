use crate::atoms::glass::{GlassDepth, glass};
use crate::molecules::action::ActionButton;
use crate::theme;
use bevy::prelude::*;
use tade::{Chunk, decode_nested, render, sigil};

#[derive(Component)]
pub struct TableRoot;

pub fn spawn(commands: &mut Commands, parent: Entity, chunk: &Chunk) -> Entity {
    let rows = decode_nested(&chunk.payload);
    let mut headers: Vec<String> = Vec::new();
    let mut data_rows: Vec<(Vec<String>, Option<String>)> = Vec::new();

    for row in &rows {
        if row.sigil == sigil::FAS && row.render == render::STRUCT {
            if headers.is_empty() {
                for h in decode_nested(&row.payload) {
                    headers.push(String::from_utf8_lossy(&h.payload).into_owned());
                }
            } else {
                // A second header is a body row that leaked. Treat as data.
                let cells: Vec<String> = decode_nested(&row.payload)
                    .iter()
                    .map(|c| String::from_utf8_lossy(&c.payload).into_owned())
                    .collect();
                let (visible, target) = split_target(cells);
                data_rows.push((visible, target));
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

    let cols = headers
        .len()
        .max(data_rows.iter().map(|(c, _)| c.len()).max().unwrap_or(0))
        .max(1);
    let tracks = column_tracks(cols, &headers);

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
        let header_row = spawn_lane(commands, root, true, 0);
        for i in 0..cols {
            spawn_cell(
                commands,
                header_row,
                headers.get(i).map(String::as_str).unwrap_or(""),
                true,
                i,
                tracks[i],
            );
        }
    }

    for (i, (cells, target)) in data_rows.iter().enumerate() {
        let data_row = spawn_lane(commands, root, false, i);
        if let Some(t) = target {
            commands.entity(data_row).insert((
                Button,
                ActionButton {
                    label: cells.first().cloned().unwrap_or_default(),
                    target_ref: t.clone(),
                },
            ));
        }
        for j in 0..cols {
            spawn_cell(
                commands,
                data_row,
                cells.get(j).map(String::as_str).unwrap_or(""),
                false,
                j,
                tracks[j],
            );
        }
    }

    root
}

/// One width per column, same on every lane. First column takes the leftover;
/// the rest are equal tracks. Percents sum to 100.
fn column_tracks(n: usize, headers: &[String]) -> Vec<f32> {
    match n {
        0 | 1 => vec![100.0],
        2 => vec![62.0, 38.0],
        3 => vec![50.0, 25.0, 25.0],
        4 if headers.first().is_some_and(|h| h == "from") => vec![38.0, 38.0, 12.0, 12.0],
        4 => vec![54.0, 14.0, 18.0, 14.0],
        n => {
            let rest = 56.0 / (n as f32 - 1.0);
            let mut t = vec![44.0];
            t.extend(std::iter::repeat(rest).take(n - 1));
            t
        }
    }
}

fn spawn_lane(commands: &mut Commands, parent: Entity, header: bool, i: usize) -> Entity {
    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                width: Val::Percent(100.0),
                min_width: Val::Percent(100.0),
                padding: UiRect::axes(Val::Px(theme::G * 1.5), Val::Px(theme::G)),
                border: UiRect::bottom(Val::Px(1.0)),
                overflow: Overflow::clip(),
                ..default()
            },
            BorderColor::all(if header {
                theme::BORDER
            } else {
                Color::srgba(0.13, 0.92, 0.51, if i % 2 == 0 { 0.06 } else { 0.14 })
            }),
            ChildOf(parent),
        ))
        .id()
}

fn split_target(mut cells: Vec<String>) -> (Vec<String>, Option<String>) {
    if cells.last().is_some_and(|c| {
        c.starts_with("particle:")
            || c.starts_with("cyb://")
            || c.starts_with("model:")
            || c.starts_with("fetch:")
            || c.starts_with("vault:")
            || c.starts_with("work:")
    }) {
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
    width_pct: f32,
) {
    let numeric = col > 0;
    let (size, color) = if is_header {
        (theme::MICRO, theme::TEXT_DIM)
    } else if col == 0 {
        (theme::BODY, theme::TEXT_PRIMARY)
    } else {
        (theme::CAPTION, theme::TEXT_DIM)
    };
    let fade_w = theme::G * 3.0;
    let ink = Color::srgba(0.0, 0.0, 0.0, 0.0);
    let veil = theme::DARK_BASE;
    let fade = if numeric {
        LinearGradient::to_right(vec![
            ColorStop::percent(veil, 0.0),
            ColorStop::percent(ink, 100.0),
        ])
    } else {
        LinearGradient::to_right(vec![
            ColorStop::percent(ink, 0.0),
            ColorStop::percent(veil, 100.0),
        ])
    };
    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                width: Val::Percent(width_pct),
                min_width: Val::Px(0.0),
                max_width: Val::Percent(width_pct),
                flex_grow: 0.0,
                flex_shrink: 0.0,
                overflow: Overflow::clip(),
                justify_content: if numeric {
                    JustifyContent::FlexEnd
                } else {
                    JustifyContent::FlexStart
                },
                align_items: AlignItems::Center,
                padding: UiRect::horizontal(Val::Px(theme::G * 0.5)),
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
                TextLayout::new_with_no_wrap(),
            ));
            cell.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    top: Val::Px(0.0),
                    bottom: Val::Px(0.0),
                    width: Val::Px(fade_w),
                    left: if numeric { Val::Px(0.0) } else { Val::Auto },
                    right: if numeric { Val::Auto } else { Val::Px(0.0) },
                    ..default()
                },
                BackgroundGradient::from(fade),
                ZIndex(1),
                Pickable::IGNORE,
            ));
        });
}
