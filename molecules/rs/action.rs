use bevy::prelude::*;
use tade::{Chunk, sigil, render, decode_nested};
use crate::theme;
use crate::atoms::glass::{GlassDepth, glass};

#[derive(Component)]
pub struct ActionButton {
    pub label: String,
    pub target_ref: String,
}

pub fn spawn(commands: &mut Commands, parent: Entity, chunk: &Chunk) -> Entity {
    let inner = decode_nested(&chunk.payload);
    let mut label  = String::new();
    let mut target = String::new();
    for c in &inner {
        if c.sigil == sigil::SIG && c.render == render::TEXT {
            label = String::from_utf8_lossy(&c.payload).into_owned();
        } else if c.sigil == sigil::HAX && c.render == render::TEXT {
            target = String::from_utf8_lossy(&c.payload).into_owned();
        }
    }
    if label.is_empty() { label = "action".into(); }

    commands.spawn((
        ActionButton { label: label.clone(), target_ref: target },
        Button,
        Node {
            padding: UiRect::axes(Val::Px(theme::G * 2.0), Val::Px(theme::G * 0.75)),
            margin: UiRect::vertical(Val::Px(2.0)),
            align_self: AlignSelf::FlexStart,
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        },
        glass(GlassDepth::Midground),
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
