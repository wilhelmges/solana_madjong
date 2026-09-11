from pathlib import Path
import io
import re
import shutil
import zipfile

import requests
from PIL import Image, ImageDraw


# ============================================================
# НАЛАШТУВАННЯ
# ============================================================

INPUT_ZIP = Path("tiles.zip")
OUTPUT_DIR = Path("solana_logos")

IMAGE_SIZE = 1024

REQUEST_TIMEOUT = 30

# Спочатку рекомендую True.
# Скрипт покаже, що саме він знайшов,
# але нічого не завантажуватиме.
DRY_RUN = False


# ============================================================
# GITHUB
# ============================================================

GITHUB_API = (
    "https://api.github.com/repos/"
    "metasolbot/solana-icons/git/trees/main?recursive=1"
)

RAW_BASE = (
    "https://raw.githubusercontent.com/"
    "metasolbot/solana-icons/main/"
)


# ============================================================
# ЯВНІ ВІДПОВІДНОСТІ
#
# Ліва частина = назва твого PNG.
#
# Права частина = можливі назви реального проекту.
# Скрипт сам шукає відповідний PNG.
# ============================================================

ALIASES = {

    "sol": [
        "solana",
    ],

    "jup": [
        "jupiter",
        "jupiter-exchange",
    ],

    "jto": [
        "jito",
        "jto",
    ],

    "jito": [
        "jito",
    ],

    "ray": [
        "raydium",
    ],

    "orca": [
        "orca",
    ],

    "helius": [
        "helius",
    ],

    "pyth": [
        "pyth",
    ],

    "triton": [
        "triton",
    ],

    "marinade": [
        "marinade",
    ],

    "lido": [
        "lido",
    ],

    "bonk": [
        "bonk",
    ],

    "wif": [
        "dogwifhat",
        "wif",
        "dog-wif-hat",
    ],

    "saga": [
        "saga",
    ],

    "firedancer": [
        "firedancer",
    ],

    "figment": [
        "figment",
    ],

    "anza": [
        "anza",
    ],

    "usdc": [
        "usdc",
        "usd-coin",
    ],

    "usdt": [
        "usdt",
        "tether",
    ],

    "wallet": [
        "phantom",
    ],

    "account": [
        "phantom",
    ],

    "nft": [
        "tensor",
        "metaplex",
    ],

    "mint": [
        "metaplex",
    ],

    "stake": [
        "sanctum",
    ],

    "lend": [
        "kamino",
    ],

    "dao": [
        "squads",
        "realms",
    ],

    "vote": [
        "realms",
    ],

    "program": [
        "anchor",
    ],

    "bridge": [
        "wormhole",
    ],

    "pool": [
        "orca",
    ],

    "swap": [
        "jupiter",
        "jupiter-exchange",
    ],

    "farm": [
        "raydium",
    ],

    "yield": [
        "jito",
    ],

    "rocket": [
        "saga",
    ],

    "token2022": [
        "token2022",
        "token-2022",
        "solana",
    ],
}


# ============================================================
# HTTP SESSION
# ============================================================

session = requests.Session()

session.headers.update({
    "User-Agent": "Solana-Logo-Downloader/1.0"
})


# ============================================================
# NORMALIZE
# ============================================================

def normalize(value: str) -> str:

    value = value.lower()

    return re.sub(
        r"[^a-z0-9]+",
        "",
        value,
    )


# ============================================================
# ОТРИМАННЯ СПИСКУ ФАЙЛІВ REPOSITORY
# ============================================================

def get_repository_files():

    print("Отримую список assets...")

    response = session.get(
        GITHUB_API,
        timeout=REQUEST_TIMEOUT,
    )

    response.raise_for_status()

    data = response.json()

    files = []

    for item in data.get("tree", []):

        if item.get("type") != "blob":
            continue

        path = item.get("path", "")

        extension = Path(path).suffix.lower()

        # ВАЖЛИВО:
        # використовуємо тільки готові растрові файли.
        if extension in {
            ".png",
            ".jpg",
            ".jpeg",
            ".webp",
        }:

            files.append(path)

    return files


# ============================================================
# ПОШУК LOGO
# ============================================================

def find_logo(
    tile_name: str,
    repository_files: list[str],
):

    aliases = ALIASES.get(
        tile_name,
        [tile_name],
    )

    aliases = [
        normalize(alias)
        for alias in aliases
    ]

    candidates = []

    for path in repository_files:

        filename = Path(path).stem

        normalized_filename = normalize(
            filename
        )

        for alias in aliases:

            # --------------------------------------------
            # EXACT MATCH
            # --------------------------------------------

            if normalized_filename == alias:

                score = 1000

                candidates.append(
                    (
                        score,
                        path,
                    )
                )

                continue

            # --------------------------------------------
            # filename contains alias
            # --------------------------------------------

            if alias in normalized_filename:

                score = 500

                # Штраф за зайві символи.
                score -= (
                    len(normalized_filename)
                    - len(alias)
                )

                candidates.append(
                    (
                        score,
                        path,
                    )
                )

    if not candidates:
        return None

    candidates.sort(
        key=lambda x: x[0],
        reverse=True,
    )

    return candidates[0][1]


# ============================================================
# ОТРИМАННЯ НАЗВ З ТВОГО ZIP
# ============================================================

def get_tile_names():

    if not INPUT_ZIP.exists():

        raise FileNotFoundError(
            f"Не знайдено: {INPUT_ZIP}"
        )

    names = []

    with zipfile.ZipFile(
        INPUT_ZIP,
        "r",
    ) as archive:

        for filename in archive.namelist():

            path = Path(filename)

            if path.suffix.lower() in {
                ".png",
                ".jpg",
                ".jpeg",
                ".webp",
            }:

                names.append(
                    path.stem
                )

    return sorted(set(names))


