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
    pub z: i32,
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

        let margin = 26.0f32;
        let bottom = 44.0f32;
        let avail_w = (screen_w - margin * 2.0).max(100.0);
        let avail_h = (screen_h - margin * 2.0 - bottom).max(100.0);

        // Tiles keep a 4:3 aspect ratio; stacked layers shift by a fraction of a tile.
        let layer_off_w = 0.10f32;
        let layer_off_h = 0.10f32;

        // A "unit" u is the column pitch; row pitch = u * 0.75.
        let u_w = avail_w / (n_cols + layer_off_w * max_z);
        let u_h = avail_h / ((n_rows + layer_off_h * max_z) * 0.75);
        let u = u_w.min(u_h).clamp(14.0, 96.0);

        let col_pitch = u;
        let row_pitch = u * 0.75;
        let gap_x = (u * 0.04).clamp(1.0, 4.0);
        let gap_y = (row_pitch * 0.04).clamp(1.0, 3.0);

        let tile_w = col_pitch - gap_x;
        let tile_h = row_pitch - gap_y;
        let off_x = col_pitch * layer_off_w;
        let off_y = row_pitch * layer_off_h;

        let total_w = n_cols * col_pitch + max_z * off_x;
        let total_h = n_rows * row_pitch + max_z * off_y;
        let offset_x = (screen_w - total_w) / 2.0;
        let offset_y = margin + (avail_h - total_h) / 2.0;

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
                z: tile.position.z,
            });
        }

        self.tile_positions
            .sort_by(|a, b| a.z.cmp(&b.z).then(a.tile_id.cmp(&b.tile_id)));
    }

    pub fn get_tile_at(&self, x: f32, y: f32) -> Option<usize> {
        for info in self.tile_positions.iter().rev() {
            if x >= info.screen_x
                && x <= info.screen_x + info.width
                && y >= info.screen_y
                && y <= info.screen_y + info.height
            {
                return Some(info.tile_id);
            }
        }
        None
    }
}
