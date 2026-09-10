pub mod draw;

use crate::game_core::GameState;

pub struct Renderer {
    pub tile_positions: Vec<TileRenderInfo>,
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
        Self {
            tile_positions: Vec::new(),
        }
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

        let grid_w = (max_x - min_x + 2) as f32;
        let grid_h = (max_y - min_y + 2) as f32;

        let margin = 60.0f32;
        let button_area = 50.0f32;
        let avail_w = screen_w - margin * 2.0;
        let avail_h = screen_h - margin * 2.0 - button_area;

        let tile_base_w = 40.0f32;
        let tile_base_h = 30.0f32;
        let tile_gap_x = 2.0f32;
        let tile_gap_y = 2.0f32;
        let layer_offset_x = 4.0f32;
        let layer_offset_y = 4.0f32;

        let step_x = tile_base_w + tile_gap_x;
        let step_y = tile_base_h + tile_gap_y;

        let total_grid_w = grid_w * step_x + max_z as f32 * layer_offset_x;
        let total_grid_h = grid_h * step_y + max_z as f32 * layer_offset_y;

        let scale_x = avail_w / total_grid_w;
        let scale_y = avail_h / total_grid_h;
        let scale = scale_x.min(scale_y).min(1.5);

        let rendered_w = total_grid_w * scale;
        let rendered_h = total_grid_h * scale;
        let offset_x = (screen_w - rendered_w) / 2.0;
        let offset_y = margin + (avail_h - rendered_h) / 2.0;

        for tile in &active_tiles {
            let sx = offset_x
                + (tile.position.x - min_x) as f32 * step_x * scale
                + tile.position.z as f32 * layer_offset_x * scale;
            let sy = offset_y
                + (tile.position.y - min_y) as f32 * step_y * scale
                + tile.position.z as f32 * layer_offset_y * scale;
            let tw = tile_base_w * scale;
            let th = tile_base_h * scale;

            self.tile_positions.push(TileRenderInfo {
                tile_id: tile.id,
                screen_x: sx,
                screen_y: sy,
                width: tw,
                height: th,
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
