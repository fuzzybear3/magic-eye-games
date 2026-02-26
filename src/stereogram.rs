//! Port of the Thimbleby, Inglis, Witten (1994) SIRDS algorithm.
//! Reference: http://www.cs.sfu.ca/CourseCentral/414/li/material/refs/SIRDS-Computer-94.pdf

use rand::Rng;

const COLORS: &[[u8; 4]] = &[
    [220, 50,  50,  255],
    [50,  200, 50,  255],
    [50,  50,  220, 255],
    [220, 200, 50,  255],
    [200, 50,  200, 255],
    [50,  200, 200, 255],
    [220, 130, 50,  255],
    [130, 50,  220, 255],
    [220, 220, 220, 255],
    [60,  60,  60,  255],
];

/// Generate a stereogram image from a depth map.
///
/// - `depth_map`: row-major, width × height floats in [0.0, 1.0]
///   where 0.0 = far background, 1.0 = closest to the viewer
/// - Returns RGBA bytes (width × height × 4)
pub fn generate(depth_map: &[f32], width: usize, height: usize, rng: &mut impl Rng) -> Vec<u8> {
    let dpi = 72.0_f32;
    let eye_sep = (2.5 * dpi).round() as i32; // ~180 px  (assumed 2.5-inch eye separation)
    let mu = 1.0_f32 / 3.0;                   // depth-of-field as fraction of viewing distance

    let mut pixels = vec![0u8; width * height * 4];

    for y in 0..height {
        // same[x] == x  → free pixel (gets a random color)
        // same[x] == r  → constrained to match pixel r (r > x, already filled)
        let mut same: Vec<usize> = (0..width).collect();

        // ── Forward pass: build same[] constraint links ──────────────────────
        for x in 0..width {
            let z = depth_map[y * width + x];

            // Stereo separation in pixels for this depth value
            let sep = ((1.0 - mu * z) * eye_sep as f32 / (2.0 - mu * z)).round() as i32;

            // Pixel columns seen by each eye
            let left_i  = x as i32 - (sep + (sep & (y as i32) & 1)) / 2;
            let right_i = left_i + sep;

            if left_i < 0 || right_i >= width as i32 {
                continue;
            }

            let mut left  = left_i  as usize;
            let mut right = right_i as usize;

            // Hidden-surface removal: walk outward until we find an occluder
            // or exhaust the depth range (zt reaches 1.0).
            let visible = 'check: {
                let mut t = 1i32;
                loop {
                    let zt = z + 2.0 * (2.0 - mu * z) * t as f32 / (mu * eye_sep as f32);

                    let xl = x as i32 - t;
                    let xr = x as i32 + t;

                    // Out-of-bounds treated as occluded (matches JS behaviour)
                    let lc = xl >= 0           && depth_map[y * width + xl as usize] < zt;
                    let rc = xr < width as i32 && depth_map[y * width + xr as usize] < zt;

                    let v = lc && rc;
                    t += 1;

                    if !v || zt >= 1.0 {
                        break 'check v;
                    }
                }
            };

            if visible {
                // Merge constraint chains while keeping left < right
                let mut k = same[left];
                while k != left && k != right {
                    if k < right {
                        left = k;
                    } else {
                        left = right;
                        right = k;
                    }
                    k = same[left];
                }
                same[left] = right;
            }
        }

        // ── Backward pass: assign colors right → left ────────────────────────
        for x in (0..width).rev() {
            let dst = (y * width + x) * 4;
            if same[x] == x {
                // Free pixel → pick a random color from the palette
                let c = COLORS[rng.gen_range(0..COLORS.len())];
                pixels[dst..dst + 4].copy_from_slice(&c);
            } else {
                // Constrained → copy color from same[x] (already processed, same[x] > x)
                let src = (y * width + same[x]) * 4;
                pixels.copy_within(src..src + 4, dst);
            }
        }
    }

    pixels
}
