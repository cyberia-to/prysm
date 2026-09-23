use bevy::input::keyboard::{Key, KeyboardInput};
use bevy::ecs::message::MessageReader;
use bevy::prelude::*;
use crate::theme;
use crate::atoms::glass::{GlassDepth, glass_bg};

#[derive(Component)]
pub struct TextInput {
    pub value:       String,
    pub focused:     bool,
    pub placeholder: String,
}

#[derive(Component)]
pub struct CursorBlink(pub f32);

/// Apply one logical-key press to `value`: backspace pops a character,
/// enter is a no-op (submission is handled elsewhere), any other key with
/// echoed text appends its non-control characters.
fn apply_key(value: &mut String, key: &Key, text: Option<&str>) {
    match (key, text) {
        (Key::Backspace, _) => { value.pop(); }
        (Key::Enter, _)     => {}
        (_, Some(t)) => {
            for ch in t.chars() {
                if !ch.is_ascii_control() { value.push(ch); }
            }
        }
        _ => {}
    }
}

/// The `value|`/`placeholder|` caret-suffixed line a focused text input shows.
fn display_text(value: &str, placeholder: &str) -> String {
    if value.is_empty() { format!("{placeholder}|") } else { format!("{value}|") }
}

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
            apply_key(&mut input.value, &ev.logical_key, ev.text.as_deref());
        }
        let display = display_text(&input.value, &input.placeholder);
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
                ..default()
            },
            glass_bg(GlassDepth::Midground),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn backspace_pops_a_character() {
        let mut value = "abc".to_string();
        apply_key(&mut value, &Key::Backspace, None);
        assert_eq!(value, "ab");
    }

    #[test]
    fn backspace_on_empty_value_is_a_no_op() {
        let mut value = String::new();
        apply_key(&mut value, &Key::Backspace, None);
        assert_eq!(value, "");
    }

    #[test]
    fn enter_is_a_no_op_even_with_echoed_text() {
        let mut value = "abc".to_string();
        apply_key(&mut value, &Key::Enter, Some("\r"));
        assert_eq!(value, "abc");
    }

    #[test]
    fn echoed_text_appends_non_control_characters() {
        let mut value = "ab".to_string();
        apply_key(&mut value, &Key::Character("c".into()), Some("c"));
        assert_eq!(value, "abc");
    }

    #[test]
    fn control_characters_in_echoed_text_are_dropped() {
        let mut value = "ab".to_string();
        apply_key(&mut value, &Key::Character("\t".into()), Some("\tc\u{7f}"));
        assert_eq!(value, "abc");
    }

    #[test]
    fn no_echoed_text_and_not_backspace_or_enter_is_a_no_op() {
        let mut value = "ab".to_string();
        apply_key(&mut value, &Key::ArrowLeft, None);
        assert_eq!(value, "ab");
    }

    #[test]
    fn display_text_shows_placeholder_with_caret_when_empty() {
        assert_eq!(display_text("", "search"), "search|");
    }

    #[test]
    fn display_text_shows_value_with_caret_when_nonempty() {
        assert_eq!(display_text("hello", "search"), "hello|");
    }
}
