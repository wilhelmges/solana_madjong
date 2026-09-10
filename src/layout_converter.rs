use std::fs;
use std::path::Path;

#[derive(serde::Serialize)]
struct TilePos {
    x: i32,
    y: i32,
    z: i32,
    #[serde(rename = "type_id")]
    type_id: u32,
}

#[derive(serde::Serialize)]
struct Level {
    tiles: Vec<TilePos>,
}

fn parse_layout(content: &str) -> Level {
    let lines: Vec<&str> = content.lines().collect();
    let mut width: usize = 32;
    let mut height: usize = 16;
    let mut depth: usize = 0;
    let mut grids: Vec<Vec<String>> = Vec::new();
    let mut current_grid: Vec<String> = Vec::new();
    let mut in_grid = false;

    for line in &lines {
        let trimmed = line.trim();
        if trimmed.starts_with("kmahjongg") || trimmed.starts_with('#') || trimmed.is_empty() {
            continue;
        }
        if trimmed.starts_with('w') {
            width = trimmed[1..].parse().unwrap_or(32);
            continue;
        }
        if trimmed.starts_with('h') {
            height = trimmed[1..].parse().unwrap_or(16);
            continue;
        }
        if trimmed.starts_with('d') {
            depth = trimmed[1..].parse().unwrap_or(5);
            continue;
        }
        // It's a grid line
        if !in_grid {
            current_grid.clear();
            in_grid = true;
        }
        current_grid.push(trimmed.to_string());
        if current_grid.len() == height {
            grids.push(current_grid.clone());
            current_grid.clear();
            in_grid = false;
        }
    }
    if !current_grid.is_empty() {
        grids.push(current_grid);
    }

    if depth == 0 {
        depth = grids.len();
    }

    let mut tiles = Vec::new();
    let mut tile_index: u32 = 0;

    for (z, grid) in grids.iter().enumerate() {
        for (row, line) in grid.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch == '1' {
                    let type_id = tile_index % 36;
                    tiles.push(TilePos {
                        x: col as i32,
                        y: row as i32,
                        z: z as i32,
                        type_id,
                    });
                    tile_index += 1;
                }
            }
        }
    }

    println!(
        "  Parsed: {} tiles, {} levels, grid {}x{}",
        tiles.len(),
        grids.len(),
        width,
        height
    );

    Level { tiles }
}

fn main() {
    let layouts_dir = Path::new("tools/layouts");
    let output_dir = Path::new("levels");

    fs::create_dir_all(output_dir).expect("Failed to create levels directory");

    let layout_files = [
        // Ordered from easiest to hardest based on initial side-blocking:
        "dragon.layout",
        "pirates.layout",
        "bug.layout",
        "enterprise.layout",
        "order.layout",
        "arena.layout",
        "arrow.layout",
        "maya.layout",
        "cat.layout",
        "default.layout",
    ];

    for (i, filename) in layout_files.iter().enumerate() {
        let level_num = i + 1;
        let input_path = layouts_dir.join(filename);
        let output_path = output_dir.join(format!("level_{:02}.json", level_num));

        println!("Converting {} -> level_{:02}.json", filename, level_num);

        let content = fs::read_to_string(&input_path)
            .unwrap_or_else(|e| panic!("Failed to read {}: {}", input_path.display(), e));

        let level = parse_layout(&content);

        if level.tiles.len() != 144 {
            eprintln!(
                "  WARNING: {} has {} tiles, expected 144",
                filename,
                level.tiles.len()
            );
        }

        let json = serde_json::to_string_pretty(&level).expect("Failed to serialize");
        fs::write(&output_path, &json)
            .unwrap_or_else(|e| panic!("Failed to write {}: {}", output_path.display(), e));

        println!("  Written: {}", output_path.display());
    }

    println!("\nDone! {} level files generated.", layout_files.len());
}
