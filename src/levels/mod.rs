use crate::game_core::LevelData;

const LEVEL_1: &str = include_str!("../../levels/level_01.json");
const LEVEL_2: &str = include_str!("../../levels/level_02.json");
const LEVEL_3: &str = include_str!("../../levels/level_03.json");
const LEVEL_4: &str = include_str!("../../levels/level_04.json");
const LEVEL_5: &str = include_str!("../../levels/level_05.json");
const LEVEL_6: &str = include_str!("../../levels/level_06.json");
const LEVEL_7: &str = include_str!("../../levels/level_07.json");
const LEVEL_8: &str = include_str!("../../levels/level_08.json");
const LEVEL_9: &str = include_str!("../../levels/level_09.json");
const LEVEL_10: &str = include_str!("../../levels/level_10.json");

pub fn load_level(level: usize) -> Option<LevelData> {
    let raw: &str = match level {
        1 => LEVEL_1,
        2 => LEVEL_2,
        3 => LEVEL_3,
        4 => LEVEL_4,
        5 => LEVEL_5,
        6 => LEVEL_6,
        7 => LEVEL_7,
        8 => LEVEL_8,
        9 => LEVEL_9,
        10 => LEVEL_10,
        _ => return None,
    };

    serde_json::from_str(raw).ok()
}

pub fn total_levels() -> usize {
    10
}