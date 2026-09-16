use std::collections::HashMap;
use bevy::prelude::*;
use crossbeam_channel::{Receiver, Sender, unbounded};
use tade::{Chunk, ChunkId, sigil, render, read_kv};

// ── StreamChannel ─────────────────────────────────────────────────────────────

/// Inject this as a resource to push chunks from outside the Bevy main thread.
#[derive(Resource, Clone)]
pub struct StreamChannel {
    pub tx: Sender<Chunk>,
    pub rx: Receiver<Chunk>,
}

impl Default for StreamChannel {
    fn default() -> Self {
        let (tx, rx) = unbounded();
        Self { tx, rx }
    }
}

// ── StreamScrollback ──────────────────────────────────────────────────────────

/// Vertical list of rendered particles. One per terminal session.
#[derive(Component, Default)]
pub struct StreamScrollback {
    pub id_map: HashMap<ChunkId, Entity>,
    pub last_status: Option<i32>,
}

// ── dispatch ──────────────────────────────────────────────────────────────────

/// Spawn the Bevy widget for `chunk` as a child of `parent`. Returns the root entity.
/// Each particle owns its own visual — no separate design system needed.
pub fn dispatch(commands: &mut Commands, parent: Entity, chunk: &Chunk) -> Entity {
    match (chunk.sigil, chunk.render) {
        (sigil::HAX, render::TEXT)      => crate::atoms::text::spawn(commands, parent, chunk),
        (sigil::SIG, render::TEXT)      => crate::atoms::text::spawn_annotation(commands, parent, chunk),
        (sigil::PAT, render::TEXT)      => crate::molecules::neuron::spawn(commands, parent, chunk),
        (sigil::DOT, render::LOG)       => crate::molecules::log::spawn(commands, parent, chunk),
        (sigil::ZAP, render::ERROR)     => crate::molecules::error::spawn(commands, parent, chunk),
        (sigil::DOT, render::STATUS)    => crate::molecules::status::spawn(commands, parent, chunk),
        (sigil::DOT, render::PROGRESS)  => crate::molecules::progress::spawn(commands, parent, chunk),
        (sigil::ZAP, render::COMPONENT) => crate::molecules::action::spawn(commands, parent, chunk),
        (sigil::BUC, render::COMPONENT) => crate::atoms::stat::spawn(commands, parent, chunk),
        (sigil::LUS, render::COMPONENT) => crate::molecules::component::spawn_row(commands, parent, chunk),
        (sigil::BAR, render::COMPONENT) => crate::molecules::component::spawn(commands, parent, chunk),
        (sigil::FAS, render::COMPONENT) => crate::molecules::component::spawn_scope(commands, parent, chunk),
        (sigil::HAX, render::TABLE)     => crate::molecules::table::spawn(commands, parent, chunk),
        // Structured results: a tree of pairs. Without these two arms every
        // record, list and nested value fell through to the debug label
        // below, which prints one grey line holding the first 60 bytes — so
        // a 60-row result rendered as a single truncated line and there was
        // nothing left to scroll.
        (sigil::FAS, render::STRUCT)    => crate::molecules::component::spawn_scope(commands, parent, chunk),
        (sigil::COL, render::STRUCT)    => crate::molecules::component::spawn_pair(commands, parent, chunk),
        _ => {
            let label = format!(
                "[?{} {}] {}",
                chunk.sigil as char, chunk.render as char,
                String::from_utf8_lossy(&chunk.payload).chars().take(60).collect::<String>()
            );
            commands.spawn((
                Text::new(label),
                TextFont { font_size: crate::theme::MICRO, ..default() },
                TextColor(Color::srgba(0.5, 0.5, 0.5, 1.0)),
                ChildOf(parent),
            )).id()
        }
    }
}

// ── consume_chunks system ─────────────────────────────────────────────────────

pub fn consume_chunks(
    channel: Res<StreamChannel>,
    mut scrollback_q: Query<(Entity, &mut StreamScrollback)>,
    mut commands: Commands,
) {
    let Ok((scrollback_entity, mut scrollback)) = scrollback_q.single_mut() else { return };

    for chunk in channel.rx.try_iter() {
        if chunk.sigil == sigil::DOT && chunk.render == render::STATUS {
            let m = read_kv(&chunk.payload);
            let code: i32 = m.get("code")
                .and_then(|c| String::from_utf8_lossy(&c.payload).parse().ok())
                .unwrap_or(0);
            scrollback.last_status = Some(code);
        }

        if let Some(id) = chunk.id {
            if let Some(old) = scrollback.id_map.remove(&id) {
                commands.entity(old).despawn();
            }
        }

        let entity = dispatch(&mut commands, scrollback_entity, &chunk);

        if let Some(id) = chunk.id {
            scrollback.id_map.insert(id, entity);
        }
    }
}

// ── StreamPlugin ──────────────────────────────────────────────────────────────

pub struct StreamPlugin;

impl Plugin for StreamPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StreamChannel>()
            .add_systems(Update, consume_chunks);
    }
}
