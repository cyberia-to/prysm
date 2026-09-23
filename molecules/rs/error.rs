use bevy::prelude::*;
use tade::Chunk;
use crate::theme;

pub fn spawn(commands: &mut Commands, parent: Entity, chunk: &Chunk) -> Entity {
    let m = tade::read_kv(&chunk.payload);
    let level   = kv_str(&m, "level",   "error");
    let source  = kv_str(&m, "source",  "");
    let message = kv_str(&m, "message", "unknown error");

    let (prefix, color) = error_style(&level);

    let container = commands.spawn((
        Node {
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(theme::G)),
            margin: UiRect::vertical(Val::Px(2.0)),
            border: UiRect::left(Val::Px(2.0)),
            ..default()
        },
        BorderColor::all(color),
        BackgroundColor(Color::srgba(color.to_srgba().red, 0.05, 0.05, 0.15)),
        ChildOf(parent),
    )).id();

    commands.spawn((
        Text::new(format!("{prefix}{message}")),
        TextFont { font_size: theme::BODY, ..default() },
        TextColor(color),
        ChildOf(container),
    ));

    if !source.is_empty() {
        commands.spawn((
            Text::new(source),
            TextFont { font_size: theme::MICRO, ..default() },
            TextColor(theme::TEXT_DIM),
            ChildOf(container),
        ));
    }

    container
}

/// Prefix and color for a reported error's `level` field.
fn error_style(level: &str) -> (&'static str, Color) {
    match level {
        "warn" | "warning" => ("⚠ ", theme::ACID_ORANGE),
        _ => ("✕ ", theme::ACID_RED),
    }
}

fn kv_str(m: &std::collections::HashMap<String, Chunk>, key: &str, default: &str) -> String {
    m.get(key)
        .map(|c| String::from_utf8_lossy(&c.payload).into_owned())
        .unwrap_or_else(|| default.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tade::Chunk;

    fn chunk(payload: &[u8]) -> Chunk {
        Chunk::new(tade::sigil::HAX, tade::render::TEXT, tade::bytes::Bytes::copy_from_slice(payload))
    }

    #[test]
    fn error_style_defaults_to_error() {
        let (prefix, color) = error_style("error");
        assert_eq!(prefix, "✕ ");
        assert_eq!(color, theme::ACID_RED);
    }

    #[test]
    fn error_style_unknown_level_falls_back_to_error() {
        let (prefix, color) = error_style("fatal");
        assert_eq!(prefix, "✕ ");
        assert_eq!(color, theme::ACID_RED);
    }

    #[test]
    fn error_style_warn_and_warning_both_match() {
        assert_eq!(error_style("warn"), ("⚠ ", theme::ACID_ORANGE));
        assert_eq!(error_style("warning"), ("⚠ ", theme::ACID_ORANGE));
    }

    #[test]
    fn kv_str_reads_present_key_as_utf8() {
        let mut m = std::collections::HashMap::new();
        m.insert("message".to_string(), chunk(b"boom"));
        assert_eq!(kv_str(&m, "message", "fallback"), "boom");
    }

    #[test]
    fn kv_str_falls_back_on_missing_key() {
        let m = std::collections::HashMap::new();
        assert_eq!(kv_str(&m, "source", "fallback"), "fallback");
    }

    #[test]
    fn kv_str_lossy_decodes_non_utf8_payload() {
        let mut m = std::collections::HashMap::new();
        m.insert("message".to_string(), chunk(&[0xff, 0xfe]));
        // Never panics on invalid UTF-8 — decodes lossily instead.
        let _ = kv_str(&m, "message", "fallback");
    }
}
