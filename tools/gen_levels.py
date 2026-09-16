"""Generate simplified levels 2-10 for Solana Mahjong.

Level 1 is kept as-is (12 tiles). Levels 2-10 are reduced from 144 tiles
to smaller, more focused boards that fit comfortably on screen.

Grid uses 2-unit spacing. Upper layer offsets must also be even (multiples of 2)
to align with base positions.

Progression:
  Lv  2: 24 tiles  (6x4, 1 layer, 6 types)
  Lv  3: 32 tiles  (6x4 base + 4x2 upper, 2 layers, 8 types)
  Lv  4: 36 tiles  (6x6, 1 layer, 9 types)
  Lv  5: 48 tiles  (6x6 base + 4x3 upper, 2 layers, 12 types)
  Lv  6: 48 tiles  (8x6, 1 layer, 12 types)
  Lv  7: 56 tiles  (8x6 base + 4x2 upper, 2 layers, 12 types)
  Lv  8: 60 tiles  (8x6 base + 4x3 upper, 2 layers, 12 types)
  Lv  9: 72 tiles  (8x6 base + 6x4 upper, 2 layers, 12 types)
  Lv 10: 80 tiles  (10x8 base + 4x2 upper, 2 layers, 12 types)
"""
import json, os, random

random.seed(42)

OUT_DIR = os.path.join(os.path.dirname(__file__), '..', 'levels')


def write_level(num, tiles):
    path = os.path.join(OUT_DIR, f'level_{num:02d}.json')
    with open(path, 'w') as f:
        json.dump({'tiles': tiles}, f)
    # Verify: every tile on z>0 has a base tile below it.
    base_xy = {(t['x'], t['y']) for t in tiles if t['z'] == 0}
    for t in tiles:
        if t['z'] > 0:
            assert (t['x'], t['y']) in base_xy, \
                f"Orphan tile at ({t['x']},{t['y']}) z={t['z']}"
    # Verify: even tile count.
    assert len(tiles) % 2 == 0, f"Odd tile count: {len(tiles)}"
    print(f'  level_{num:02d}: {len(tiles)} tiles, '
          f'{max(t["z"] for t in tiles)+1} layers, '
          f'{len(set(t["type_id"] for t in tiles))} types')


def make_grid(cols, rows, x0=0, y0=0):
    """Generate (x, y) positions for a rectangular grid with 2-unit spacing."""
    return [(x0 + c * 2, y0 + r * 2)
            for r in range(rows) for c in range(cols)]


def fill_pairs(positions, num_types):
    """Assign type_ids so every tile has exactly one matching pair."""
    n = len(positions)
    assert n % 2 == 0, f"Need even tile count, got {n}"
    pairs_needed = n // 2
    types = [(i % num_types) for i in range(pairs_needed)]
    types = types + types  # duplicate for pairs
    random.shuffle(types)
    return [{'x': p[0], 'y': p[1], 'z': 0, 'type_id': t}
            for p, t in zip(positions, types)]


def add_upper_layer(tiles, cols, rows, x0, y0, z=1, num_types=None):
    """Add an upper layer. Only places tiles where a base tile exists."""
    base_xy = {(t['x'], t['y']) for t in tiles if t['z'] == 0}
    positions = make_grid(cols, rows, x0, y0)
    valid = [(x, y) for x, y in positions if (x, y) in base_xy]
    n = len(valid)
    if n % 2 != 0:
        n -= 1
        valid = valid[:n]
    if num_types is None:
        num_types = max(t['type_id'] for t in tiles) + 1
    pairs = n // 2
    types = [(i % num_types) for i in range(pairs)]
    types = types + types
    random.shuffle(types)
    for (x, y), t in zip(valid, types):
        tiles.append({'x': x, 'y': y, 'z': z, 'type_id': t})
    return tiles


print('Generating levels...')

# Lv 2: 24 tiles — simple 6x4 grid, 1 layer
write_level(2, fill_pairs(make_grid(6, 4), 6))

# Lv 3: 32 tiles — 6x4 base + 4x2 upper center
t = fill_pairs(make_grid(6, 4), 8)
add_upper_layer(t, 4, 2, x0=2, y0=0, z=1, num_types=8)
write_level(3, t)

# Lv 4: 36 tiles — 6x6 grid, 1 layer
write_level(4, fill_pairs(make_grid(6, 6), 9))

# Lv 5: 48 tiles — 6x6 base + 4x3 upper center
t = fill_pairs(make_grid(6, 6), 12)
add_upper_layer(t, 4, 3, x0=2, y0=0, z=1, num_types=12)
write_level(5, t)

# Lv 6: 48 tiles — 8x6 grid, 1 layer
write_level(6, fill_pairs(make_grid(8, 6), 12))

# Lv 7: 56 tiles — 8x6 base + 4x2 upper center
t = fill_pairs(make_grid(8, 6), 12)
add_upper_layer(t, 4, 2, x0=4, y0=0, z=1, num_types=12)
write_level(7, t)

# Lv 8: 60 tiles — 8x6 base + 4x3 upper center
t = fill_pairs(make_grid(8, 6), 12)
add_upper_layer(t, 4, 3, x0=4, y0=0, z=1, num_types=12)
write_level(8, t)

# Lv 9: 72 tiles — 8x6 base + 6x4 upper center
t = fill_pairs(make_grid(8, 6), 12)
add_upper_layer(t, 6, 4, x0=2, y0=0, z=1, num_types=12)
write_level(9, t)

# Lv 10: 80 tiles — 10x8 base + 4x2 upper center
t = fill_pairs(make_grid(10, 8), 12)
add_upper_layer(t, 4, 2, x0=6, y0=2, z=1, num_types=12)
write_level(10, t)

print('Done.')
