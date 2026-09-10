use std::fs;
use std::path::Path;

const SAVE_FILE: &str = "save.json";

#[derive(serde::Serialize, serde::Deserialize)]
struct SaveData {
    current_level: usize,
}

pub fn load_progress() -> usize {
    let path = Path::new(SAVE_FILE);
    if !path.exists() {
        return 1;
    }
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return 1,
    };
    let data: SaveData = match serde_json::from_str(&content) {
        Ok(d) => d,
        Err(_) => return 1,
    };
    data.current_level.clamp(1, 10)
}

pub fn save_progress(level: usize) {
    let data = SaveData {
        current_level: level.clamp(1, 10),
    };
    let json = match serde_json::to_string_pretty(&data) {
        Ok(j) => j,
        Err(_) => return,
    };
    let _ = fs::write(SAVE_FILE, json);
}

pub fn reset_progress() {
    let _ = fs::remove_file(SAVE_FILE);
}
