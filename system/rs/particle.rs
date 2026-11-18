use bevy::prelude::*;
use tade::Chunk;

/// A self-contained visual particle. Owns identity (sigil + render),
/// wire decoding, and Bevy spawning. The error is red intrinsically —
/// no separate "design system" needed.
pub trait Particle: Send + Sync + 'static {
    fn sigil() -> u8 where Self: Sized;
    fn render() -> u8 where Self: Sized;
    fn from_chunk(c: &Chunk) -> Self where Self: Sized;
    fn to_chunk(&self) -> Chunk;
    fn spawn(&self, commands: &mut Commands, parent: Entity) -> Entity;
}
