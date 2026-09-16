use bevy::prelude::*;

pub const G: f32 = 8.0;

pub const ACID_BLUE:   Color = Color::srgb(0.12, 0.80, 0.99);
pub const ACID_GREEN:  Color = Color::srgb(0.13, 0.92, 0.51);
pub const ACID_RED:    Color = Color::srgb(1.0,  0.25, 0.25);
pub const ACID_ORANGE: Color = Color::srgb(1.0,  0.55, 0.10);
pub const ACID_YELLOW: Color = Color::srgb(1.0,  0.95, 0.10);
pub const ACID_INDIGO: Color = Color::srgb(0.35, 0.35, 1.0);
pub const ACID_VIOLET: Color = Color::srgb(0.75, 0.25, 1.0);

pub const SUBTLE:     f32 = 0.07;
pub const BACKGROUND: f32 = 0.15;
pub const MIDGROUND:  f32 = 0.38;
pub const FOREGROUND: f32 = 0.70;

pub const H1:      f32 = 32.0;
pub const H2:      f32 = 24.0;
pub const H3:      f32 = 20.0;
pub const BODY:    f32 = 16.0;
pub const CAPTION: f32 = 14.0;
pub const MICRO:   f32 = 12.0;

/// The widest a column of text grows before the eye stops tracking line to
/// line. It is a ceiling, not a width: a viewport narrower than this gets the
/// whole of itself, because a phone has no width to give away to margins.
pub const MEASURE: f32 = 760.0;

/// The one background. Every surface in every view sits on pure black — depth
/// comes from [`BORDER`] and the glass alphas, never from a lighter fill.
pub const DARK_BASE:     Color = Color::srgba(0.0, 0.0, 0.0, 1.0);
/// Hairline that separates one black surface from the next.
///
/// Tinted, never neutral: a grey line on black reads as a grey box, and the
/// surface stops looking black at all. This is the acid green the rest of the
/// interface speaks, dimmed until it is just structure.
pub const BORDER:        Color = Color::srgba(0.13, 0.92, 0.51, 0.22);
pub const TEXT_PRIMARY:  Color = Color::srgba(0.95, 0.95, 0.96, 1.0);
pub const TEXT_DIM:      Color = Color::srgba(0.45, 0.45, 0.50, 1.0);
