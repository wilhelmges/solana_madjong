pub mod solana;

pub use solana::*;

pub struct Theme {
    pub name: &'static str,
    pub tile_types: Vec<TileType>,
}

pub struct TileType {
    pub id: u32,
    pub name: &'static str,
    pub category: &'static str,
    pub color: (u8, u8, u8),
}

pub fn get_theme() -> Theme {
    solana_theme()
}
