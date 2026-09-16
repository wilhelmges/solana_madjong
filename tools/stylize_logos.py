"""Stylize official brand marks into unified game badges (256x256 transparent PNG).

Badge style (matches tools/gen_unified_logos.py monograms):
  transparent bg, theme-color disc R92 + darker ring, subtle gloss.
Modes per mark:
  badge  - full-bleed-ish mark placed straight on the theme disc
           (for marks designed as coins/medallions: JUP, ORCA)
  white  - white inner disc R66, mark fitted inside (for line marks: SOL, RAY)

Marks are never stretched or recolored (per Raydium/Orca/Solana brand rules);
only uniformly scaled, centered, with >=14% clear-space margin.

Pilot run writes previews to tools/preview/ (game assets untouched).
Final run overwrites assets/tiles/<stem>.png.
Run: python tools/stylize_logos.py [--apply]
"""
import os
import sys
from PIL import Image, ImageDraw

ROOT = os.path.join(os.path.dirname(__file__), "..")
SRC = os.path.join(os.path.dirname(__file__), "logo_src")
PREVIEW = os.path.join(os.path.dirname(__file__), "preview")
OUT = os.path.join(ROOT, "assets", "tiles")
SIZE = 256
CENTER = 128
RADIUS = 92

# stem, mode, mark file, badge color (theme), mark box px, circular-crop
PILOTS = [
    ("sol", "white", "sol/mark-256.png", (150, 50, 200), 118, False),
    ("ray", "white", "ray/mark-test.png", (200, 200, 50), 118, False),
    ("jup", "badge", "jup/token-256.png", (100, 200, 100), 184, False),
    ("orca", "badge", "orca/logomark-256.png", (50, 100, 200), 184, False),
]

BATCH2 = [
    ("usdc", "badge", "cg/usdc.png", (0, 200, 100), 184, False),
    ("usdt", "white", "cg/usdt.png", (200, 100, 0), 118, False),
    ("bonk", "badge", "cg/bonk.jpg", (200, 50, 50), 184, True),
    ("wif", "badge", "cg/wif.jpg", (50, 200, 200), 184, True),
    ("pyth", "badge", "cg/pyth.png", (200, 150, 100), 184, True),
    ("jto", "badge", "cg/jto.png", (150, 200, 50), 184, False),
    ("jito", "badge", "cg/jto.png", (50, 50, 200), 184, False),
    ("marinade", "badge", "gh/marinade.png", (100, 50, 200), 184, False),
    ("lido", "badge", "gh/lido.png", (150, 50, 200), 184, True),
    ("helius", "badge", "gh/helius.png", (200, 100, 200), 184, True),
    ("anza", "badge", "gh/anza.png", (100, 100, 200), 184, True),
    ("saga", "badge", "gh/saga.png", (180, 0, 180), 184, True),
    ("firedancer", "badge", "gh/firedancer.png", (200, 0, 150), 184, False),
]

ENTRIES = PILOTS + BATCH2


def darker(color, f=0.62):
    return tuple(max(0, min(255, int(c * f))) for c in color)


def base_badge(color):
    img = Image.new("RGBA", (SIZE, SIZE), (0, 0, 0, 0))
    d = ImageDraw.Draw(img)
    d.ellipse(
        [CENTER - RADIUS + 3, CENTER - RADIUS + 6, CENTER + RADIUS + 3, CENTER + RADIUS + 6],
        fill=(0, 0, 0, 60),
    )
    img0 = Image.new("RGBA", (SIZE, SIZE), (0, 0, 0, 0))
    d0 = ImageDraw.Draw(img0)
    d0.ellipse(
        [CENTER - RADIUS, CENTER - RADIUS, CENTER + RADIUS, CENTER + RADIUS],
        fill=color + (255,),
    )
    d0.ellipse(
        [CENTER - RADIUS, CENTER - RADIUS, CENTER + RADIUS, CENTER + RADIUS],
        outline=darker(color) + (255,),
        width=7,
    )
    img = Image.alpha_composite(img, img0)
    gloss = Image.new("RGBA", (SIZE, SIZE), (0, 0, 0, 0))
    dg = ImageDraw.Draw(gloss)
    dg.ellipse(
        [CENTER - 58, CENTER - 82, CENTER + 58, CENTER - 34],
        fill=(255, 255, 255, 34),
    )
    return img, gloss


def fit_mark(path, box, circ=False):
    mark = Image.open(path).convert("RGBA")
    mark.thumbnail((box, box), Image.LANCZOS)
    if circ:
        # Center-square circular crop (for opaque square sources: photos, avatars).
        s = min(mark.size)
        left = (mark.width - s) // 2
        top = (mark.height - s) // 2
        mark = mark.crop((left, top, left + s, top + s))
        mask = Image.new("L", (s, s), 0)
        ImageDraw.Draw(mask).ellipse([0, 0, s, s], fill=255)
        mark.putalpha(mask)
    canvas = Image.new("RGBA", (SIZE, SIZE), (0, 0, 0, 0))
    canvas.alpha_composite(mark, (CENTER - mark.width // 2, CENTER - mark.height // 2))
    return canvas


def stylize(stem, mode, mark_rel, color, box, circ=False):
    img, gloss = base_badge(color)
    if mode == "white":
        inner = Image.new("RGBA", (SIZE, SIZE), (0, 0, 0, 0))
        di = ImageDraw.Draw(inner)
        di.ellipse(
            [CENTER - 66, CENTER - 66, CENTER + 66, CENTER + 66],
            fill=(255, 253, 245, 255),
        )
        img = Image.alpha_composite(img, inner)
    mark = fit_mark(os.path.join(SRC, mark_rel), box, circ)
    img = Image.alpha_composite(img, mark)
    img = Image.alpha_composite(img, gloss)
    return img


def main():
    apply = "--apply" in sys.argv
    only = [a for a in sys.argv[1:] if not a.startswith("--")]
    dest_dir = OUT if apply else PREVIEW
    os.makedirs(dest_dir, exist_ok=True)
    for stem, mode, mark_rel, color, box, circ in ENTRIES:
        if only and stem not in only:
            continue
        img = stylize(stem, mode, mark_rel, color, box, circ)
        path = os.path.join(dest_dir, stem + ".png")
        img.save(path, "PNG")
        print(("applied " if apply else "preview ") + path)


if __name__ == "__main__":
    main()
