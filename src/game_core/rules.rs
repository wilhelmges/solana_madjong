use super::state::{GameState, Tile};

pub fn is_selectable(state: &GameState, tile: &Tile) -> bool {
    if !tile.active {
        return false;
    }

    if is_blocked_from_above(state, tile) {
        return false;
    }

    if !has_free_side(state, tile) {
        return false;
    }

    true
}

fn is_blocked_from_above(state: &GameState, tile: &Tile) -> bool {
    let pos = tile.position;
    state.tiles.iter().any(|t| {
        t.active
            && t.id != tile.id
            && t.position.x == pos.x
            && t.position.y == pos.y
            && t.position.z > pos.z
    })
}

fn has_free_side(state: &GameState, tile: &Tile) -> bool {
    let pos = tile.position;
    let left_free = !state.tiles.iter().any(|t| {
        t.active
            && t.id != tile.id
            && t.position.z == pos.z
            && t.position.y == pos.y
            && t.position.x == pos.x - 2
    });
    let right_free = !state.tiles.iter().any(|t| {
        t.active
            && t.id != tile.id
            && t.position.z == pos.z
            && t.position.y == pos.y
            && t.position.x == pos.x + 2
    });
    let top_free = !state.tiles.iter().any(|t| {
        t.active
            && t.id != tile.id
            && t.position.z == pos.z
            && t.position.x == pos.x
            && t.position.y == pos.y - 2
    });
    let bottom_free = !state.tiles.iter().any(|t| {
        t.active
            && t.id != tile.id
            && t.position.z == pos.z
            && t.position.x == pos.x
            && t.position.y == pos.y + 2
    });

    left_free || right_free || top_free || bottom_free
}

pub fn tiles_match(tile_a: &Tile, tile_b: &Tile) -> bool {
    tile_a.type_id == tile_b.type_id
}

pub fn try_select_tile(state: &mut GameState, tile_id: usize) -> Option<(usize, usize)> {
    let tile = match state.tiles.get(tile_id) {
        Some(t) if t.active => t.clone(),
        _ => return None,
    };

    if !is_selectable(state, &tile) {
        return None;
    }

    match state.selected_tile_id {
        None => {
            state.selected_tile_id = Some(tile_id);
            None
        }
        Some(prev_id) => {
            if prev_id == tile_id {
                state.selected_tile_id = None;
                return None;
            }

            let prev_tile = match state.tiles.get(prev_id) {
                Some(t) if t.active => t.clone(),
                _ => {
                    state.selected_tile_id = Some(tile_id);
                    return None;
                }
            };

            if !is_selectable(state, &prev_tile) {
                state.selected_tile_id = Some(tile_id);
                return None;
            }

            if tiles_match(&prev_tile, &tile) {
                state.tiles[prev_id].active = false;
                state.tiles[tile_id].active = false;
                state.selected_tile_id = None;

                if state.active_tile_count() == 0 {
                    state.phase = super::state::GamePhase::LevelCompleted;
                }

                Some((prev_id, tile_id))
            } else {
                state.selected_tile_id = Some(tile_id);
                None
            }
        }
    }
}

