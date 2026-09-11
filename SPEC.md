# Solana Mahjong — Технічне завдання

## 1. Огляд проекту

**Назва:** Solong (Solana Mahjong)
**Тип:** Кросплатформенна гра-маджонг (Mahjong Solitaire) з тематикою екосистеми Solana
**Мова:** Rust (edition 2021)
**Рушій:** macroquad 0.4.16 (має вбудовану підтримку Android через miniquad 0.4.11)
**Цільові платформи:** Windows (x86_64), Android (aarch64)
**Поточна версія:** 0.1.0

## 2. Архітектура

### 2.1 Структура коду

```
src/
├── main.rs                  — Точка входу, головний цикл гри, App struct
├── layout_converter.rs      — Офлайн-інструмент: .layout → level JSON
├── game_core/
│   ├── mod.rs               — Публічний API game_core
│   ├── state.rs             — GameState, Tile, TilePosition, LevelData
│   ├── rules.rs             — Логіка вибору/з'єднання тайлів (KMahjongg rules)
│   └── progress.rs          — Збереження/завантаження прогресу (save.json)
├── renderer/
│   ├── mod.rs               — Renderer struct, компоновка тайлів, хіт-тестинг
│   ├── draw.rs              — Малювання всіх екранів (старт, гра, перехід, фінал)
│   ├── tiles.rs             — Завантаження текстур тайлів через include_bytes!
│   └── icons.rs             — Процедурне малювання векторних іконок тайлів
├── input/
│   └── mod.rs               — Enum Action (SelectTile, RestartLevel, тощо)
├── levels/
│   └── mod.rs               — Завантаження 10 рівнів з вбудованих JSON-рядків
└── theme/
    ├── mod.rs               — Theme та TileType структури
    └── solana.rs            — Тема "Solana": 36 типів тайлів з назвами та кольорами
```

### 2.2 Створення

Для кожної платформи створюється окремий бінарник:
- **Windows:** `cargo build --release` → `solong.exe`
- **Android:** `cargo ndk -t arm64-v8a build --release` → `libsolong.so` (cdylib)

Для Android також компілюються Java-обгортки та збирається APK вручну (без Gradle):
1. `cargo ndk` → `.so`
2. `javac` → `.class`
3. `d8` → `classes.dex`
4. `aapt2 link` → unsigned APK
5. `aapt add` → додавання `.so` та `dex` в APK
6. `zipalign` + `apksigner` → фінальний APK

### 2.3 JNI-міст (Android)

Два Java-файли забезпечують зв'язок між Android SDK та Rust:

**`res/src/quad_native/QuadNative.java`** — Оголошення native-методів:
- `activityOnCreate(Object activity)` — Ініціалізація, зберігання Activity
- `activityOnResume()` / `activityOnPause()` / `activityOnDestroy()` — Життєвий цикл
- `surfaceOnSurfaceCreated(Surface)` / `surfaceOnSurfaceDestroyed(Surface)` — Створення/знищення Surface
- `surfaceOnSurfaceChanged(Surface, int, int)` — Зміна розміру Surface
- `surfaceOnTouch(int id, int phase, float x, float y)` — Дотик
- `surfaceOnKeyDown(int keycode)` / `surfaceOnKeyUp(int keycode)` / `surfaceOnCharacter(int character)` — Клавіатура

**`res/src/com/solong/mahjong/MainActivity.java`** — Android Activity:
- Керує `QuadSurface` (SurfaceView) для рендерингу
- Реалізує `OnTouchListener`, `OnKeyListener`, `SurfaceHolder.Callback`
- Делегує всі події через `QuadNative.*` виклики в нативний код

### 2.4 Критичні JNI-методи Activity (викликаються з Rust через JNI)

Мініквад викликає наступні методи на Activity-об'єкті через `GetObjectClass` + `GetMethodID`:

| Метод | Сигнатура | Коли викликається |
|-------|-----------|-------------------|
| `setFullScreen(boolean)` | `(Z)V` | При старті (якщо `fullscreen: true`), при зміні стану |
| `showKeyboard(boolean)` | `(Z)V` | При запиті показу/приховання клавіатури |
| `getClipboardText()` | `()Ljava/lang/String;` | При читанні буферу обміну |
| `setClipboardText(String)` | `(Ljava/lang/String;)V` | При записі в буфер обміну |
| `getAssets()` | `()Landroid/content/res/AssetManager;` | При завантаженні ассетів (стандартний метод Activity) |

> **Увага:** Відсутність будь-якого з цих методів у `MainActivity.java` призводить до `assertion failed: !method.is_null()` та крашу додатку через SIGABRT.

## 3. Ігрова логіка

### 3.1 Тайли

- **36 унікальних типів тайлів** з тематики Solana-екосистеми (SOL, USDC, USDT, BONK, WIF, JUP, RAY, ORCA, PYTH, JTO, Swap, Stake, Lend, Yield, Bridge, DAO, NFT, Mint, Pool, Farm, Saga, Firedancer, Token2022, Program, Account, Block, Vote, Wallet, Jito, Marinade, Lido, Rocket, Figment, Anza, Triton, Helius)
- Кожен тайл має текстуру (256x26 PNG, вбудована через `include_bytes!`)
- Для тайлів без графічних ресурсів є процедурна векторна іконка (icons.rs)