# ============================================================
# DOWNLOAD
# ============================================================

def download_logo(
    repository_path: str,
):

    url = (
        RAW_BASE +
        repository_path
    )

    response = session.get(
        url,
        timeout=REQUEST_TIMEOUT,
    )

    response.raise_for_status()

    return response.content


# ============================================================
# LOAD PNG
# ============================================================

def load_image(data: bytes):

    image = Image.open(
        io.BytesIO(data)
    )

    return image.convert("RGBA")


# ============================================================
# CREATE UNIFIED TILE
# ============================================================

def create_tile(
    logo: Image.Image,
):

    canvas = Image.new(
        "RGBA",
        (
            IMAGE_SIZE,
            IMAGE_SIZE,
        ),
        (20, 22, 30, 255),
    )

    # --------------------------------------------
    # Біла область під логотипом
    # --------------------------------------------

    margin = 100

    area_size = (
        IMAGE_SIZE - margin * 2
    )

    background = Image.new(
        "RGBA",
        (
            area_size,
            area_size,
        ),
        (255, 255, 255, 255),
    )

    # --------------------------------------------
    # Масштабування LOGO
    # --------------------------------------------

    logo.thumbnail(
        (
            area_size - 80,
            area_size - 80,
        ),
        Image.Resampling.LANCZOS,
    )

    x = (
        area_size - logo.width
    ) // 2

    y = (
        area_size - logo.height
    ) // 2

    background.alpha_composite(
        logo,
        (
            x,
            y,
        ),
    )

    canvas.alpha_composite(
        background,
        (
            margin,
            margin,
        ),
    )

    # --------------------------------------------
    # Рамка
    # --------------------------------------------

    draw = ImageDraw.Draw(
        canvas
    )

    draw.rounded_rectangle(
        (
            20,
            20,
            IMAGE_SIZE - 20,
            IMAGE_SIZE - 20,
        ),
        radius=45,
        outline=(80, 85, 100, 255),
        width=4,
    )

    return canvas.convert("RGB")


# ============================================================
# MAIN
# ============================================================

def main():

    print()
    print("=" * 70)
    print("SOLANA LOGO DOWNLOADER")
    print("=" * 70)
    print()

    # --------------------------------------------
    # 1. Отримуємо назви оригінальних файлів
    # --------------------------------------------

    tile_names = get_tile_names()

    print(
        f"У ZIP знайдено: {len(tile_names)} файлів"
    )

    print()

    # --------------------------------------------
    # 2. Отримуємо каталог assets
    # --------------------------------------------

    repository_files = (
        get_repository_files()
    )

    print(
        f"PNG/JPG assets у repository: "
        f"{len(repository_files)}"
    )

    print()

    # --------------------------------------------
    # 3. MATCH
    # --------------------------------------------

    matches = {}

    print("=" * 70)
    print("ЗНАЙДЕНІ ВІДПОВІДНОСТІ")
    print("=" * 70)

    for tile_name in tile_names:

        match = find_logo(
            tile_name,
            repository_files,
        )

        matches[tile_name] = match

        if match:

            print(
                f"{tile_name:<15} -> {match}"
            )

        else:

            print(
                f"{tile_name:<15} -> NOT FOUND"
            )

    print()

    # --------------------------------------------
    # DRY RUN
    # --------------------------------------------

    if DRY_RUN:

        print("=" * 70)
        print("DRY_RUN = True")
        print("=" * 70)

        print()
        print(
            "Файли ще НЕ завантажені."
        )

        print(
            "Перевір відповідності вище."
        )

        print(
            "Якщо все правильно, зміни:"
        )

        print(
            "    DRY_RUN = False"
        )

        print(
            "і запусти скрипт ще раз."
        )

        return

    # --------------------------------------------
    # 4. Очистити output
    # --------------------------------------------

    if OUTPUT_DIR.exists():

        shutil.rmtree(
            OUTPUT_DIR
        )

    OUTPUT_DIR.mkdir(
        parents=True
    )

    # --------------------------------------------
    # 5. DOWNLOAD
    # --------------------------------------------

    success = 0
    errors = 0

    print("=" * 70)
    print("ЗАВАНТАЖЕННЯ")
    print("=" * 70)

    for tile_name in tile_names:

        repository_path = (
            matches[tile_name]
        )

        if not repository_path:

            print(
                f"[SKIP] {tile_name}"
            )

            errors += 1

            continue

        try:

            print(
                f"[DOWNLOAD] {tile_name}"
            )

            data = download_logo(
                repository_path
            )

            logo = load_image(
                data
            )

            tile = create_tile(
                logo
            )

            output_file = (
                OUTPUT_DIR /
                f"{tile_name}.png"
            )

            tile.save(
                output_file,
                "PNG",
                optimize=True,
            )

            print(
                f"           -> {output_file}"
            )

            success += 1

        except Exception as exc:

            print(
                f"[ERROR] {tile_name}: {exc}"
            )

            errors += 1

    # --------------------------------------------
    # 6. RESULT
    # --------------------------------------------

    print()
    print("=" * 70)
    print("ГОТОВО")
    print("=" * 70)

    print(
        f"Успішно: {success}"
    )

    print(
        f"Помилок: {errors}"
    )

    print(
        f"Папка: {OUTPUT_DIR.resolve()}"
    )


if __name__ == "__main__":
    main()