pub fn has_any_moves(state: &GameState) -> bool {
    let active: Vec<&Tile> = state.tiles.iter().filter(|t| t.active).collect();
    for i in 0..active.len() {
        if !is_selectable(state, active[i]) {
            continue;
        }
        for j in (i + 1)..active.len() {
            if !is_selectable(state, active[j]) {
                continue;
            }
            if tiles_match(active[i], active[j]) {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game_core::{GameState, LevelData, Tile, TileData, TilePosition};

    fn make_tile(id: usize, x: i32, y: i32, z: i32, type_id: u32) -> Tile {
        Tile {
            id,
            position: TilePosition { x, y, z },
            type_id,
            active: true,
        }
    }

    fn make_level(tiles: Vec<Tile>) -> LevelData {
        let tiles_data = tiles
            .iter()
            .map(|t| TileData {
                x: t.position.x,
                y: t.position.y,
                z: t.position.z,
                type_id: t.type_id,
            })
            .collect();
        LevelData { tiles: tiles_data }
    }

    #[test]
    fn test_side_selection() {
        let tiles = vec![
            make_tile(0, 3, 0, 0, 0),
            make_tile(1, 5, 0, 0, 1),
            make_tile(2, 7, 0, 0, 0),
            make_tile(3, 9, 0, 0, 1),
        ];
        let mut state = GameState::new();
        state.load_level(1, &make_level(tiles));

        assert!(is_selectable(&state, &state.tiles[0]));
        assert!(is_selectable(&state, &state.tiles[1]));

        let removed = try_select_tile(&mut state, 0);
        assert!(removed.is_none());
        assert_eq!(state.selected_tile_id, Some(0));

        let removed = try_select_tile(&mut state, 2);
        assert!(removed.is_some());
        assert_eq!(state.selected_tile_id, None);
        assert!(!state.tiles[0].active);
        assert!(!state.tiles[2].active);
    }

    #[test]
    fn test_blocked_from_above() {
        let tiles = vec![
            make_tile(0, 3, 0, 0, 0),
            make_tile(1, 3, 0, 1, 5),
            make_tile(2, 3, 0, 2, 0),
        ];
        let mut state = GameState::new();
        state.load_level(1, &make_level(tiles));

        assert!(!is_selectable(&state, &state.tiles[0]));
        assert!(!is_selectable(&state, &state.tiles[1]));
        assert!(is_selectable(&state, &state.tiles[2]));
    }

    #[test]
    fn test_blocked_by_neighbors() {
        let tiles = vec![
            make_tile(0, 3, 0, 0, 0),
            make_tile(1, 5, 0, 0, 1),
            make_tile(2, 7, 0, 0, 2),
            make_tile(3, 5, 2, 0, 3),
            make_tile(4, 5, -2, 0, 4),
        ];
        let mut state = GameState::new();
        state.load_level(1, &make_level(tiles));

        assert!(is_selectable(&state, &state.tiles[0]));
        assert!(!is_selectable(&state, &state.tiles[1]));
        assert!(is_selectable(&state, &state.tiles[2]));
        assert!(is_selectable(&state, &state.tiles[3]));
        assert!(is_selectable(&state, &state.tiles[4]));
    }

    #[test]
    fn test_match_requires_same_type() {
        let tiles = vec![
            make_tile(0, 3, 0, 0, 0),
            make_tile(1, 7, 0, 0, 1),
        ];
        let mut state = GameState::new();
        state.load_level(1, &make_level(tiles));

        try_select_tile(&mut state, 0);
        let removed = try_select_tile(&mut state, 1);
        assert!(removed.is_none());
        assert!(state.tiles[0].active);
        assert!(state.tiles[1].active);
        assert_eq!(state.selected_tile_id, Some(1));
    }

    #[test]
    fn test_deselect_on_second_click() {
        let tiles = vec![make_tile(0, 3, 0, 0, 0)];
        let mut state = GameState::new();
        state.load_level(1, &make_level(tiles));

        try_select_tile(&mut state, 0);
        assert_eq!(state.selected_tile_id, Some(0));
        try_select_tile(&mut state, 0);
        assert_eq!(state.selected_tile_id, None);
    }

    #[test]
    fn test_level_completion() {
        let tiles = vec![
            make_tile(0, 3, 0, 0, 7),
            make_tile(1, 7, 0, 0, 7),
        ];
        let mut state = GameState::new();
        state.load_level(1, &make_level(tiles));

        try_select_tile(&mut state, 0);
        let removed = try_select_tile(&mut state, 1);
        assert!(removed.is_some());
        assert_eq!(state.active_tile_count(), 0);
        assert_eq!(state.phase, super::super::state::GamePhase::LevelCompleted);
    }
}
