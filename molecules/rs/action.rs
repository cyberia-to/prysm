use bevy::prelude::*;
use tade::{Chunk, sigil, render, decode_nested};
use crate::theme;
use crate::atoms::glass::{GlassDepth, glass_bg};

#[derive(Component)]
pub struct ActionButton {
    pub label: String,
    pub target_ref: String,
}

/// Pull `(label, target_ref)` out of an action chunk's nested `(~, t)`/`(#, t)`
/// children. An empty or missing label falls back to `"action"`; a missing
/// target is the empty string, same as `ActionButton::target_ref` always was.
fn parse_action(chunk: &Chunk) -> (String, String) {
    let mut label  = String::new();
    let mut target = String::new();
    for c in decode_nested(&chunk.payload) {
        if c.sigil == sigil::SIG && c.render == render::TEXT {
            label = String::from_utf8_lossy(&c.payload).into_owned();
        } else if c.sigil == sigil::HAX && c.render == render::TEXT {
            target = String::from_utf8_lossy(&c.payload).into_owned();
        }
    }
    if label.is_empty() { label = "action".into(); }
    (label, target)
}

pub fn spawn(commands: &mut Commands, parent: Entity, chunk: &Chunk) -> Entity {
    let (label, target) = parse_action(chunk);

    commands.spawn((
        ActionButton { label: label.clone(), target_ref: target },
        Button,
        Node {
            padding: UiRect::axes(Val::Px(theme::G * 2.0), Val::Px(theme::G * 0.75)),
            margin: UiRect::vertical(Val::Px(2.0)),
            align_self: AlignSelf::FlexStart,
            ..default()
        },
        glass_bg(GlassDepth::Midground),
        ChildOf(parent),
    ))
    .with_children(|b| {
        b.spawn((
            Text::new(label),
            TextFont { font_size: theme::BODY, ..default() },
            TextColor(theme::TEXT_PRIMARY),
        ));
    })
    .id()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tade::{bytes::Bytes, encode_nested};

    fn wrapper(children: &[Chunk]) -> Chunk {
        Chunk::new(sigil::ZAP, render::COMPONENT, encode_nested(children))
    }

    #[test]
    fn label_and_target_from_sig_and_hax_children() {
        let chunk = wrapper(&[Chunk::annotation("open"), Chunk::text("cyb://particle/abcd")]);
        let (label, target) = parse_action(&chunk);
        assert_eq!(label, "open");
        assert_eq!(target, "cyb://particle/abcd");
    }

    #[test]
    fn missing_label_falls_back_to_action() {
        let chunk = wrapper(&[Chunk::text("cyb://particle/abcd")]);
        let (label, target) = parse_action(&chunk);
        assert_eq!(label, "action");
        assert_eq!(target, "cyb://particle/abcd");
    }

    #[test]
    fn empty_label_falls_back_to_action() {
        let chunk = wrapper(&[Chunk::annotation(""), Chunk::text("t")]);
        let (label, _) = parse_action(&chunk);
        assert_eq!(label, "action");
    }

    #[test]
    fn missing_target_is_empty_string() {
        let chunk = wrapper(&[Chunk::annotation("open")]);
        let (label, target) = parse_action(&chunk);
        assert_eq!(label, "open");
        assert_eq!(target, "");
    }

    #[test]
    fn no_children_falls_back_to_action_with_empty_target() {
        let chunk = wrapper(&[]);
        let (label, target) = parse_action(&chunk);
        assert_eq!(label, "action");
        assert_eq!(target, "");
    }

    #[test]
    fn unrelated_sigil_or_render_children_are_ignored() {
        let chunk = wrapper(&[
            Chunk::annotation("open"),
            Chunk::text("cyb://particle/abcd"),
            Chunk::new(sigil::COL, render::TEXT, Bytes::from_static(b"noise")),
            Chunk::new(sigil::SIG, render::STRUCT, Bytes::from_static(b"not-text-render")),
        ]);
        let (label, target) = parse_action(&chunk);
        assert_eq!(label, "open");
        assert_eq!(target, "cyb://particle/abcd");
    }

    #[test]
    fn non_utf8_payload_lossy_decodes_without_panic() {
        let chunk = wrapper(&[
            Chunk::new(sigil::SIG, render::TEXT, Bytes::from_static(&[0xFF, 0xFE])),
            Chunk::new(sigil::HAX, render::TEXT, Bytes::from_static(&[0xFF, 0xFE])),
        ]);
        let (label, target) = parse_action(&chunk);
        assert_eq!(label, "\u{FFFD}\u{FFFD}");
        assert_eq!(target, "\u{FFFD}\u{FFFD}");
    }

    #[test]
    fn last_matching_child_wins_on_duplicates() {
        let chunk = wrapper(&[
            Chunk::annotation("first"),
            Chunk::annotation("second"),
        ]);
        let (label, _) = parse_action(&chunk);
        assert_eq!(label, "second");
    }
}
