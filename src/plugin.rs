//! Bevy plugin that replaces normal rendering with a live Magic Eye stereogram.
//!
//! Pipeline each frame (runs in PostUpdate, after game logic):
//!   1. `rasterize_depth_buffer` – paint every `DepthSprite` entity into a CPU f32 buffer
//!   2. `generate_stereogram`    – run the SIRDS algorithm, upload result as a texture

use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
};
use rand::{rngs::SmallRng, SeedableRng};

use crate::stereogram;

// ── Public component ─────────────────────────────────────────────────────────

/// Add to any entity to make it visible in the stereogram.
///
/// - `size`:  bounding rectangle in world pixels
/// - `depth`: 0.0 = farthest background, 1.0 = closest to viewer
#[derive(Component)]
pub struct DepthSprite {
    pub size: Vec2,
    pub depth: f32,
}

// ── Internal resources ────────────────────────────────────────────────────────

#[derive(Resource)]
pub struct DepthBuffer {
    pub data: Vec<f32>,
    pub width: usize,
    pub height: usize,
}

#[derive(Resource)]
struct StereogramOutput(Handle<Image>);

#[derive(Resource)]
struct StereogramRng(SmallRng);

#[derive(Resource, Clone, Copy)]
struct ScreenSize {
    width: u32,
    height: u32,
}

// ── Plugin ────────────────────────────────────────────────────────────────────

pub struct MagicEyePlugin {
    pub width: u32,
    pub height: u32,
}

impl Plugin for MagicEyePlugin {
    fn build(&self, app: &mut App) {
        let (w, h) = (self.width, self.height);
        app.insert_resource(DepthBuffer {
                data: vec![0.0; (w * h) as usize],
                width: w as usize,
                height: h as usize,
            })
            .insert_resource(ScreenSize { width: w, height: h })
            .insert_resource(StereogramRng(SmallRng::seed_from_u64(12345)))
            .add_systems(Startup, setup_output_sprite)
            .add_systems(
                PostUpdate,
                (rasterize_depth_buffer, generate_stereogram.after(rasterize_depth_buffer)),
            );
    }
}

// ── Systems ───────────────────────────────────────────────────────────────────

fn setup_output_sprite(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    size: Res<ScreenSize>,
) {
    let (w, h) = (size.width, size.height);

    let mut image = Image::new_fill(
        Extent3d { width: w, height: h, depth_or_array_layers: 1 },
        TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    // COPY_DST lets us push new pixel data from the CPU every frame.
    image.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST;

    let handle = images.add(image);
    commands.insert_resource(StereogramOutput(handle.clone()));

    // Single fullscreen sprite – this is the only thing the camera sees.
    commands.spawn((
        Sprite {
            image: handle,
            custom_size: Some(Vec2::new(w as f32, h as f32)),
            ..default()
        },
        Transform::default(),
    ));
}

/// Rasterize all `DepthSprite` entities into the CPU depth buffer.
/// Lower-depth entities are painted first so higher-depth ones overwrite them.
pub fn rasterize_depth_buffer(
    mut buf: ResMut<DepthBuffer>,
    query: Query<(&Transform, &DepthSprite)>,
) {
    let (w, h) = (buf.width, buf.height);
    let (wf, hf) = (w as f32, h as f32);

    buf.data.fill(0.0);

    // Painter's algorithm: sort by depth ascending so deeper (closer) sprites win.
    let mut entities: Vec<_> = query.iter().collect();
    entities.sort_by(|a, b| {
        a.1.depth.partial_cmp(&b.1.depth).unwrap_or(std::cmp::Ordering::Equal)
    });

    for (transform, sprite) in entities {
        let pos  = transform.translation.truncate();
        let half = sprite.size * 0.5;

        // Bevy world: origin at centre, Y-up
        // Pixel space: origin at top-left, Y-down
        let px0 = ((pos.x - half.x + wf * 0.5) as i32).clamp(0, w as i32) as usize;
        let px1 = ((pos.x + half.x + wf * 0.5) as i32).clamp(0, w as i32) as usize;
        let py0 = ((-pos.y - half.y + hf * 0.5) as i32).clamp(0, h as i32) as usize;
        let py1 = ((-pos.y + half.y + hf * 0.5) as i32).clamp(0, h as i32) as usize;

        for py in py0..py1 {
            for px in px0..px1 {
                buf.data[py * w + px] = sprite.depth;
            }
        }
    }
}

/// Run the stereogram algorithm and upload the result to the GPU texture.
fn generate_stereogram(
    buf: Res<DepthBuffer>,
    output: Res<StereogramOutput>,
    mut images: ResMut<Assets<Image>>,
    mut rng: ResMut<StereogramRng>,
) {
    let Some(image) = images.get_mut(&output.0) else { return };

    image.data = Some(stereogram::generate(&buf.data, buf.width, buf.height, &mut rng.0));
}
