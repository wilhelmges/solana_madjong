pub mod draw;
pub mod icons;
pub mod tiles;

use crate::game_core::GameState;

pub struct Renderer {
    pub tile_positions: Vec<TileRenderInfo>,
    pub tile_textures: tiles::TileTextures,
}

#[derive(Clone)]
pub struct TileRenderInfo {
    pub tile_id: usize,
    pub screen_x: f32,
    pub screen_y: f32,
    pub width: f32,
    pub height: f32,
    pub thickness: f32,
    pub z: i32,
    pub row: i32,
    pub col: i32,
}

impl Renderer {
    pub fn new() -> Self {
        Self::new_empty()
    }

    pub fn new_empty() -> Self {
        Self {
            tile_positions: Vec::new(),
            tile_textures: tiles::TileTextures::new_empty(),
        }
    }

    pub async fn load_tile_textures(&mut self) {
        self.tile_textures = tiles::TileTextures::load().await;
    }

    pub fn compute_layout(
        &mut self,
        state: &GameState,
        screen_w: f32,
        screen_h: f32,
    ) {
        self.tile_positions.clear();

        let active_tiles: Vec<_> = state.tiles.iter().filter(|t| t.active).collect();
        if active_tiles.is_empty() {
            return;
        }

        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;
        let mut min_y = i32::MAX;
        let mut max_y = i32::MIN;
        let mut max_z = 0i32;

        for tile in &active_tiles {
            min_x = min_x.min(tile.position.x);
            max_x = max_x.max(tile.position.x);
            min_y = min_y.min(tile.position.y);
            max_y = max_y.max(tile.position.y);
            max_z = max_z.max(tile.position.z);
        }

        // Consecutive tiles are 2 quarter-tile units apart (KMahjongg grid).
        let n_cols = ((max_x - min_x) / 2 + 1).max(1) as f32;
        let n_rows = ((max_y - min_y) / 2 + 1).max(1) as f32;
        let max_z = max_z as f32;

        let margin = 16.0f32;
        let top_bar = 76.0f32;
        let bottom_bar = 78.0f32;
        let avail_w = (screen_w - margin * 2.0).max(100.0);
        let avail_h = (screen_h - top_bar - bottom_bar).max(100.0);

        // Tiles keep a ~1:1.25 portrait-ish face like the reference; stacked
        // layers shift by roughly one thickness to reveal the 3D sides.
        // A "unit" u is the column pitch; row pitch is slightly shorter so the
        // pyramid looks dense.
        let u_w = avail_w / (n_cols + 0.35 * max_z);
        let u_h = avail_h / ((n_rows + 0.35 * max_z) * 0.86);
        let u = u_w.min(u_h).clamp(14.0, 96.0);

        let col_pitch = u;
        let row_pitch = u * 0.86;
        let gap_x = (u * 0.05).clamp(1.0, 5.0);
        let gap_y = (row_pitch * 0.05).clamp(1.0, 4.0);

        let tile_w = col_pitch - gap_x;
        let tile_h = row_pitch - gap_y;
        // Procedural 3D depth, drawn by code (see draw::draw_tile).
        let thickness = (u * 0.14).clamp(5.0, 11.0);
        let off_x = thickness * 0.85;
        let off_y = thickness * 0.85;

        let total_w = n_cols * col_pitch + max_z * off_x + thickness;
        let total_h = n_rows * row_pitch + max_z * off_y + thickness;
        let offset_x = (screen_w - total_w) / 2.0;
        let offset_y = top_bar + (avail_h - total_h) / 2.0;

        for tile in &active_tiles {
            let col = (tile.position.x - min_x) / 2;
            let row = (tile.position.y - min_y) / 2;
            let sx = offset_x + col as f32 * col_pitch + tile.position.z as f32 * off_x;
            let sy = offset_y + row as f32 * row_pitch + tile.position.z as f32 * off_y;

            self.tile_positions.push(TileRenderInfo {
                tile_id: tile.id,
                screen_x: sx,
                screen_y: sy,
                width: tile_w,
                height: tile_h,
                thickness,
                z: tile.position.z,
                row,
                col,
            });
        }

        // Back-to-front: lower layers first, then top rows, then left-to-right.
        self.tile_positions.sort_by(|a, b| {
            a.z.cmp(&b.z)
                .then(a.row.cmp(&b.row))
                .then(a.col.cmp(&b.col))
                .then(a.tile_id.cmp(&b.tile_id))
        });
    }

    pub fn get_tile_at(&self, x: f32, y: f32) -> Option<usize> {
        // Include the procedural side overhang so clicks on the 3D edge still hit.
        for info in self.tile_positions.iter().rev() {
            if x >= info.screen_x
                && x <= info.screen_x + info.width + info.thickness
                && y >= info.screen_y
                && y <= info.screen_y + info.height + info.thickness
            {
                return Some(info.tile_id);
            }
        }
        None
    }
}
