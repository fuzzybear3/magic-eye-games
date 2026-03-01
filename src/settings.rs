use bevy::prelude::*;

use crate::plugin::DebugSettings;
use crate::AppState;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Settings), spawn_settings)
            .add_systems(
                Update,
                (handle_input, sync_toggle_text).run_if(in_state(AppState::Settings)),
            )
            .add_systems(OnExit(AppState::Settings), despawn_settings);
    }
}

#[derive(Component)]
struct SettingsRoot;

#[derive(Component)]
struct MagicEyeToggle;

#[derive(Component)]
struct MagicEyeToggleText;

#[derive(Component)]
struct BackButton;

fn spawn_settings(mut commands: Commands, debug: Res<DebugSettings>) {
    let toggle_label = toggle_label(debug.magic_eye_enabled);

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(32.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.85)),
            SettingsRoot,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Settings"),
                TextFont { font_size: 60.0, ..default() },
                TextColor(Color::WHITE),
            ));

            // Magic Eye toggle
            parent
                .spawn((
                    Button,
                    MagicEyeToggle,
                    Node {
                        padding: UiRect::axes(Val::Px(32.0), Val::Px(16.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.15, 0.15, 0.15, 0.9)),
                ))
                .with_children(|btn| {
                    btn.spawn((
                        Text::new(toggle_label),
                        TextFont { font_size: 36.0, ..default() },
                        TextColor(Color::srgb(0.8, 0.9, 1.0)),
                        MagicEyeToggleText,
                    ));
                });

            // Back button
            parent
                .spawn((
                    Button,
                    BackButton,
                    Node {
                        padding: UiRect::axes(Val::Px(32.0), Val::Px(16.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
                ))
                .with_children(|btn| {
                    btn.spawn((
                        Text::new("Back"),
                        TextFont { font_size: 28.0, ..default() },
                        TextColor(Color::srgb(0.5, 0.5, 0.5)),
                    ));
                });
        });
}

fn handle_input(
    toggle_q: Query<&Interaction, (Changed<Interaction>, With<MagicEyeToggle>)>,
    back_q: Query<&Interaction, (Changed<Interaction>, With<BackButton>)>,
    mut debug: ResMut<DebugSettings>,
    mut next: ResMut<NextState<AppState>>,
) {
    for interaction in &toggle_q {
        if *interaction == Interaction::Pressed {
            debug.magic_eye_enabled = !debug.magic_eye_enabled;
        }
    }
    for interaction in &back_q {
        if *interaction == Interaction::Pressed {
            next.set(AppState::Menu);
        }
    }
}

fn sync_toggle_text(
    debug: Res<DebugSettings>,
    mut text_q: Query<&mut Text, With<MagicEyeToggleText>>,
) {
    if debug.is_changed() {
        if let Ok(mut text) = text_q.single_mut() {
            *text = Text::new(toggle_label(debug.magic_eye_enabled));
        }
    }
}

fn despawn_settings(mut commands: Commands, query: Query<Entity, With<SettingsRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn toggle_label(enabled: bool) -> &'static str {
    if enabled { "Magic Eye: ON" } else { "Magic Eye: OFF" }
}
