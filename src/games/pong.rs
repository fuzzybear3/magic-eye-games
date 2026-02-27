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

const PADDLE_W: f32 = 16.0;
const PADDLE_H: f32 = 100.0;
const BALL_SIZE: f32 = 16.0;
const PADDLE_SPEED: f32 = 300.0;
const BALL_SPEED: f32 = 300.0;
const PADDLE_X: f32 = 580.0;

// ── Components ────────────────────────────────────────────────────────────────

#[derive(Component)]
struct PongEntity;

#[derive(Component)]
struct LeftPaddle;

#[derive(Component)]
struct RightPaddle;

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

    // Left paddle
    commands.spawn((
        PongEntity,
        LeftPaddle,
        DepthSprite {
            size: Vec2::new(PADDLE_W, PADDLE_H),
            depth: 0.8,
        },
        Transform::from_xyz(-PADDLE_X, 0.0, 0.0),
    ));

    // Right paddle
    commands.spawn((
        PongEntity,
        RightPaddle,
        DepthSprite {
            size: Vec2::new(PADDLE_W, PADDLE_H),
            depth: 0.8,
        },
        Transform::from_xyz(PADDLE_X, 0.0, 0.0),
    ));

    // Ball
    commands.spawn((
        PongEntity,
        Ball,
        Velocity(Vec2::new(BALL_SPEED, BALL_SPEED * 0.6)),
        DepthSprite {
            size: Vec2::new(BALL_SIZE, BALL_SIZE),
            depth: 1.0,
        },
        Transform::default(),
    ));
}

fn move_paddles(
    keys: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut left: Query<&mut Transform, (With<LeftPaddle>, Without<RightPaddle>)>,
    mut right: Query<&mut Transform, (With<RightPaddle>, Without<LeftPaddle>)>,
) {
    let dt = time.delta_secs();
    let max_y = HEIGHT as f32 / 2.0 - PADDLE_H / 2.0;

    if let Ok(mut t) = left.single_mut() {
        if keys.pressed(KeyCode::KeyW) {
            t.translation.y = (t.translation.y + PADDLE_SPEED * dt).min(max_y);
        }
        if keys.pressed(KeyCode::KeyS) {
            t.translation.y = (t.translation.y - PADDLE_SPEED * dt).max(-max_y);
        }
    }

    if let Ok(mut t) = right.single_mut() {
        if keys.pressed(KeyCode::ArrowUp) {
            t.translation.y = (t.translation.y + PADDLE_SPEED * dt).min(max_y);
        }
        if keys.pressed(KeyCode::ArrowDown) {
            t.translation.y = (t.translation.y - PADDLE_SPEED * dt).max(-max_y);
        }
    }
}

fn update_ball(
    time: Res<Time>,
    mut ball_q: Query<(&mut Transform, &mut Velocity), With<Ball>>,
    left_q: Query<&Transform, (With<LeftPaddle>, Without<Ball>)>,
    right_q: Query<&Transform, (With<RightPaddle>, Without<Ball>)>,
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

    // Top / bottom wall bounce
    if bt.translation.y + ball_half > half_h {
        bt.translation.y = half_h - ball_half;
        vel.0.y = -vel.0.y.abs();
    } else if bt.translation.y - ball_half < -half_h {
        bt.translation.y = -half_h + ball_half;
        vel.0.y = vel.0.y.abs();
    }

    // Left paddle bounce
    if let Ok(lt) = left_q.single() {
        let lx = lt.translation.x;
        let ly = lt.translation.y;
        if vel.0.x < 0.0
            && bt.translation.x - ball_half < lx + paddle_half_w
            && bt.translation.x + ball_half > lx - paddle_half_w
            && bt.translation.y < ly + paddle_half_h
            && bt.translation.y > ly - paddle_half_h
        {
            bt.translation.x = lx + paddle_half_w + ball_half;
            vel.0.x = vel.0.x.abs();
        }
    }

    // Right paddle bounce
    if let Ok(rt) = right_q.single() {
        let rx = rt.translation.x;
        let ry = rt.translation.y;
        if vel.0.x > 0.0
            && bt.translation.x + ball_half > rx - paddle_half_w
            && bt.translation.x - ball_half < rx + paddle_half_w
            && bt.translation.y < ry + paddle_half_h
            && bt.translation.y > ry - paddle_half_h
        {
            bt.translation.x = rx - paddle_half_w - ball_half;
            vel.0.x = -vel.0.x.abs();
        }
    }

    // Score: ball exits left or right — reset to centre
    if bt.translation.x.abs() > half_w + ball_half {
        let dir = if bt.translation.x > 0.0 { -1.0_f32 } else { 1.0_f32 };
        bt.translation = Vec3::ZERO;
        vel.0 = Vec2::new(BALL_SPEED * dir, BALL_SPEED * 0.6);
    }
}

fn despawn_pong(mut commands: Commands, query: Query<Entity, With<PongEntity>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
