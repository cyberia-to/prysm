//! prysm — cyb's visual composition layer.
//!
//! Atoms (glass, text ions), molecules (mind/commander, button, input, and all
//! stream particles), and a palette. Renderer-agnostic in spirit; the Bevy
//! plugin here is one concrete binding.
//!
//! Each particle is self-contained: identity (sigil + render), wire encoding
//! (from/to Chunk), and visual spawning. The error is red intrinsically —
//! no separate design-system glue needed.
//!
//! Two layout paradigms:
//! - `layout::scrollback` — linear Emacs-style scrollback for the terminal
//! - `layout::grid`       — proof-backed grid (formal spec in proofs/Algebra.ei)

#[path = "../system/rs/theme.rs"]    pub mod theme;
#[path = "../system/rs/particle.rs"] pub mod particle;
#[path = "../atoms/rs/mod.rs"]       pub mod atoms;
#[path = "../molecules/rs/mod.rs"]   pub mod molecules;
#[path = "../system/rs/mod.rs"]      pub mod layout;

pub use tade;

pub use theme::{
    G, ACID_BLUE, ACID_GREEN, ACID_RED, ACID_ORANGE, ACID_YELLOW, ACID_INDIGO, ACID_VIOLET,
    SUBTLE, BACKGROUND, MIDGROUND, FOREGROUND,
    H1, H2, H3, BODY, CAPTION, MICRO,
    DARK_BASE, BORDER, TEXT_PRIMARY, TEXT_DIM,
};
pub use atoms::{GlassDepth, Glass, Saber, glass, saber_h};
pub use molecules::{
    TabItem, Commander, ActiveTab, ButtonPrysm, TextInput, CursorBlink,
    spawn_commander, spawn_button, spawn_input,
    text_input_system, input_focus_system,
};
pub use layout::scrollback::{StreamChannel, StreamScrollback, StreamPlugin, dispatch};

pub mod navigation;

use bevy::prelude::*;

pub struct PrysmPlugin;

impl Plugin for PrysmPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(layout::scrollback::StreamPlugin)
            .init_resource::<ActiveTab>()
            .init_resource::<navigation::ActiveDestination>()
            .init_resource::<navigation::DestinationList>()
            .add_systems(Update, (
                molecules::text_input_system,
                molecules::input_focus_system,
            ));
    }
}
