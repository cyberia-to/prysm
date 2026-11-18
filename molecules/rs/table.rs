use crate::atoms::glass::{GlassDepth, glass};
use crate::molecules::action::ActionButton;
use crate::theme;
use bevy::prelude::*;
use tade::{Chunk, decode_nested, render, sigil};

#[derive(Component)]
pub struct TableRoot;

/// A row as it leaves `split_targets`: the visible cells, then its targets.
type Row = (Vec<String>, Vec<String>);

pub fn spawn(commands: &mut Commands, parent: Entity, chunk: &Chunk) -> Entity {
    let rows = decode_nested(&chunk.payload);
    let mut headers: Vec<String> = Vec::new();
    let mut data_rows: Vec<Row> = Vec::new();

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
                let (visible, targets) = split_targets(cells);
                data_rows.push((visible, targets));
            }
        } else if row.sigil == sigil::COL && row.render == render::STRUCT {
            let cells: Vec<String> = decode_nested(&row.payload)
                .iter()
                .map(|c| String::from_utf8_lossy(&c.payload).into_owned())
                .collect();
            let (visible, targets) = split_targets(cells);
            data_rows.push((visible, targets));
        }
    }

    let cols = headers
        .len()
        .max(data_rows.iter().map(|(c, _)| c.len()).max().unwrap_or(0))
        .max(1);
    let tracks = column_tracks(cols, &headers, &data_rows);
    // Alignment belongs to a column's values, not to its position in the row.
    let align_end: Vec<bool> = (0..cols).map(|c| numeric_column(c, &data_rows)).collect();

    let (fill, hair) = glass(GlassDepth::Midground);
    let root = commands
        .spawn((
            TableRoot,
            Node {
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                flex_shrink: 0.0,
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
                align_end[i],
                tracks[i],
                None,
            );
        }
    }

    for (i, (cells, targets)) in data_rows.iter().enumerate() {
        let data_row = spawn_lane(commands, root, false, i);
        if let Some(target) = row_target(targets) {
            commands.entity(data_row).insert((
                Button,
                ActionButton {
                    label: cells.first().cloned().unwrap_or_default(),
                    target_ref: target.to_string(),
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
                align_end[j],
                tracks[j],
                if targets.len() > 1 {
                    targets.get(j).cloned().filter(|t| t != "_")
                } else {
                    None
                },
            );
        }
    }

    root
}

/// One width per column, same on every lane. A column's share follows the
/// widest value standing in it — header included — so the tracks come from the
/// frame and no caller's column names are written into the renderer. Every
/// column keeps a floor of half an even share, and the shares sum to 100.
fn column_tracks(n: usize, headers: &[String], rows: &[Row]) -> Vec<f32> {
    if n <= 1 {
        return vec![100.0];
    }
    let widest: Vec<f32> = (0..n)
        .map(|col| {
            let header = headers.get(col).map_or(0, |h| h.chars().count());
            rows.iter()
                .filter_map(|(cells, _)| cells.get(col))
                .map(|c| c.chars().count())
                .fold(header, usize::max) as f32
        })
        .collect();
    let total: f32 = widest.iter().sum();
    let floor = 50.0 / n as f32;
    let mut tracks: Vec<f32> = if total > 0.0 {
        widest.iter().map(|w| floor + 50.0 * w / total).collect()
    } else {
        vec![100.0 / n as f32; n]
    };
    // The lane is filled exactly: the last track absorbs the rounding drift.
    let drift = 100.0 - tracks.iter().sum::<f32>();
    *tracks.last_mut().unwrap() += drift;
    tracks
}

/// A column is numeric — right-aligned, per `molecules/specs/table.md` — when it
/// holds at least one value and every value in it reads as a number. Read from
/// the column's own cells, never from the column's index.
fn numeric_column(col: usize, rows: &[Row]) -> bool {
    let mut seen = false;
    for (cells, _) in rows {
        let Some(cell) = cells.get(col) else { continue };
        if cell.trim().is_empty() {
            continue;
        }
        if !reads_as_number(cell) {
            return false;
        }
        seen = true;
    }
    seen
}

/// A number as a table carries one: digits with an optional sign, group
/// separators, a decimal point, and a trailing percent.
fn reads_as_number(cell: &str) -> bool {
    let bare = cell.trim().trim_end_matches('%').replace([',', '_'], "");
    !bare.is_empty() && bare.parse::<f64>().is_ok()
}

fn spawn_lane(commands: &mut Commands, parent: Entity, header: bool, i: usize) -> Entity {
    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Row,
                width: Val::Percent(100.0),
                min_width: Val::Percent(100.0),
                flex_shrink: 0.0,
                padding: UiRect::axes(Val::Px(theme::G * 1.5), Val::Px(theme::G)),
                border: UiRect::bottom(Val::Px(1.0)),
                overflow: Overflow::clip(),
                ..default()
            },
            BorderColor::all(if header {
                theme::BORDER
            } else {
                theme::acid(if i % 2 == 0 { 0.06 } else { 0.14 })
            }),
            ChildOf(parent),
        ))
        .id()
}

