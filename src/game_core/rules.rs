use super::state::{BufferAction, GameState, GamePhase, BUFFER_SIZE, Tile};

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

pub fn buffer_click(state: &mut GameState, tile_id: usize) -> bool {
    // Returns true if the click resulted in a pairing (tiles removed).
    let tile = match state.tiles.get(tile_id) {
        Some(t) if t.active => t.clone(),
        _ => return false,
    };

    if !is_selectable(state, &tile) {
        return false;
    }

    // Look for a matching tile already in the buffer.
    if let Some(pos) = state
        .buffer
        .iter()
        .position(|&bid| state.tiles.get(bid).map(|t| t.type_id) == Some(tile.type_id))
    {
        let match_id = state.buffer.remove(pos);
        state.tiles[tile_id].active = false;
        state.tiles[match_id].active = false;
        state.history.push(BufferAction::Paired(tile_id, match_id, pos));
        state.score = state.score.saturating_add(100);

        if state.active_tile_count() == 0 {
            state.phase = GamePhase::LevelCompleted;
        }
        true
    } else {
        if state.buffer.len() >= BUFFER_SIZE {
            state.phase = GamePhase::Lost;
            return false;
        }
        // The tile leaves the field and moves into the buffer.
        state.tiles[tile_id].active = false;
        state.buffer.push(tile_id);
        state.history.push(BufferAction::AddedToBuffer(tile_id));
        false
    }
}

