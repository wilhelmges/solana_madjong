use super::{Theme, TileType};

pub fn solana_theme() -> Theme {
    let types_data: &[(&str, &str, (u8, u8, u8))] = &[
        ("SOL", "Token", (150, 50, 200)),
        ("USDC", "Token", (0, 200, 100)),
        ("USDT", "Token", (200, 100, 0)),
        ("BONK", "Token", (200, 50, 50)),
        ("WIF", "Token", (50, 200, 200)),
        ("JUP", "Token", (100, 200, 100)),
        ("RAY", "Token", (200, 200, 50)),
        ("ORCA", "Token", (50, 100, 200)),
        ("PYTH", "Token", (200, 150, 100)),
        ("JTO", "Token", (150, 200, 50)),
        ("Swap", "DeFi", (0, 150, 200)),
        ("Stake", "DeFi", (0, 200, 150)),
        ("Lend", "DeFi", (50, 200, 200)),
        ("Yield", "DeFi", (100, 200, 150)),
        ("Bridge", "DeFi", (0, 180, 180)),
        ("DAO", "DeFi", (50, 150, 200)),
        ("NFT", "DeFi", (100, 180, 200)),
        ("Mint", "DeFi", (0, 200, 180)),
        ("Pool", "DeFi", (50, 200, 180)),
        ("Farm", "DeFi", (100, 200, 180)),
        ("Saga", "Eco", (180, 0, 180)),
        ("Firedancer", "Eco", (200, 0, 150)),
        ("Token2022", "Eco", (220, 50, 200)),
        ("Program", "Eco", (200, 0, 200)),
        ("Account", "Eco", (180, 50, 180)),
        ("Block", "Eco", (200, 100, 200)),
        ("Vote", "Eco", (180, 0, 200)),
        ("Wallet", "Eco", (200, 50, 180)),
        ("Jito", "Validator", (50, 50, 200)),
        ("Marinade", "Validator", (100, 50, 200)),
        ("Lido", "Validator", (150, 50, 200)),
        ("Rocket", "Validator", (200, 50, 200)),
        ("Figment", "Validator", (50, 100, 200)),
        ("Anza", "Validator", (100, 100, 200)),
        ("Triton", "Validator", (150, 100, 200)),
        ("Helius", "Validator", (200, 100, 200)),
    ];

    let tile_types: Vec<TileType> = types_data
        .iter()
        .enumerate()
        .map(|(i, &(name, category, color))| TileType {
            id: i as u32,
            name,
            category,
            color,
        })
        .collect();

    Theme {
        name: "Solana",
        tile_types,
    }
}