fn is_target(c: &str) -> bool {
    c == "_"
        || c.starts_with("particle:")
        || c.starts_with("cyb://")
        || c.starts_with("model:")
        || c.starts_with("fetch:")
        || c.starts_with("vault:")
        || c.starts_with("work:")
        || c.starts_with("block:")
}

/// The target of a whole row: the one trailing target cell, when there is
/// exactly one and it is not the `_` that marks a row with no target at all.
/// A lone `_` installs nothing, so the row stays unclickable.
fn row_target(targets: &[String]) -> Option<&str> {
    match targets {
        [one] if one != "_" => Some(one.as_str()),
        _ => None,
    }
}

fn split_targets(mut cells: Vec<String>) -> (Vec<String>, Vec<String>) {
    let mut targets = Vec::new();
    while cells.last().is_some_and(|c| is_target(c)) {
        targets.push(cells.pop().unwrap());
    }
    targets.reverse();
    (cells, targets)
}

fn spawn_cell(
    commands: &mut Commands,
    row: Entity,
    text: &str,
    is_header: bool,
    col: usize,
    align_end: bool,
    width_pct: f32,
    target: Option<String>,
) {
    // Alignment is the column's, read off its own values. The emphasis tier is
    // the lane's first column — the primary column the fold in
    // `molecules/specs/table.md` keeps last.
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
    let fade = if align_end {
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
    let mut e = commands.spawn((
        Node {
            flex_direction: FlexDirection::Row,
            width: Val::Percent(width_pct),
            min_width: Val::Px(0.0),
            max_width: Val::Percent(width_pct),
            flex_grow: 0.0,
            flex_shrink: 0.0,
            overflow: Overflow::clip(),
            justify_content: if align_end {
                JustifyContent::FlexEnd
            } else {
                JustifyContent::FlexStart
            },
            align_items: AlignItems::Center,
            padding: UiRect::horizontal(Val::Px(theme::G * 0.5)),
            ..default()
        },
        ChildOf(row),
    ));
    if let Some(t) = target {
        e.insert((
            Button,
            ActionButton {
                label: text.to_string(),
                target_ref: t,
            },
        ));
    }
    e.with_children(|cell| {
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
                left: if align_end { Val::Px(0.0) } else { Val::Auto },
                right: if align_end { Val::Auto } else { Val::Px(0.0) },
                ..default()
            },
            BackgroundGradient::from(fade),
            ZIndex(1),
            Pickable::IGNORE,
        ));
    });
}

#[cfg(test)]
mod tests {
    use super::{Row, column_tracks, numeric_column, row_target, split_targets};

    fn headers(hs: &[&str]) -> Vec<String> {
        hs.iter().map(|h| h.to_string()).collect()
    }

    fn rows(rs: &[&[&str]]) -> Vec<Row> {
        rs.iter().map(|r| (headers(r), Vec::new())).collect()
    }

    fn sums_to_a_hundred(tracks: &[f32]) -> bool {
        (tracks.iter().sum::<f32>() - 100.0).abs() < 0.05
    }

    #[test]
    fn one_track_per_column_and_the_tracks_fill_the_lane() {
        for n in 1..=12usize {
            let tracks = column_tracks(n, &[], &[]);
            assert_eq!(tracks.len(), n, "{n} columns need {n} tracks");
            assert!(sums_to_a_hundred(&tracks), "{n} columns: {tracks:?}");
        }
    }

