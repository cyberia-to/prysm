use bevy::prelude::*;
use tade::{Chunk, sigil, render, decode_nested};
use crate::theme;

#[derive(Component)]
pub struct TableRoot;

/// Extract header labels and data-row cell text from a table chunk's nested
/// payload: a `(/, s)` schema row of headers followed by zero or more `(:, s)`
/// data rows of cells (the shape `tade::table_chunk` builds). Any other sigil
/// or render at either level is skipped rather than rejected, so a stream
/// producer can interleave unrelated chunks without corrupting the table.
pub fn parse_table(rows: &[Chunk]) -> (Vec<String>, Vec<Vec<String>>) {
    let mut headers: Vec<String>       = Vec::new();
    let mut data_rows: Vec<Vec<String>> = Vec::new();

    for row in rows {
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

    (headers, data_rows)
}

pub fn spawn(commands: &mut Commands, parent: Entity, chunk: &Chunk) -> Entity {
    let rows = decode_nested(&chunk.payload);
    let (headers, data_rows) = parse_table(&rows);

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

#[cfg(test)]
mod tests {
    use super::*;
    use tade::table_chunk;

    fn rows_of(chunk: &Chunk) -> Vec<Chunk> {
        decode_nested(&chunk.payload)
    }

    #[test]
    fn empty_table_yields_no_headers_no_rows() {
        let chunk = table_chunk(&[], vec![]);
        let (headers, data) = parse_table(&rows_of(&chunk));
        assert!(headers.is_empty());
        assert!(data.is_empty());
    }

    #[test]
    fn headers_only() {
        let chunk = table_chunk(&["a", "b", "c"], vec![]);
        let (headers, data) = parse_table(&rows_of(&chunk));
        assert_eq!(headers, vec!["a", "b", "c"]);
        assert!(data.is_empty());
    }

    #[test]
    fn headers_and_data_rows() {
        let chunk = table_chunk(
            &["name", "count"],
            vec![
                vec![Chunk::text("alpha"), Chunk::text("1")],
                vec![Chunk::text("beta"), Chunk::text("2")],
            ],
        );
        let (headers, data) = parse_table(&rows_of(&chunk));
        assert_eq!(headers, vec!["name", "count"]);
        assert_eq!(data, vec![
            vec!["alpha".to_string(), "1".to_string()],
            vec!["beta".to_string(), "2".to_string()],
        ]);
    }

    #[test]
    fn data_only_no_schema_row() {
        // table_chunk always emits a schema row, so build the nested payload
        // by hand to cover a producer that sends only data rows.
        let data_row = Chunk::new(sigil::COL, render::STRUCT, tade::encode_nested(&[
            Chunk::text("x"), Chunk::text("y"),
        ]));
        let (headers, data) = parse_table(&[data_row]);
        assert!(headers.is_empty());
        assert_eq!(data, vec![vec!["x".to_string(), "y".to_string()]]);
    }

    #[test]
    fn ragged_rows_preserve_their_own_cell_count() {
        let chunk = table_chunk(
            &["a", "b"],
            vec![
                vec![Chunk::text("only-one")],
                vec![Chunk::text("x"), Chunk::text("y"), Chunk::text("z")],
            ],
        );
        let (_, data) = parse_table(&rows_of(&chunk));
        assert_eq!(data[0], vec!["only-one".to_string()]);
        assert_eq!(data[1], vec!["x".to_string(), "y".to_string(), "z".to_string()]);
    }

    #[test]
    fn non_ascii_cell_text_round_trips() {
        let chunk = table_chunk(&["名前"], vec![vec![Chunk::text("日本語")]]);
        let (headers, data) = parse_table(&rows_of(&chunk));
        assert_eq!(headers, vec!["名前"]);
        assert_eq!(data, vec![vec!["日本語".to_string()]]);
    }

    #[test]
    fn unrelated_sigil_or_render_at_row_level_is_skipped() {
        // Neither a schema nor a data row by this function's contract: ignored,
        // not an error, so a stream can interleave unrelated chunks.
        let stray = Chunk::annotation("not a table row");
        let chunk = table_chunk(&["a"], vec![vec![Chunk::text("1")]]);
        let mut rows = rows_of(&chunk);
        rows.push(stray);
        let (headers, data) = parse_table(&rows);
        assert_eq!(headers, vec!["a"]);
        assert_eq!(data, vec![vec!["1".to_string()]]);
    }
}
