use bevy::prelude::*;

use crate::AppState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Menu), spawn_menu)
            .add_systems(Update, menu_input.run_if(in_state(AppState::Menu)))
            .add_systems(OnExit(AppState::Menu), despawn_menu);
    }
}

#[derive(Component)]
struct MenuRoot;

/// Marks a button with the state it transitions to when pressed.
#[derive(Component)]
struct GameChoice(AppState);

fn spawn_menu(mut commands: Commands) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                row_gap: Val::Px(24.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.65)),
            MenuRoot,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("MAGIC EYE GAMES"),
                TextFont { font_size: 72.0, ..default() },
                TextColor(Color::WHITE),
            ));

            spawn_button(parent, "Tetris", AppState::Tetris);
            spawn_button(parent, "Two-Player Pong", AppState::Pong);

            parent.spawn((
                Text::new("tap to select"),
                TextFont { font_size: 24.0, ..default() },
                TextColor(Color::srgb(0.5, 0.5, 0.5)),
            ));
        });
}

fn spawn_button(parent: &mut ChildSpawnerCommands, label: &str, choice: AppState) {
    parent
        .spawn((
            Button,
            GameChoice(choice),
            Node {
                padding: UiRect::axes(Val::Px(32.0), Val::Px(16.0)),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0)),
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(label),
                TextFont { font_size: 40.0, ..default() },
                TextColor(Color::srgb(0.8, 0.9, 1.0)),
            ));
        });
}

fn menu_input(
    interaction_q: Query<(&Interaction, &GameChoice), Changed<Interaction>>,
    mut next: ResMut<NextState<AppState>>,
) {
    for (interaction, choice) in &interaction_q {
        if *interaction == Interaction::Pressed {
            next.set(choice.0.clone());
        }
    }
}

fn despawn_menu(mut commands: Commands, query: Query<Entity, With<MenuRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
