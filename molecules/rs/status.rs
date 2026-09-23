use bevy::prelude::*;
use tade::Chunk;
use crate::theme;

pub fn spawn(commands: &mut Commands, parent: Entity, chunk: &Chunk) -> Entity {
    let m = tade::read_kv(&chunk.payload);
    let code = status_code(&m);
    let color = status_color(code);
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(1.0),
            margin: UiRect::vertical(Val::Px(4.0)),
            ..default()
        },
        BackgroundColor(color),
        ChildOf(parent),
    )).id()
}

/// Read the `code` key, defaulting to 0 (ok) on a missing key or an
/// unparseable payload.
fn status_code(m: &std::collections::HashMap<String, Chunk>) -> i32 {
    m.get("code")
        .and_then(|c| String::from_utf8_lossy(&c.payload).parse().ok())
        .unwrap_or(0)
}

/// A faint white bar for `code == 0`; a red-tinted bar for anything else.
fn status_color(code: i32) -> Color {
    if code == 0 {
        Color::srgba(1.0, 1.0, 1.0, 0.06)
    } else {
        Color::srgba(theme::ACID_RED.to_srgba().red, 0.1, 0.1, 0.3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kv(pairs: &[(&str, &str)]) -> std::collections::HashMap<String, Chunk> {
        pairs
            .iter()
            .map(|(k, v)| (k.to_string(), Chunk::text(v)))
            .collect()
    }

    #[test]
    fn missing_code_defaults_to_ok() {
        assert_eq!(status_code(&kv(&[])), 0);
    }

    #[test]
    fn zero_code_parses_as_ok() {
        assert_eq!(status_code(&kv(&[("code", "0")])), 0);
    }

    #[test]
    fn nonzero_code_parses() {
        assert_eq!(status_code(&kv(&[("code", "404")])), 404);
        assert_eq!(status_code(&kv(&[("code", "-1")])), -1);
    }

    #[test]
    fn unparseable_code_defaults_to_ok() {
        assert_eq!(status_code(&kv(&[("code", "not-a-number")])), 0);
        assert_eq!(status_code(&kv(&[("code", "")])), 0);
    }

    #[test]
    fn ok_status_is_faint_white() {
        let c = status_color(0).to_srgba();
        assert_eq!((c.red, c.green, c.blue, c.alpha), (1.0, 1.0, 1.0, 0.06));
    }

    #[test]
    fn nonzero_status_is_red_tinted() {
        let c = status_color(404).to_srgba();
        assert_eq!((c.green, c.blue, c.alpha), (0.1, 0.1, 0.3));
        assert!(c.red > 0.0, "red channel should carry theme::ACID_RED's red");
    }
}
