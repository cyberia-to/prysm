use bevy::prelude::*;
use tade::Chunk;
use crate::theme;

#[derive(Component)]
pub struct ProgressBar;

#[derive(Component)]
pub struct ProgressFill;

#[derive(Component)]
pub struct ProgressLabel;

pub fn spawn(commands: &mut Commands, parent: Entity, chunk: &Chunk) -> Entity {
    let m = tade::read_kv(&chunk.payload);
    let label   = kv_str(&m, "label", "");
    let current = kv_u64(&m, "current");
    let total   = kv_u64(&m, "total");
    let percent = progress_percent(current, total);

    let container = commands.spawn((
        ProgressBar,
        Node {
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.0),
            padding: UiRect::vertical(Val::Px(2.0)),
            ..default()
        },
        ChildOf(parent),
    )).id();

    commands.spawn((
        ProgressLabel,
        Text::new(label),
        TextFont { font_size: theme::MICRO, ..default() },
        TextColor(theme::TEXT_DIM),
        ChildOf(container),
    ));

    let track = commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(4.0),
            margin: UiRect::top(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.08)),
        ChildOf(container),
    )).id();

    commands.spawn((
        ProgressFill,
        Node {
            width: Val::Percent(percent),
            height: Val::Percent(100.0),
            ..default()
        },
        BackgroundColor(theme::ACID_BLUE),
        ChildOf(track),
    ));

    container
}

fn kv_str(m: &std::collections::HashMap<String, Chunk>, key: &str, default: &str) -> String {
    m.get(key)
        .map(|c| String::from_utf8_lossy(&c.payload).into_owned())
        .unwrap_or_else(|| default.to_string())
}

fn kv_u64(m: &std::collections::HashMap<String, Chunk>, key: &str) -> u64 {
    m.get(key)
        .and_then(|c| String::from_utf8_lossy(&c.payload).parse().ok())
        .unwrap_or(0)
}

/// `current` of `total` as a fill percent, `0..=100`. `total == 0` is `0`,
/// not a division by zero; `current > total` clamps at `100` rather than
/// overshooting the bar.
fn progress_percent(current: u64, total: u64) -> f32 {
    if total == 0 {
        return 0.0;
    }
    (current as f32 / total as f32 * 100.0).clamp(0.0, 100.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn progress_percent_zero_total_is_zero() {
        assert_eq!(progress_percent(0, 0), 0.0);
        assert_eq!(progress_percent(5, 0), 0.0);
    }

    #[test]
    fn progress_percent_midpoint() {
        assert_eq!(progress_percent(1, 2), 50.0);
    }

    #[test]
    fn progress_percent_complete() {
        assert_eq!(progress_percent(10, 10), 100.0);
    }

    #[test]
    fn progress_percent_clamps_when_current_exceeds_total() {
        assert_eq!(progress_percent(15, 10), 100.0);
    }

    #[test]
    fn progress_percent_zero_current() {
        assert_eq!(progress_percent(0, 10), 0.0);
    }

    #[test]
    fn kv_str_reads_present_key() {
        let mut m = std::collections::HashMap::new();
        m.insert("label".to_string(), Chunk::text("uploading"));
        assert_eq!(kv_str(&m, "label", ""), "uploading");
    }

    #[test]
    fn kv_str_falls_back_to_default() {
        let m = std::collections::HashMap::new();
        assert_eq!(kv_str(&m, "label", "working"), "working");
    }

    #[test]
    fn kv_u64_parses_present_key() {
        let mut m = std::collections::HashMap::new();
        m.insert("current".to_string(), Chunk::text("42"));
        assert_eq!(kv_u64(&m, "current"), 42);
    }

    #[test]
    fn kv_u64_defaults_to_zero_when_missing() {
        let m = std::collections::HashMap::new();
        assert_eq!(kv_u64(&m, "current"), 0);
    }

    #[test]
    fn kv_u64_defaults_to_zero_on_unparseable_payload() {
        let mut m = std::collections::HashMap::new();
        m.insert("current".to_string(), Chunk::text("not-a-number"));
        assert_eq!(kv_u64(&m, "current"), 0);
    }
}
