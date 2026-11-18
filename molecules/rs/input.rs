use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::ecs::message::MessageReader;
use bevy::prelude::*;
use crate::theme;
use crate::atoms::glass::{GlassDepth, glass};

#[derive(Component)]
pub struct TextInput {
    pub value:       String,
    pub focused:     bool,
    pub placeholder: String,
}

#[derive(Component)]
pub struct CursorBlink(pub f32);

pub fn text_input_system(
    mut key_evts: MessageReader<KeyboardInput>,
    mut input_q:  Query<(&mut TextInput, &Children)>,
    mut text_q:   Query<&mut Text>,
) {
    let events: Vec<_> = key_evts.read().cloned().collect();
    if events.is_empty() { return; }

    for (mut input, children) in &mut input_q {
        if !input.focused { continue; }
        for ev in &events {
            if !ev.state.is_pressed() { continue; }
            match (&ev.logical_key, &ev.text) {
                (Key::Backspace, _) => { input.value.pop(); }
                (Key::Enter, _)     => {}
                (_, Some(t)) => {
                    for ch in t.chars() {
                        if !ch.is_ascii_control() { input.value.push(ch); }
                    }
                }
                _ => {}
            }
        }
        let display = if input.value.is_empty() {
            format!("{}|", input.placeholder)
        } else {
            format!("{}|", input.value)
        };
        for child in children.iter() {
            if let Ok(mut t) = text_q.get_mut(child) { **t = display.clone(); }
        }
    }
}

pub fn input_focus_system(
    interaction_q: Query<(Entity, &Interaction), (With<TextInput>, Changed<Interaction>)>,
    mut all_q:     Query<(Entity, &mut TextInput)>,
) {
    let mut pressed: Option<Entity> = None;
    for (e, i) in &interaction_q {
        if *i == Interaction::Pressed { pressed = Some(e); }
    }
    let Some(target) = pressed else { return };
    for (e, mut inp) in &mut all_q {
        inp.focused = e == target;
    }
}

pub fn spawn_input(parent: &mut ChildSpawnerCommands, placeholder: &str) -> Entity {
    let placeholder = placeholder.to_string();
    parent
        .spawn((
            Button,
            TextInput { value: String::new(), focused: false, placeholder: placeholder.clone() },
            Node {
                width: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(theme::G)),
                border: UiRect::all(Val::Px(1.0)),
                ..default()
            },
            glass(GlassDepth::Midground),
        ))
        .with_children(|p| {
            p.spawn((
                Text::new(format!("{}|", placeholder)),
                TextFont { font_size: theme::BODY, ..default() },
                TextColor(theme::TEXT_DIM),
            ));
        })
        .id()
}
