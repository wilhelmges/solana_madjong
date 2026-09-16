"""Generates a unified tile-logo set (256x256 transparent PNG) into assets/tiles/.

Style: transparent background, solid circle in the tile's theme color,
darker ring, white bold abbreviation. Designed for the procedural 3D
renderer in src/renderer/draw.rs, which draws the cream face + caption
in code and composites this logo centered on top.

Originals are backed up in assets/tiles_backup_original/.
Run: python tools/gen_unified_logos.py
"""
import os
from PIL import Image, ImageDraw, ImageFont

OUT = os.path.join(os.path.dirname(__file__), "..", "assets", "tiles")
SIZE = 256
CENTER = 128
RADIUS = 92

# (TileType name, file stem, abbrev, (r, g, b)) — colors from src/theme/solana.rs
TILES = [
    ("SOL", "sol", "SOL", (150, 50, 200)),
    ("USDC", "usdc", "$", (0, 200, 100)),
    ("USDT", "usdt", "T", (200, 100, 0)),
    ("BONK", "bonk", "BONK", (200, 50, 50)),
    ("WIF", "wif", "WIF", (50, 200, 200)),
    ("JUP", "jup", "JUP", (100, 200, 100)),
    ("RAY", "ray", "RAY", (200, 200, 50)),
    ("ORCA", "orca", "ORCA", (50, 100, 200)),
    ("PYTH", "pyth", "PYTH", (200, 150, 100)),
    ("JTO", "jto", "JTO", (150, 200, 50)),
    ("Swap", "swap", "SW", (0, 150, 200)),
    ("Stake", "stake", "ST", (0, 200, 150)),
    ("Lend", "lend", "LE", (50, 200, 200)),
    ("Yield", "yield", "Y", (100, 200, 150)),
    ("Bridge", "bridge", "BR", (0, 180, 180)),
    ("DAO", "dao", "DAO", (50, 150, 200)),
    ("NFT", "nft", "NFT", (100, 180, 200)),
    ("Mint", "mint", "MI", (0, 200, 180)),
    ("Pool", "pool", "PO", (50, 200, 180)),
    ("Farm", "farm", "FA", (100, 200, 180)),
    ("Saga", "saga", "S", (180, 0, 180)),
    ("Firedancer", "firedancer", "FD", (200, 0, 150)),
    ("Token2022", "token2022", "T22", (220, 50, 200)),
    ("Program", "program", "</>", (200, 0, 200)),
    ("Account", "account", "AC", (180, 50, 180)),
    ("Block", "block", "BL", (200, 100, 200)),
    ("Vote", "vote", "V", (180, 0, 200)),
    ("Wallet", "wallet", "W", (200, 50, 180)),
    ("Jito", "jito", "JI", (50, 50, 200)),
    ("Marinade", "marinade", "MA", (100, 50, 200)),
    ("Lido", "lido", "LI", (150, 50, 200)),
    ("Rocket", "rocket", "R", (200, 50, 200)),
    ("Figment", "figment", "FI", (50, 100, 200)),
    ("Anza", "anza", "AN", (100, 100, 200)),
    ("Triton", "triton", "TR", (150, 100, 200)),
    ("Helius", "helius", "HE", (200, 100, 200)),
]


def load_font(size: int):
    for path in (
        r"C:\Windows\Fonts\arialbd.ttf",
        r"C:\Windows\Fonts\arial.ttf",
        "/usr/share/fonts/truetype/dejavu/DejaVuSans-Bold.ttf",
    ):
        try:
            return ImageFont.truetype(path, size)
        except OSError:
            continue
    return ImageFont.load_default()


def darker(color, f=0.62):
    return tuple(max(0, min(255, int(c * f))) for c in color)


def make_one(abbrev: str, color) -> Image.Image:
    img = Image.new("RGBA", (SIZE, SIZE), (0, 0, 0, 0))

    def layer() -> tuple[Image.Image, ImageDraw.ImageDraw]:
        l = Image.new("RGBA", (SIZE, SIZE), (0, 0, 0, 0))
        return l, ImageDraw.Draw(l)

    # Drop shadow (soft, transparent) — renderer also draws one, keep tiny.
    l, d = layer()
    d.ellipse(
        [CENTER - RADIUS + 3, CENTER - RADIUS + 6, CENTER + RADIUS + 3, CENTER + RADIUS + 6],
        fill=(0, 0, 0, 60),
    )
    img = Image.alpha_composite(img, l)

    # Main disc + darker outer ring.
    l, d = layer()
    d.ellipse(
        [CENTER - RADIUS, CENTER - RADIUS, CENTER + RADIUS, CENTER + RADIUS],
        fill=color + (255,),
    )
    d.ellipse(
        [CENTER - RADIUS, CENTER - RADIUS, CENTER + RADIUS, CENTER + RADIUS],
        outline=darker(color) + (255,),
        width=7,
    )
    img = Image.alpha_composite(img, l)

    # Subtle top gloss (own layer so the alpha composites over the disc).
    l, d = layer()
    d.ellipse(
        [CENTER - 58, CENTER - 82, CENTER + 58, CENTER - 34],
        fill=(255, 255, 255, 34),
    )
    img = Image.alpha_composite(img, l)

    # Abbreviation, auto-fit (opaque white text drawn directly).
    d = ImageDraw.Draw(img)
    # Abbreviation, auto-fit.
    size = 92
    font = load_font(size)
    while size > 20:
        bbox = d.textbbox((0, 0), abbrev, font=font, stroke_width=0)
        if bbox[2] - bbox[0] <= 132:
            break
        size -= 6
        font = load_font(size)
    bbox = d.textbbox((0, 0), abbrev, font=font)
    tw, th = bbox[2] - bbox[0], bbox[3] - bbox[1]
    d.text(
        (CENTER - tw / 2 - bbox[0], CENTER - th / 2 - bbox[1] - 2),
        abbrev,
        font=font,
        fill=(255, 255, 255, 255),
    )
    return img


def main():
    os.makedirs(OUT, exist_ok=True)
    for _name, stem, abbrev, color in TILES:
        img = make_one(abbrev, color)
        path = os.path.join(OUT, stem + ".png")
        img.save(path, "PNG")
        print("wrote", path)
    print(f"Done: {len(TILES)} images")


if __name__ == "__main__":
    main()
