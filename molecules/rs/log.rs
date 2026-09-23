use bevy::prelude::*;
use tade::Chunk;
use crate::theme;

pub fn spawn(commands: &mut Commands, parent: Entity, chunk: &Chunk) -> Entity {
    let m = tade::read_kv(&chunk.payload);
    let level   = kv_str(&m, "level",   "info");
    let source  = kv_str(&m, "source",  "");
    let message = kv_str(&m, "message", "");

    let (prefix, color) = level_style(&level);
    let full = format_line(prefix, &source, &message);
    commands.spawn((
        Text::new(full),
        TextFont { font_size: theme::MICRO, ..default() },
        TextColor(color),
        Node { margin: UiRect::vertical(Val::Px(0.5)), ..default() },
        ChildOf(parent),
    )).id()
}

/// `"{prefix} {message}"`, or `"{prefix} [{source}] {message}"` when a
/// source is given.
fn format_line(prefix: &str, source: &str, message: &str) -> String {
    if source.is_empty() {
        format!("{prefix} {message}")
    } else {
        format!("{prefix} [{source}] {message}")
    }
}

fn level_style(level: &str) -> (&'static str, Color) {
    match level {
        "error" | "err"     => ("E", theme::ACID_RED),
        "warn"  | "warning" => ("W", theme::ACID_ORANGE),
        "debug" | "dbg"     => ("D", theme::TEXT_DIM),
        "trace"             => ("T", Color::srgba(0.3, 0.3, 0.35, 1.0)),
        _                   => ("I", theme::TEXT_DIM),
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

    #[test]
    fn format_line_without_source() {
        assert_eq!(format_line("I", "", "hello"), "I hello");
    }

    #[test]
    fn format_line_with_source() {
        assert_eq!(format_line("E", "worker-3", "boom"), "E [worker-3] boom");
    }

    #[test]
    fn level_style_recognizes_every_synonym() {
        for level in ["error", "err"] {
            assert_eq!(level_style(level), ("E", theme::ACID_RED));
        }
        for level in ["warn", "warning"] {
            assert_eq!(level_style(level), ("W", theme::ACID_ORANGE));
        }
        for level in ["debug", "dbg"] {
            assert_eq!(level_style(level), ("D", theme::TEXT_DIM));
        }
        assert_eq!(level_style("trace"), ("T", Color::srgba(0.3, 0.3, 0.35, 1.0)));
    }

    #[test]
    fn level_style_defaults_to_info() {
        assert_eq!(level_style("info"), ("I", theme::TEXT_DIM));
        assert_eq!(level_style(""), ("I", theme::TEXT_DIM));
        assert_eq!(level_style("nonsense"), ("I", theme::TEXT_DIM));
    }

    #[test]
    fn level_style_is_case_sensitive() {
        // "Error" does not match "error"/"err" — falls through to the default.
        assert_eq!(level_style("Error"), ("I", theme::TEXT_DIM));
    }

    #[test]
    fn kv_str_reads_present_key() {
        let mut m = std::collections::HashMap::new();
        m.insert("level".to_string(), Chunk::text("warn"));
        assert_eq!(kv_str(&m, "level", "info"), "warn");
    }

    #[test]
    fn kv_str_falls_back_to_default() {
        let m = std::collections::HashMap::new();
        assert_eq!(kv_str(&m, "level", "info"), "info");
    }

    #[test]
    fn kv_str_handles_non_utf8_payload() {
        let mut m = std::collections::HashMap::new();
        m.insert("message".to_string(), Chunk::new(b'#', b't', tade::bytes::Bytes::from_static(&[0xff, 0xfe])));
        // from_utf8_lossy never panics on invalid UTF-8, replaces with U+FFFD.
        assert_eq!(kv_str(&m, "message", ""), "\u{FFFD}\u{FFFD}");
    }
}
