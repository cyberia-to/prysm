//! File molecule: particle identity plus a data surface.
//! The spark produced the surface; this is the frame around it.

use super::particle_card;
use crate::theme;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct FileFrame;

pub fn spawn(
    commands: &mut Commands,
    parent: Entity,
    name: &str,
    short_hex: &str,
    detail: Option<&str>,
) -> Entity {
    let root = commands
        .spawn((
            FileFrame,
            Node {
                flex_direction: FlexDirection::Column,
                width: Val::Percent(100.0),
                row_gap: Val::Px(theme::G * 2.0),
                ..default()
            },
            ChildOf(parent),
        ))
        .id();
    particle_card::spawn(commands, root, name, short_hex, detail);
    root
}
