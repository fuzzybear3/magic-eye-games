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
            parent.spawn((
                Text::new("[ 1 ]  Tetris"),
                TextFont { font_size: 40.0, ..default() },
                TextColor(Color::srgb(0.8, 0.9, 1.0)),
            ));
            parent.spawn((
                Text::new("[ 2 ]  Two-Player Pong"),
                TextFont { font_size: 40.0, ..default() },
                TextColor(Color::srgb(0.8, 0.9, 1.0)),
            ));
            parent.spawn((
                Text::new("ESC to return here"),
                TextFont { font_size: 24.0, ..default() },
                TextColor(Color::srgb(0.5, 0.5, 0.5)),
            ));
        });
}

fn menu_input(keys: Res<ButtonInput<KeyCode>>, mut next: ResMut<NextState<AppState>>) {
    if keys.just_pressed(KeyCode::Digit1) {
        next.set(AppState::Tetris);
    }
    if keys.just_pressed(KeyCode::Digit2) {
        next.set(AppState::Pong);
    }
}

fn despawn_menu(mut commands: Commands, query: Query<Entity, With<MenuRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
