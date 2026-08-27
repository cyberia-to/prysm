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
    // The hairline carries depth in its alpha and the interface's own hue in
    // its colour. White at low alpha over black is grey, and grey edges make
    // a black surface look grey — the one thing this palette does not do.
    let Color::Srgba(acid) = theme::ACID_GREEN else { unreachable!() };
    (
        BackgroundColor(theme::DARK_BASE),
        BorderColor::all(Color::srgba(acid.red, acid.green, acid.blue, depth.alpha())),
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
        // Same rule as the glass hairline: tinted, never neutral grey.
        BackgroundColor(Color::srgba(0.13, 0.92, 0.51, 0.14)),
    ));
}
