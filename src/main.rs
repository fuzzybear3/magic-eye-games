mod plugin;
mod stereogram;

use bevy::prelude::*;
use plugin::{DepthSprite, MagicEyePlugin};

const WIDTH:  u32 = 1280;
const HEIGHT: u32 = 800;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Magic Eye Game".into(),
                resolution: (WIDTH, HEIGHT).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(MagicEyePlugin { width: WIDTH, height: HEIGHT })
        .add_systems(Startup, setup)
        .add_systems(Update, (move_player, update_ball))
        .run();
}

// ── Components ────────────────────────────────────────────────────────────────

#[derive(Component)] struct Player;
#[derive(Component)] struct Ball;
#[derive(Component)] struct Velocity(Vec2);

// ── Setup ─────────────────────────────────────────────────────────────────────

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    // Background  (depth 0.05 – just above zero so it registers in the depth map)
    commands.spawn((
        DepthSprite { size: Vec2::new(WIDTH as f32, HEIGHT as f32), depth: 0.05 },
        Transform::default(),
    ));

    // Platforms  (depth 0.35)
    for (x, y, w, h) in [
        (-200.0_f32, -80.0_f32, 200.0_f32, 25.0_f32),
        (  50.0,      60.0,     180.0,      25.0),
        ( 220.0,    -140.0,     160.0,      25.0),
    ] {
        commands.spawn((
            DepthSprite { size: Vec2::new(w, h), depth: 0.35 },
            Transform::from_xyz(x, y, 0.0),
        ));
    }

    // Player  (depth 0.7) – move with WASD or arrow keys
    commands.spawn((
        Player,
        DepthSprite { size: Vec2::new(40.0, 40.0), depth: 0.7 },
        Transform::from_xyz(-150.0, 0.0, 0.0),
    ));

    // Bouncing ball  (depth 0.9 – closest, most "popping out")
    commands.spawn((
        Ball,
        Velocity(Vec2::new(160.0, 110.0)),
        DepthSprite { size: Vec2::new(30.0, 30.0), depth: 0.9 },
        Transform::from_xyz(80.0, 80.0, 0.0),
    ));
}

// ── Game systems ──────────────────────────────────────────────────────────────

fn move_player(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let speed = 200.0;
    let mut dir = Vec2::ZERO;

    if keys.pressed(KeyCode::ArrowUp)    || keys.pressed(KeyCode::KeyW) { dir.y += 1.0; }
    if keys.pressed(KeyCode::ArrowDown)  || keys.pressed(KeyCode::KeyS) { dir.y -= 1.0; }
    if keys.pressed(KeyCode::ArrowLeft)  || keys.pressed(KeyCode::KeyA) { dir.x -= 1.0; }
    if keys.pressed(KeyCode::ArrowRight) || keys.pressed(KeyCode::KeyD) { dir.x += 1.0; }

    let Ok(mut t) = query.single_mut() else { return };
    t.translation += dir.normalize_or_zero().extend(0.0) * speed * time.delta_secs();

    let half_w = WIDTH  as f32 / 2.0 - 20.0;
    let half_h = HEIGHT as f32 / 2.0 - 20.0;
    t.translation.x = t.translation.x.clamp(-half_w, half_w);
    t.translation.y = t.translation.y.clamp(-half_h, half_h);
}

fn update_ball(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Ball>>,
) {
    let half_w = WIDTH  as f32 / 2.0 - 15.0;
    let half_h = HEIGHT as f32 / 2.0 - 15.0;

    for (mut t, mut vel) in &mut query {
        t.translation += vel.0.extend(0.0) * time.delta_secs();

        if t.translation.x.abs() > half_w {
            vel.0.x = -vel.0.x;
            t.translation.x = t.translation.x.signum() * half_w;
        }
        if t.translation.y.abs() > half_h {
            vel.0.y = -vel.0.y;
            t.translation.y = t.translation.y.signum() * half_h;
        }
    }
}