### 3.2 Рівні

- **10 рівнів** (level_01.json — level_10.json)
- Кожен рівень — це KMahjongg-розкладка: масив тайлів з позиціями (x, y, z) та type_id
- Розкладки генеруються з файлів `.layout` (KMahjongg формат) через `layout_converter.rs`

### 3.3 Правила гри

- Класичний Mahjong Solitaire (KMahjongg rules): тайл можна вибрати, якщо він вільний зліва або справа і не закритий зверху
- Два однакових вільних тайли зникають при виборі
- Рівень завершується, коли всі тайли зібрані

### 3.4 Прогрес

- Зберігається в `save.json` (поточний рівень)
- Завантажується при старті гри
- Можна скинути через стартовий екран

## 4. Екрани

| Екран | Опис |
|-------|------|
| Start | Назва гри, кнопка "Start", кнопка "Reset Progress" |
| Game | Ігрове поле з тайлами, кнопки Restart та Exit |
| Transition | Екран між рівнями з кнопкою "Next Level" |
| Completion | Фінальний екран після проходження всіх 10 рівнів |

## 5. Зовнішні залежності

| Залежність | Версія | Призначення |
|------------|--------|-------------|
| macroquad | 0.4.16 | Рушій для рендерингу, введення, аудіо |
| miniquad | 0.4.11 | Низькорівнева абстракція платформи (входить у macroquad) |
| serde | 1.x | Серіалізація/десеріалізація JSON |
| serde_json | 1.x | Робота з JSON-даними рівнів та збереження |

## 6. Збірка

### Windows

```bat
cargo build --release
copy target\release\solong.exe .
```

### Android

```bat
call setenv.bat
cargo ndk -t arm64-v8a build --release
javac --release 8 -classpath "%ANDROID_JAR%" -d obj res\src\com\solong\mahjong\MainActivity.java res\src\quad_native\QuadNative.java
d8 --lib "%ANDROID_JAR%" --output target\android @target\android\classes_list.txt
aapt2 link -o target\android\Solong-unsigned.apk -I "%ANDROID_JAR%" --manifest res/AndroidManifest.xml --auto-add-overlay --min-sdk-version 28 --target-sdk-version 35
aapt add target\android\Solong-unsigned.apk target\android\classes.dex lib/arm64-v8a/libsolong.so
zipalign -f 4 target\android\Solong-unsigned.apk target\android\Solong-aligned.apk
apksigner sign --ks debug.keystore --out target\android\Solong.apk target\android\Solong-aligned.apk
```

### Середовище (setenv.bat)

- NDK: `29.0.14206865`
- Build tools: `35.0.0`
- Target SDK: `35`, Min SDK: `28`

## 7. Відомі обмеження та баги

### 7.1 Вирішено: Краш Android-додатку через відсутні JNI-методи

**Проблема:** Додаток крашився на Android через ~300 мс після старту з помилкою:
```
assertion failed: !method.is_null()
panic in a function that cannot unwind
Fatal signal 6 (SIGABRT)
```

**Причина:** `MainActivity.java` не реалізовувала методи, які мініквад викликає через JNI: `setFullScreen(boolean)`, `showKeyboard(boolean)`, `getClipboardText()`, `setClipboardText(String)`. Ці методи є в офіційному шаблоні miniquad-0.4.11, але були відсутні у кастомній версії.

**Рішення:** Додано 4 методи до `MainActivity.java`, скопійовано з `miniquad-0.4.11/java/MainActivity.java`.

### 7.2 Відсутні логотипи для деяких тайлів

Тайли без графічних ресурсів ( Placeholder-зображення генеруються `tools\gen_tile_images.ps1`, або використовується процедурна іконка з `icons.rs`):
- **Концепції без брендів:** Swap, Stake, Yield, Mint, Pool, Farm, Program, Account, Block, Vote
- **Офіційні PNG недоступні:** Saga, DAO, Token2022

### 7.3 Шлях до шрифту захардкоджений для Windows

У `main.rs:41`:
```rust
let font = match load_ttf_font("C:\\Windows\\Fonts\\arial.ttf").await {
    Ok(f) => f,
    Err(_) => get_default_font(),
};
```
На Android цей шлях не існує, тому завжди використовується шрифт за замовчуванням.

### 7.4 Збереження прогресу в робочому каталозі

`progress.rs` використовує відносний шлях `save.json`. На Android робочий каталог може бути непередбачуваним, тому збереження може не працювати між запусками.

## 8. Файли проекту

| Файл | Опис |
|------|------|
| `Cargo.toml` | Маніфест Rust: пакет solong v0.1.0, залежності |
| `build_windows.bat` | Скрипт збірки Windows |
| `build_android.bat` | Скрипт збірки Android APK (6 кроків) |
| `setenv.bat` | Налаштування середовища Android SDK/NDK |
| `debug.keystore` | Debug-ключ для підпису APK |
| `res/AndroidManifest.xml` | Маніфест Android: package com.solong.mahjong |
| `res/src/com/solong/mahjong/MainActivity.java` | Android Activity (334 рядки) |
| `res/src/quad_native/QuadNative.java` | JNI native-оголошення |
| `vibecoding.log.txt` | Журнал розробки |
