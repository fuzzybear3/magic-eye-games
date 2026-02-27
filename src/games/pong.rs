use bevy::prelude::*;

use crate::plugin::DepthSprite;
use crate::{AppState, HEIGHT, WIDTH};

pub struct PongPlugin;

impl Plugin for PongPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Pong), spawn_pong)
            .add_systems(
                Update,
                (move_paddles, update_ball).run_if(in_state(AppState::Pong)),
            )
            .add_systems(OnExit(AppState::Pong), despawn_pong);
    }
}

// ── Constants ─────────────────────────────────────────────────────────────────

const PADDLE_W: f32 = 160.0; // wide horizontal bar
const PADDLE_H: f32 = 32.0;
const BALL_SIZE: f32 = 40.0;
const PADDLE_SPEED: f32 = 400.0;
const BALL_SPEED: f32 = 300.0;
const PADDLE_Y: f32 = 350.0; // distance from centre to each paddle

// ── Components ────────────────────────────────────────────────────────────────

#[derive(Component)]
struct PongEntity;

#[derive(Component)]
struct TopPaddle;

#[derive(Component)]
struct BottomPaddle;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Velocity(Vec2);

// ── Systems ───────────────────────────────────────────────────────────────────

fn spawn_pong(mut commands: Commands) {
    // Scene background
    commands.spawn((
        PongEntity,
        DepthSprite {
            size: Vec2::new(WIDTH as f32, HEIGHT as f32),
            depth: 0.0,
        },
        Transform::default(),
    ));

    // Top paddle
    commands.spawn((
        PongEntity,
        TopPaddle,
        DepthSprite {
            size: Vec2::new(PADDLE_W, PADDLE_H),
            depth: 0.8,
        },
        Transform::from_xyz(0.0, PADDLE_Y, 0.0),
    ));

    // Bottom paddle
    commands.spawn((
        PongEntity,
        BottomPaddle,
        DepthSprite {
            size: Vec2::new(PADDLE_W, PADDLE_H),
            depth: 0.8,
        },
        Transform::from_xyz(0.0, -PADDLE_Y, 0.0),
    ));

    // Ball
    commands.spawn((
        PongEntity,
        Ball,
        Velocity(Vec2::new(BALL_SPEED * 0.6, BALL_SPEED)),
        DepthSprite {
            size: Vec2::new(BALL_SIZE, BALL_SIZE),
            depth: 1.0,
        },
        Transform::default(),
    ));
}

fn move_paddles(
    touches: Res<Touches>,
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut top: Query<&mut Transform, (With<TopPaddle>, Without<BottomPaddle>)>,
    mut bottom: Query<&mut Transform, (With<BottomPaddle>, Without<TopPaddle>)>,
) {
    let dt = time.delta_secs();
    let max_x = WIDTH as f32 / 2.0 - PADDLE_W / 2.0;

    // Keyboard fallback (A/D = bottom, ←/→ = top)
    if let Ok(mut t) = bottom.single_mut() {
        if keys.pressed(KeyCode::KeyA) {
            t.translation.x = (t.translation.x - PADDLE_SPEED * dt).max(-max_x);
        }
        if keys.pressed(KeyCode::KeyD) {
            t.translation.x = (t.translation.x + PADDLE_SPEED * dt).min(max_x);
        }
    }
    if let Ok(mut t) = top.single_mut() {
        if keys.pressed(KeyCode::ArrowLeft) {
            t.translation.x = (t.translation.x - PADDLE_SPEED * dt).max(-max_x);
        }
        if keys.pressed(KeyCode::ArrowRight) {
            t.translation.x = (t.translation.x + PADDLE_SPEED * dt).min(max_x);
        }
    }

    // Touch: finger X sets paddle X directly.
    // Top half of screen → top paddle; bottom half → bottom paddle.
    let half_w = WIDTH as f32 / 2.0;
    let half_h = HEIGHT as f32 / 2.0;
    for touch in touches.iter() {
        let pos = touch.position();
        let world_x = (pos.x - half_w).clamp(-max_x, max_x);
        if pos.y < half_h {
            if let Ok(mut t) = top.single_mut() {
                t.translation.x = world_x;
            }
        } else if let Ok(mut t) = bottom.single_mut() {
            t.translation.x = world_x;
        }
    }
}

fn update_ball(
    time: Res<Time>,
    mut ball_q: Query<(&mut Transform, &mut Velocity), With<Ball>>,
    top_q: Query<&Transform, (With<TopPaddle>, Without<Ball>)>,
    bottom_q: Query<&Transform, (With<BottomPaddle>, Without<Ball>)>,
) {
    let dt = time.delta_secs();
    let half_w = WIDTH as f32 / 2.0;
    let half_h = HEIGHT as f32 / 2.0;
    let ball_half = BALL_SIZE / 2.0;
    let paddle_half_w = PADDLE_W / 2.0;
    let paddle_half_h = PADDLE_H / 2.0;

    let Ok((mut bt, mut vel)) = ball_q.single_mut() else {
        return;
    };

    // Move
    bt.translation.x += vel.0.x * dt;
    bt.translation.y += vel.0.y * dt;

    // Left / right wall bounce
    if bt.translation.x + ball_half > half_w {
        bt.translation.x = half_w - ball_half;
        vel.0.x = -vel.0.x.abs();
    } else if bt.translation.x - ball_half < -half_w {
        bt.translation.x = -half_w + ball_half;
        vel.0.x = vel.0.x.abs();
    }

    // Top paddle bounce
    if let Ok(tt) = top_q.single() {
        let tx = tt.translation.x;
        let ty = tt.translation.y;
        if vel.0.y > 0.0
            && bt.translation.y + ball_half > ty - paddle_half_h
            && bt.translation.y - ball_half < ty + paddle_half_h
            && bt.translation.x < tx + paddle_half_w
            && bt.translation.x > tx - paddle_half_w
        {
            bt.translation.y = ty - paddle_half_h - ball_half;
            vel.0.y = -vel.0.y.abs();
        }
    }

    // Bottom paddle bounce
    if let Ok(bt2) = bottom_q.single() {
        let bx = bt2.translation.x;
        let by = bt2.translation.y;
        if vel.0.y < 0.0
            && bt.translation.y - ball_half < by + paddle_half_h
            && bt.translation.y + ball_half > by - paddle_half_h
            && bt.translation.x < bx + paddle_half_w
            && bt.translation.x > bx - paddle_half_w
        {
            bt.translation.y = by + paddle_half_h + ball_half;
            vel.0.y = vel.0.y.abs();
        }
    }

    // Score: ball exits top or bottom — reset to centre
    if bt.translation.y.abs() > half_h + ball_half {
        let dir = if bt.translation.y > 0.0 {
            -1.0_f32
        } else {
            1.0_f32
        };
        bt.translation = Vec3::ZERO;
        vel.0 = Vec2::new(BALL_SPEED * 0.6, BALL_SPEED * dir);
    }
}

fn despawn_pong(mut commands: Commands, query: Query<Entity, With<PongEntity>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
