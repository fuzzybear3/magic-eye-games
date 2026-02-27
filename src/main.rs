mod games;
mod menu;
mod plugin;
mod stereogram;

use bevy::prelude::*;
use games::{pong::PongPlugin, tetris::TetrisPlugin};
use menu::MenuPlugin;
use plugin::MagicEyePlugin;

// ── Window ────────────────────────────────────────────────────────────────────

pub const WIDTH: u32 = 1280;
pub const HEIGHT: u32 = 800;

// ── App state ─────────────────────────────────────────────────────────────────

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    #[default]
    Menu,
    Tetris,
    Pong,
}

// ── Entry point ───────────────────────────────────────────────────────────────

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Magic Eye Games".into(),
                resolution: (WIDTH, HEIGHT).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(MagicEyePlugin {
            width: WIDTH,
            height: HEIGHT,
        })
        .init_state::<AppState>()
        .add_plugins((MenuPlugin, TetrisPlugin, PongPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, global_escape)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn global_escape(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<AppState>>,
    mut next: ResMut<NextState<AppState>>,
) {
    if keys.just_pressed(KeyCode::Escape) && *state.get() != AppState::Menu {
        next.set(AppState::Menu);
    }
}
