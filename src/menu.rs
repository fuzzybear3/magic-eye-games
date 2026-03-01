use bevy::prelude::*;

use crate::plugin::{DepthSprite, ScreenSize};
use crate::AppState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Menu), (spawn_menu_scene, spawn_menu))
            .add_systems(Update, menu_input.run_if(in_state(AppState::Menu)))
            .add_systems(OnExit(AppState::Menu), despawn_menu);
    }
}

// ── Markers ───────────────────────────────────────────────────────────────────

/// Root of the Bevy UI overlay.
#[derive(Component)]
struct MenuRoot;

/// World-space depth sprites that make the menu background magic-eye.
#[derive(Component)]
struct MenuScene;

/// Marks a button with the state it transitions to when pressed.
#[derive(Component)]
struct GameChoice(AppState);

// ── Scene (stereogram depth sprites) ─────────────────────────────────────────

fn spawn_menu_scene(mut commands: Commands, screen: Res<ScreenSize>) {
    let w = screen.width as f32;
    let h = screen.height as f32;

    // Far background
    commands.spawn((
        MenuScene,
        DepthSprite { size: Vec2::new(w, h), depth: 0.0 },
        Transform::default(),
    ));

    // Floating panel behind the menu card
    commands.spawn((
        MenuScene,
        DepthSprite { size: Vec2::new(w * 0.85, h * 0.55), depth: 0.25 },
        Transform::from_xyz(0.0, -20.0, 0.0),
    ));

    // Tetris button highlight
    commands.spawn((
        MenuScene,
        DepthSprite { size: Vec2::new(w * 0.7, 56.0), depth: 0.55 },
        Transform::from_xyz(0.0, 60.0, 0.0),
    ));

    // Pong button highlight
    commands.spawn((
        MenuScene,
        DepthSprite { size: Vec2::new(w * 0.7, 56.0), depth: 0.55 },
        Transform::from_xyz(0.0, -40.0, 0.0),
    ));
}

// ── UI overlay ────────────────────────────────────────────────────────────────

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
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.45)),
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

// ── Input ─────────────────────────────────────────────────────────────────────

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

// ── Cleanup ───────────────────────────────────────────────────────────────────

fn despawn_menu(
    mut commands: Commands,
    ui: Query<Entity, With<MenuRoot>>,
    scene: Query<Entity, With<MenuScene>>,
) {
    for e in ui.iter().chain(scene.iter()) {
        commands.entity(e).despawn();
    }
}
