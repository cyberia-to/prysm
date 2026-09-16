//! Protocol particle as a widget: name, short hash, optional rank.
//! Not the tade particle (`crate::particle`). This is the cybergraph atom.

use crate::atoms::glass::{GlassDepth, glass};
use crate::theme;
use bevy::prelude::*;

/// Identity of a particle on screen — table, now chroma, neighbor, file header.
#[derive(Component, Debug, Clone)]
pub struct ParticleCard {
    pub hex: String,
    pub name: String,
}

pub fn spawn(
    commands: &mut Commands,
    parent: Entity,
    name: &str,
    short_hex: &str,
    detail: Option<&str>,
) -> Entity {
    let (fill, hair) = glass(GlassDepth::Foreground);
    commands
        .spawn((
            ParticleCard {
                hex: short_hex.to_string(),
                name: name.to_string(),
            },
            Node {
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                row_gap: Val::Px(2.0),
                padding: UiRect::all(Val::Px(theme::G * 1.5)),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            fill,
            hair,
            ChildOf(parent),
        ))
        .with_children(|card| {
            card.spawn((
                Text::new(name.to_string()),
                TextFont {
                    font_size: theme::H3,
                    ..default()
                },
                TextColor(theme::TEXT_PRIMARY),
                TextLayout::new_with_no_wrap(),
            ));
            let meta = match detail {
                Some(d) => format!("{short_hex}   {d}"),
                None => short_hex.to_string(),
            };
            card.spawn((
                Text::new(meta),
                TextFont {
                    font_size: theme::MICRO,
                    ..default()
                },
                TextColor(theme::TEXT_DIM),
            ));
        })
        .id()
}
