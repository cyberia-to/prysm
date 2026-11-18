use bevy::prelude::*;
use crate::theme;
use crate::atoms::glass::{GlassDepth, glass, saber_h};

#[derive(Component)]
pub struct TabItem {
    pub index: usize,
}

#[derive(Component)]
pub struct Commander;

#[derive(Resource, Default)]
pub struct ActiveTab(pub usize);

pub fn spawn_commander(
    parent: &mut ChildSpawnerCommands,
    active_tab: usize,
    labels: &[&str],
) {
    parent
        .spawn((
            Commander,
            Node {
                width: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::NONE),
        ))
        .with_children(|c| {
            saber_h(c);
            c.spawn((
                Node {
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Row,
                    padding: UiRect::all(Val::Px(theme::G)),
                    column_gap: Val::Px(theme::G),
                    border: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                glass(GlassDepth::Background),
            ))
            .with_children(|row| {
                for (i, &label) in labels.iter().enumerate() {
                    row.spawn((
                        TabItem { index: i },
                        Button,
                        Node {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            padding: UiRect::axes(Val::Px(theme::G * 2.0), Val::Px(theme::G * 0.5)),
                            ..default()
                        },
                        BackgroundColor(Color::NONE),
                    ))
                    .with_children(|tab| {
                        tab.spawn((
                            Text::new(label),
                            TextFont { font_size: theme::CAPTION, ..default() },
                            TextColor(if i == active_tab {
                                theme::TEXT_PRIMARY
                            } else {
                                Color::srgba(0.62, 0.62, 0.68, 1.0)
                            }),
                        ));
                        if i == active_tab {
                            tab.spawn((
                                Node {
                                    width: Val::Percent(100.0),
                                    height: Val::Px(2.0),
                                    margin: UiRect::top(Val::Px(4.0)),
                                    ..default()
                                },
                                BackgroundColor(theme::ACID_BLUE),
                            ));
                        }
                    });
                }
            });
        });
}
