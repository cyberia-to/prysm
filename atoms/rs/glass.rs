use bevy::prelude::*;
use crate::theme;

/// How far a surface sits from the black base. Depth reads as the brightness
/// of the surface's hairline — never as a lighter fill. A lighter fill washes
/// out white content, so every surface in every view stays [`theme::DARK_BASE`].
#[derive(Clone, Copy, Debug)]
pub enum GlassDepth {
    Subtle,
    Background,
    Midground,
    Foreground,
}

impl GlassDepth {
    fn alpha(self) -> f32 {
        match self {
            GlassDepth::Subtle     => theme::SUBTLE,
            GlassDepth::Background => theme::BACKGROUND,
            GlassDepth::Midground  => theme::MIDGROUND,
            GlassDepth::Foreground => theme::FOREGROUND,
        }
    }
}

/// Fill and hairline for a glass surface. Spawn both, and give the `Node` a
/// `border` — without one the hairline has nothing to draw on and the surface
/// disappears into the base.
pub fn glass(depth: GlassDepth) -> (BackgroundColor, BorderColor) {
    (
        BackgroundColor(theme::DARK_BASE),
        BorderColor::all(Color::srgba(1.0, 1.0, 1.0, depth.alpha())),
    )
}

#[derive(Component)]
pub struct Glass;

#[derive(Component)]
pub struct Saber;

pub fn saber_h(commands: &mut ChildSpawnerCommands) {
    commands.spawn((
        Saber,
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(1.0),
            ..default()
        },
        BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.10)),
    ));
}