    #[test]
    fn a_table_with_no_columns_still_has_one_track() {
        let tracks = column_tracks(0, &[], &[]);
        assert_eq!(tracks.len(), 1);
        assert!(sums_to_a_hundred(&tracks));
    }

    #[test]
    fn a_wide_column_takes_more_of_the_lane_than_a_narrow_one() {
        let data = rows(&[
            &["bostrom1qqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqq", "7"],
            &["bostrom1wwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwwww", "12"],
        ]);
        let tracks = column_tracks(2, &headers(&["from", "n"]), &data);
        assert!(tracks[0] > tracks[1], "the long column is wider: {tracks:?}");
        assert!(tracks[1] >= 25.0, "every column keeps its floor: {tracks:?}");
        assert!(sums_to_a_hundred(&tracks));
    }

    #[test]
    fn the_header_words_do_not_decide_the_tracks() {
        let data = rows(&[&["bostrom1qqqq", "bostrom1wwww", "42", "2m"]]);
        let named = column_tracks(4, &headers(&["from", "to", "amount", "when"]), &data);
        let other = column_tracks(4, &headers(&["left", "up", "number", "time"]), &data);
        assert_eq!(
            named, other,
            "the tracks follow the content, not a caller's column names"
        );
        assert!(sums_to_a_hundred(&named));
    }

    #[test]
    fn alignment_follows_the_column_content_not_its_position() {
        let data = rows(&[&["Cargo.toml", "512 B", "1024"], &["rs", "4 kB", "2048"]]);
        assert!(!numeric_column(0, &data), "a name column reads as text");
        assert!(!numeric_column(1, &data), "a size with a unit reads as text");
        assert!(numeric_column(2, &data), "a column of digits reads as numeric");
    }

    #[test]
    fn a_column_of_nothing_is_not_numeric() {
        let data = rows(&[&["a", ""], &["b", "  "]]);
        assert!(!numeric_column(1, &data), "an empty column takes text alignment");
        assert!(!numeric_column(9, &data), "a column outside every row is text");
    }

    #[test]
    fn a_signed_or_grouped_number_still_reads_as_one() {
        let data = rows(&[&["-1.5"], &["1,024"], &["+12%"]]);
        assert!(numeric_column(0, &data));
    }

    #[test]
    fn a_row_whose_last_cell_is_data_keeps_every_cell() {
        let row = headers(&["Cargo.toml", "512 B", "2m ago"]);
        let (visible, targets) = split_targets(row.clone());
        assert_eq!(visible, row, "a data row must not lose its last cell");
        assert!(targets.is_empty(), "a data row declares no target");
    }

    #[test]
    fn a_trailing_target_leaves_the_row_and_becomes_its_target() {
        let (visible, targets) = split_targets(headers(&["zheng", "idle", "work:zheng"]));
        assert_eq!(visible, headers(&["zheng", "idle"]));
        assert_eq!(targets, headers(&["work:zheng"]));
        assert_eq!(row_target(&targets), Some("work:zheng"));
    }

    #[test]
    fn a_lone_underscore_target_leaves_the_row_unclickable() {
        let (visible, targets) = split_targets(headers(&["zheng", "idle", "_"]));
        assert_eq!(visible, headers(&["zheng", "idle"]));
        assert_eq!(targets, headers(&["_"]));
        assert_eq!(
            row_target(&targets),
            None,
            "`_` marks a row with no target: it must not become the target"
        );
    }

    #[test]
    fn a_row_has_no_single_target_when_its_targets_are_per_column() {
        let (_, targets) = split_targets(headers(&["a", "b", "particle:aa", "_"]));
        assert_eq!(row_target(&targets), None, "per-column targets, not a row target");
        assert_eq!(row_target(&[]), None, "no target cell, no row target");
    }

    #[test]
    fn one_target_per_column_keeps_its_column_order() {
        let (visible, targets) =
            split_targets(headers(&["a", "b", "c", "particle:aa", "_", "cyb://brain"]));
        assert_eq!(visible, headers(&["a", "b", "c"]));
        assert_eq!(targets, headers(&["particle:aa", "_", "cyb://brain"]));
    }
}