pub fn do_undo(state: &mut GameState) -> bool {
    let action = match state.history.pop() {
        Some(a) => a,
        None => return false,
    };

    match action {
        BufferAction::AddedToBuffer(tile_id) => {
            state.buffer.retain(|&id| id != tile_id);
            if let Some(t) = state.tiles.get_mut(tile_id) {
                t.active = true;
            }
        }
        BufferAction::Paired(tile_id, match_id, pos) => {
            if let Some(t) = state.tiles.get_mut(tile_id) {
                t.active = true;
            }
            if let Some(t) = state.tiles.get_mut(match_id) {
                t.active = true;
            }
            // Re-insert the buffer tile at its original position.
            let insert_at = pos.min(state.buffer.len());
            state.buffer.insert(insert_at, match_id);
            state.score = state.score.saturating_sub(100);
        }
    }

    if state.phase == GamePhase::Lost || state.phase == GamePhase::LevelCompleted {
        state.phase = GamePhase::Playing;
    }
    true
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
    fn test_buffer_side_selection() {
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

        // Add tile 0 to buffer (no match in buffer yet).
        let paired = buffer_click(&mut state, 0);
        assert!(!paired);
        assert_eq!(state.buffer, vec![0]);
        assert!(!state.tiles[0].active); // moved off the field into the buffer

        // Add tile 2 (same type as 0) — pair forms automatically.
        let paired = buffer_click(&mut state, 2);
        assert!(paired);
        assert!(state.buffer.is_empty());
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
        // Clicking a blocked tile does nothing.
        assert!(!buffer_click(&mut state, 0));
        assert!(state.buffer.is_empty());
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
    fn test_buffer_full_game_over() {
        // 5 distinct types, all free on the edges.
        let tiles = vec![
            make_tile(0, 3, 0, 0, 0),
            make_tile(1, 5, 0, 0, 1),
            make_tile(2, 7, 0, 0, 2),
            make_tile(3, 9, 0, 0, 3),
            make_tile(4, 11, 0, 0, 4),
        ];
        let mut state = GameState::new();
        state.load_level(1, &make_level(tiles));

        for tid in 0..4 {
            let paired = buffer_click(&mut state, tid);
            assert!(!paired);
            assert_eq!(state.phase, GamePhase::Playing);
        }
        assert_eq!(state.buffer.len(), 4);
        assert_eq!(state.buffer, vec![0, 1, 2, 3]);

        // Fifth distinct tile → Game Over.
        let paired = buffer_click(&mut state, 4);
        assert!(!paired);
        assert_eq!(state.phase, GamePhase::Lost);
        assert_eq!(state.buffer.len(), 4); // tile 4 NOT added
    }

    #[test]
    fn test_buffer_pair_when_full() {
        // Buffer fills with 4 distinct, then a 5th that matches one in buffer
        // → pair forms, no game over. Buffer has 3 after removal.
        // A 6th tile keeps the board non-empty so the level isn't completed.
        let tiles = vec![
            make_tile(0, 3, 0, 0, 0),
            make_tile(1, 5, 0, 0, 1),
            make_tile(2, 7, 0, 0, 2),
            make_tile(3, 9, 0, 0, 3),
            make_tile(4, 11, 0, 0, 2), // matches tile 2
            make_tile(5, 13, 0, 0, 5),
        ];
        let mut state = GameState::new();
        state.load_level(1, &make_level(tiles));

        for tid in 0..4 {
            buffer_click(&mut state, tid);
        }
        assert_eq!(state.buffer.len(), 4);

        let paired = buffer_click(&mut state, 4);
        assert!(paired);
        assert_eq!(state.phase, GamePhase::Playing);
        assert_eq!(state.buffer, vec![0, 1, 3]); // 2 removed, compacted left
        assert!(!state.tiles[2].active);
        assert!(!state.tiles[4].active);
        assert!(state.tiles[5].active);
    }

    #[test]
    fn test_buffer_preserves_order_and_compacts() {
        // Add 3 distinct tiles, then pair the second one → order preserved.
        let tiles = vec![
            make_tile(0, 3, 0, 0, 0),
            make_tile(1, 5, 0, 0, 1),
            make_tile(2, 7, 0, 0, 1),
            make_tile(3, 9, 0, 0, 2),
        ];
        let mut state = GameState::new();
        state.load_level(1, &make_level(tiles));

        buffer_click(&mut state, 0); // [0]
        buffer_click(&mut state, 1); // [0, 1]
        assert_eq!(state.buffer, vec![0, 1]);

        // Click 3 (distinct) → [0, 1, 3]
        buffer_click(&mut state, 3);
        assert_eq!(state.buffer, vec![0, 1, 3]);

        // Click 2 (matches 1) → removes 1, leaves [0, 3]
        assert!(buffer_click(&mut state, 2));
        assert_eq!(state.buffer, vec![0, 3]);
    }

    #[test]
    fn test_buffer_undo_add() {
        let tiles = vec![make_tile(0, 3, 0, 0, 0)];
        let mut state = GameState::new();
        state.load_level(1, &make_level(tiles));

        buffer_click(&mut state, 0);
        assert_eq!(state.buffer, vec![0]);
        assert!(do_undo(&mut state));
        assert_eq!(state.buffer.len(), 0);
        assert!(state.tiles[0].active);
    }

    #[test]
    fn test_buffer_undo_pair() {
        let tiles = vec![
            make_tile(0, 3, 0, 0, 0),
            make_tile(1, 7, 0, 0, 0),
            make_tile(2, 11, 0, 0, 1),
        ];
        let mut state = GameState::new();
        state.load_level(1, &make_level(tiles));

        assert!(!buffer_click(&mut state, 0)); // add 0
        assert!(!buffer_click(&mut state, 2)); // add 2
        assert_eq!(state.buffer, vec![0, 2]);
        assert!(buffer_click(&mut state, 1)); // pair with 0 → removes 0,1
        assert_eq!(state.buffer, vec![2]); // tile 2 stays
        assert!(!state.tiles[0].active);
        assert!(!state.tiles[1].active);

        // Undo the pairing: both tiles back, 0 back in buffer at pos 0.
        assert!(do_undo(&mut state));
        assert!(state.tiles[0].active);
        assert!(state.tiles[1].active);
        assert_eq!(state.buffer, vec![0, 2]);
    }

    #[test]
    fn test_buffer_undo_after_game_over() {
        let tiles = vec![
            make_tile(0, 3, 0, 0, 0),
            make_tile(1, 5, 0, 0, 1),
            make_tile(2, 7, 0, 0, 2),
            make_tile(3, 9, 0, 0, 3),
            make_tile(4, 11, 0, 0, 4),
        ];
        let mut state = GameState::new();
        state.load_level(1, &make_level(tiles));

        for tid in 0..4 {
            buffer_click(&mut state, tid);
        }
        buffer_click(&mut state, 4); // game over
        assert_eq!(state.phase, GamePhase::Lost);

        // Undo the "add to buffer" of tile 3 → back to Playing.
        assert!(do_undo(&mut state));
        assert_eq!(state.phase, GamePhase::Playing);
        assert_eq!(state.buffer, vec![0, 1, 2]);
    }

    #[test]
    fn test_level_completion() {
        let tiles = vec![
            make_tile(0, 3, 0, 0, 7),
            make_tile(1, 7, 0, 0, 7),
        ];
        let mut state = GameState::new();
        state.load_level(1, &make_level(tiles));

        assert!(!buffer_click(&mut state, 0)); // add to buffer
        assert_eq!(state.buffer, vec![0]);
        assert!(buffer_click(&mut state, 1)); // pair → level complete
        assert_eq!(state.active_tile_count(), 0);
        assert_eq!(state.phase, GamePhase::LevelCompleted);
    }
}
