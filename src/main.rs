mod games;
mod menu;
mod plugin;
mod settings;
mod stereogram;

use bevy::prelude::*;
use games::{pong::PongPlugin, tetris::TetrisPlugin};
use menu::MenuPlugin;
use plugin::MagicEyePlugin;
use settings::SettingsPlugin;

// Fallback resolution used on native (laptop) builds.
const DEFAULT_WIDTH: u32 = 400;
const DEFAULT_HEIGHT: u32 = 800;

// Maximum resolution on web — caps desktop browsers to a phone-sized viewport.
// Real phones fit under this, so they still use their full screen.
#[cfg(target_arch = "wasm32")]
const MAX_WEB_WIDTH: u32 = 430;
#[cfg(target_arch = "wasm32")]
const MAX_WEB_HEIGHT: u32 = 932;

// ── App state ─────────────────────────────────────────────────────────────────

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    #[default]
    Menu,
    Settings,
    Tetris,
    Pong,
}

// ── Screen measurement ────────────────────────────────────────────────────────

/// On WASM read the browser viewport; on native return the hardcoded defaults.
fn measure_screen() -> (u32, u32) {
    #[cfg(target_arch = "wasm32")]
    {
        if let Some(win) = web_sys::window() {
            let w = win.inner_width().ok()
                .and_then(|v| v.as_f64())
                .unwrap_or(DEFAULT_WIDTH as f64) as u32;
            let h = win.inner_height().ok()
                .and_then(|v| v.as_f64())
                .unwrap_or(DEFAULT_HEIGHT as f64) as u32;
            return (w.min(MAX_WEB_WIDTH), h.min(MAX_WEB_HEIGHT));
        }
    }
    (DEFAULT_WIDTH, DEFAULT_HEIGHT)
}

// ── Entry point ───────────────────────────────────────────────────────────────

fn main() {
    let (width, height) = measure_screen();

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Magic Eye Games".into(),
                resolution: (width, height).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(MagicEyePlugin { width, height })
        .init_state::<AppState>()
        .add_plugins((MenuPlugin, SettingsPlugin, TetrisPlugin, PongPlugin))
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
