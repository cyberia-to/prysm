use bevy::prelude::*;
use tade::Chunk;
use crate::theme;

/// Format a neuron chunk's payload as an `@name` handle: the payload already
/// carrying a leading `@` is passed through unchanged, otherwise one is
/// prepended.
fn format_neuron_name(payload: &[u8]) -> String {
    let raw = String::from_utf8_lossy(payload).into_owned();
    if raw.starts_with('@') { raw } else { format!("@{raw}") }
}

pub fn spawn(commands: &mut Commands, parent: Entity, chunk: &Chunk) -> Entity {
    let name = format_neuron_name(&chunk.payload);
    commands.spawn((
        Text::new(name),
        TextFont { font_size: theme::BODY, ..default() },
        TextColor(theme::ACID_BLUE),
        Node { margin: UiRect::vertical(Val::Px(1.0)), ..default() },
        ChildOf(parent),
    )).id()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prepends_at_when_missing() {
        assert_eq!(format_neuron_name(b"zipf"), "@zipf");
    }

    #[test]
    fn passes_through_when_already_prefixed() {
        assert_eq!(format_neuron_name(b"@zipf"), "@zipf");
    }

    #[test]
    fn empty_payload_becomes_bare_at() {
        assert_eq!(format_neuron_name(b""), "@");
    }

    #[test]
    fn lone_at_payload_passes_through() {
        assert_eq!(format_neuron_name(b"@"), "@");
    }

    #[test]
    fn non_utf8_payload_lossy_decodes_without_panic() {
        assert_eq!(format_neuron_name(&[0xFF, 0xFE]), "@\u{FFFD}\u{FFFD}");
    }

    #[test]
    fn non_utf8_payload_already_prefixed_passes_through() {
        assert_eq!(format_neuron_name(&[b'@', 0xFF]), "@\u{FFFD}");
    }
}
