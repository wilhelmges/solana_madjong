use serde::Deserialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub struct TilePosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct TileData {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    #[serde(rename = "type_id")]
    pub type_id: u32,
}

#[derive(Debug, Clone)]
pub struct Tile {
    pub id: usize,
    pub position: TilePosition,
    pub type_id: u32,
    pub active: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LevelData {
    pub tiles: Vec<TileData>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GamePhase {
    Playing,
    LevelCompleted,
    AllLevelsCompleted,
    Lost,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BufferAction {
    AddedToBuffer(usize),
    Paired(usize, usize, usize),
}

pub const BUFFER_SIZE: usize = 4;

pub struct GameState {
    pub current_level: usize,
    pub tiles: Vec<Tile>,
    pub buffer: Vec<usize>,
    pub phase: GamePhase,
    pub score: u32,
    pub history: Vec<BufferAction>,
    pub level_start_sec: f64,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            current_level: 1,
            tiles: Vec::new(),
            buffer: Vec::new(),
            phase: GamePhase::Playing,
            score: 0,
            history: Vec::new(),
            level_start_sec: 0.0,
        }
    }

    pub fn load_level(&mut self, level: usize, level_data: &LevelData) {
        self.current_level = level;
        self.tiles = level_data
            .tiles
            .iter()
            .enumerate()
            .map(|(i, td)| Tile {
                id: i,
                position: TilePosition {
                    x: td.x,
                    y: td.y,
                    z: td.z,
                },
                type_id: td.type_id,
                active: true,
            })
            .collect();
        self.buffer.clear();
        self.phase = GamePhase::Playing;
        self.score = 0;
        self.history.clear();
        self.level_start_sec = super::game_time_now();
    }

    pub fn active_tile_count(&self) -> usize {
        self.tiles.iter().filter(|t| t.active).count()
    }

    pub fn get_tile(&self, id: usize) -> Option<&Tile> {
        self.tiles.get(id)
    }

    pub fn get_active_tile_at(&self, pos: TilePosition) -> Option<&Tile> {
        self.tiles
            .iter()
            .find(|t| t.active && t.position == pos)
    }
}
