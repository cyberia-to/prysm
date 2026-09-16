use crate::atoms::glass::{GlassDepth, glass};
use crate::theme;
use bevy::prelude::*;
use tape::{Chunk, decode_nested, render, sigil};

/// Census card: a big value over a micro caption, glass hairline.
pub fn spawn(commands: &mut Commands, parent: Entity, chunk: &Chunk) -> Entity {
    let inner = decode_nested(&chunk.payload);
    let mut value = String::new();
    let mut caption = String::new();
    for c in &inner {
        if c.sigil == sigil::HAX && c.render == render::TEXT {
            value = String::from_utf8_lossy(&c.payload).into_owned();
        } else if c.sigil == sigil::SIG && c.render == render::TEXT {
            caption = String::from_utf8_lossy(&c.payload).into_owned();
        }
    }
    let (fill, hair) = glass(GlassDepth::Foreground);
    commands
        .spawn((
            Node {
                flex_grow: 1.0,
                flex_basis: Val::Px(0.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect::axes(Val::Px(theme::G), Val::Px(theme::G * 1.5)),
                border: UiRect::all(Val::Px(1.0)),
                row_gap: Val::Px(2.0),
                ..default()
            },
            fill,
            hair,
            ChildOf(parent),
        ))
        .with_children(|card| {
            card.spawn((
                Text::new(value),
                TextFont {
                    font_size: theme::H2,
                    ..default()
                },
                TextColor(theme::ACID_GREEN),
            ));
            card.spawn((
                Text::new(caption),
                TextFont {
                    font_size: theme::MICRO,
                    ..default()
                },
                TextColor(theme::TEXT_DIM),
            ));
        })
        .id()
}
