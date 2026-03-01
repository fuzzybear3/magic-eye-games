use bevy::prelude::*;

use crate::plugin::{DepthSprite, ScreenSize};
use crate::AppState;

pub struct TetrisPlugin;

impl Plugin for TetrisPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Tetris), spawn_tetris)
            .add_systems(OnExit(AppState::Tetris), despawn_tetris);
    }
}

// ── Constants ─────────────────────────────────────────────────────────────────

const CELL: f32 = 32.0;
const COLS: i32 = 10;
const ROWS: i32 = 20;

const TEST_PIECE_OFFSETS: [(i32, i32); 4] = [(0, 0), (1, 0), (2, 0), (3, 0)];
const TEST_PIECE_COL: i32 = 3;
const TEST_PIECE_ROW: i32 = ROWS - 2;
const TEST_PIECE_DEPTH: f32 = 1.0;

// ── Components ────────────────────────────────────────────────────────────────

#[derive(Component)]
struct TetrisEntity;

#[derive(Component)]
struct PieceCell;

// ── Helpers ───────────────────────────────────────────────────────────────────

fn board_origin() -> Vec2 {
    Vec2::new(
        -(COLS as f32 * CELL) / 2.0 + CELL / 2.0,
        -(ROWS as f32 * CELL) / 2.0 + CELL / 2.0,
    )
}

fn cell_to_world(col: i32, row: i32) -> Vec2 {
    let o = board_origin();
    Vec2::new(o.x + col as f32 * CELL, o.y + row as f32 * CELL)
}

// ── Systems ───────────────────────────────────────────────────────────────────

fn spawn_tetris(mut commands: Commands, screen: Res<ScreenSize>) {
    // Scene background
    commands.spawn((
        TetrisEntity,
        DepthSprite {
            size: Vec2::new(screen.width as f32, screen.height as f32),
            depth: 0.0,
        },
        Transform::default(),
    ));

    // Board background
    let board_w = COLS as f32 * CELL;
    let board_h = ROWS as f32 * CELL;
    commands.spawn((
        TetrisEntity,
        DepthSprite {
            size: Vec2::new(board_w, board_h),
            depth: 0.1,
        },
        Transform::default(),
    ));

    // Test I-piece
    for (dc, dr) in TEST_PIECE_OFFSETS {
        let pos = cell_to_world(TEST_PIECE_COL + dc, TEST_PIECE_ROW + dr);
        commands.spawn((
            TetrisEntity,
            PieceCell,
            DepthSprite {
                size: Vec2::new(CELL - 2.0, CELL - 2.0),
                depth: TEST_PIECE_DEPTH,
            },
            Transform::from_xyz(pos.x, pos.y, 0.0),
        ));
    }
}

fn despawn_tetris(mut commands: Commands, query: Query<Entity, With<TetrisEntity>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
