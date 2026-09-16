use macroquad::prelude::*;

use crate::game_core::GameState;
use crate::renderer::tiles::TileTextures;
use crate::renderer::{Renderer, TileRenderInfo};
use crate::theme::Theme;

fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::from_rgba(r, g, b, 255)
}

fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color::from_rgba(r, g, b, a)
}

// Reference palette: deep navy background, cream tiles with thick sides.
const BG: (u8, u8, u8) = (13, 13, 43);
const FACE: (u8, u8, u8) = (253, 244, 223);
const SIDE_MID: (u8, u8, u8) = (214, 203, 175);
const SIDE_DARK: (u8, u8, u8) = (165, 153, 125);
const INK: (u8, u8, u8) = (48, 42, 35);
const GOLD: (u8, u8, u8) = (255, 205, 70);

pub struct UiButtons {
    pub hint: ButtonRect,
    pub shuffle: ButtonRect,
    pub undo: ButtonRect,
    pub restart: ButtonRect,
    pub exit: ButtonRect,
}

#[derive(Clone)]
pub struct ButtonRect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl ButtonRect {
    pub fn contains(&self, mx: f32, my: f32) -> bool {
        mx >= self.x && mx <= self.x + self.w && my >= self.y && my <= self.y + self.h
    }
    pub fn empty() -> Self {
        Self {
            x: -100.0,
            y: -100.0,
            w: 0.0,
            h: 0.0,
        }
    }
}

fn draw_rounded_rect(x: f32, y: f32, w: f32, h: f32, r: f32, color: Color) {
    if w <= 0.0 || h <= 0.0 {
        return;
    }
    let r = r.clamp(0.0, (w.min(h)) / 2.0);
    if r < 1.0 {
        draw_rectangle(x, y, w, h, color);
        return;
    }
    draw_rectangle(x + r, y, w - 2.0 * r, h, color);
    draw_rectangle(x, y + r, w, h - 2.0 * r, color);
    draw_circle(x + r, y + r, r, color);
    draw_circle(x + w - r, y + r, r, color);
    draw_circle(x + r, y + h - r, r, color);
    draw_circle(x + w - r, y + h - r, r, color);
}

fn pill_button(
    font: &Font,
    text: &str,
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    enabled: bool,
) -> ButtonRect {
    let (mx, my) = mouse_position();
    let rect = ButtonRect { x, y, w, h };
    let hovered = enabled && rect.contains(mx, my);
    let bg = if !enabled {
        rgba(255, 255, 255, 8)
    } else if hovered {
        rgba(255, 255, 255, 32)
    } else {
        rgba(255, 255, 255, 14)
    };
    let r = h * 0.5;
    draw_rounded_rect(x, y, w, h, r, bg);
    draw_rounded_rect(x, y, w, h, r, rgba(255, 255, 255, 0)); // keep shape crisp
    draw_circle_lines(x + r, y + h / 2.0, h / 2.0 - 1.0, 1.2, rgba(255, 255, 255, 45));
    draw_circle_lines(
        x + w - r,
        y + h / 2.0,
        h / 2.0 - 1.0,
        1.2,
        rgba(255, 255, 255, 45),
    );
    draw_rectangle(x + r, y + 0.8, w - 2.0 * r, 1.2, rgba(255, 255, 255, 45));
    draw_rectangle(
        x + r,
        y + h - 2.0,
        w - 2.0 * r,
        1.2,
        rgba(255, 255, 255, 45),
    );
    let fs = 17u16;
    let tw = measure_text(text, Some(font), fs, 1.0).width;
    let color = if enabled { WHITE } else { rgba(255, 255, 255, 110) };
    draw_text_ex(
        text,
        x + (w - tw) / 2.0,
        y + h / 2.0 + fs as f32 * 0.36,
        TextParams {
            font: Some(font),
            font_size: fs,
            color,
            ..Default::default()
        },
    );
    rect
}

fn pill_label(font: &Font, text: &str, x: f32, y: f32, w: f32, h: f32) {
    let r = h * 0.5;
    draw_rounded_rect(x, y, w, h, r, rgba(255, 255, 255, 14));
    let fs = 17u16;
    let tw = measure_text(text, Some(font), fs, 1.0).width;
    draw_text_ex(
        text,
        x + (w - tw) / 2.0,
        y + h / 2.0 + fs as f32 * 0.36,
        TextParams {
            font: Some(font),
            font_size: fs,
            color: WHITE,
            ..Default::default()
        },
    );
}

pub fn draw_game_screen(
    renderer: &Renderer,
    state: &GameState,
    theme: &Theme,
    font: &Font,
    screen_w: f32,
    screen_h: f32,
    elapsed_sec: u64,
) -> UiButtons {
    clear_background(rgb(BG.0, BG.1, BG.2));
    // Subtle vignette / glow so the pyramid pops like in the reference.
    draw_circle(
        screen_w * 0.5,
        screen_h * 0.46,
        screen_w.min(screen_h) * 0.55,
        rgba(60, 50, 140, 28),
    );
    draw_rectangle(
        0.0,
        screen_h - 120.0,
        screen_w,
        120.0,
        rgba(0, 0, 0, 60),
    );

    let pairs = state.active_tile_count() / 2;
    let mm = elapsed_sec / 60;
    let ss = elapsed_sec % 60;

    // ---- top HUD ----
    let top_y = 10.0f32;
    let ph = 42.0f32;
    pill_label(font, "Solana Mahjong", 10.0, top_y, 196.0, ph);
    let center_text = format!("Pairs: {}   Time: {:02}:{:02}", pairs, mm, ss);
    pill_label(
        font,
        &center_text,
        (screen_w - 300.0) / 2.0,
        top_y,
        300.0,
        ph,
    );

    let mut rx = screen_w - 10.0;
    let exit_rect = pill_button(font, "X", rx - 42.0, top_y, 42.0, ph, true);
    rx -= 42.0 + 8.0;
    let restart_rect = pill_button(font, "*", rx - 42.0, top_y, 42.0, ph, true);
    rx -= 42.0 + 8.0;
    let shuffle_rect = pill_button(
        font,
        &format!("Shuffle {}", state.shuffles_left),
        rx - 128.0,
        top_y,
        128.0,
        ph,
        state.shuffles_left > 0,
    );
    rx -= 128.0 + 8.0;
    let hint_rect = pill_button(
        font,
        &format!("Hint {}", state.hints_left),
        rx - 96.0,
        top_y,
        96.0,
        ph,
        state.hints_left > 0,
    );

    // ---- tiles ----
    for info in &renderer.tile_positions {
        draw_tile(info, state, theme, font, &renderer.tile_textures);
    }

    // ---- bottom HUD ----
    let by = screen_h - 46.0;
    pill_label(font, &format!("Score: {}", state.score), 10.0, by, 150.0, 36.0);
    let level_text = format!("Lv {}", state.current_level);
    pill_label(font, &level_text, 168.0, by, 64.0, 36.0);
    let undo_rect = pill_button(
        font,
        "Undo",
        screen_w - 130.0,
        by,
        120.0,
        36.0,
        !state.history.is_empty(),
    );

    UiButtons {
        hint: hint_rect,
        shuffle: shuffle_rect,
        undo: undo_rect,
        restart: restart_rect,
        exit: exit_rect,
    }
}

fn draw_tile(
    info: &TileRenderInfo,
    state: &GameState,
    theme: &Theme,
    font: &Font,
    textures: &TileTextures,
) {
    let tile = match state.get_tile(info.tile_id) {
        Some(t) => t,
        None => return,
    };

    let x = info.screen_x;
    let y = info.screen_y;
    let w = info.width;
    let h = info.height;
    let t = info.thickness;
    if w < 6.0 || h < 6.0 {
        return;
    }

    let is_selected = state.selected_tile_id == Some(info.tile_id);
    let is_hinted = state.hint_pair.map(|(a, b)| a == tile.id || b == tile.id).unwrap_or(false);
    let selectable = crate::game_core::is_selectable(state, tile);

    // Lift selected / hinted tiles slightly so they pop.
    let (mut sx, mut sy) = (x, y);
    if is_selected {
        sx -= 2.0;
        sy -= 3.0;
    } else if is_hinted {
        sy -= 2.0;
    }

    let r = (w.min(h) * 0.14).clamp(3.0, 9.0);

    // Selection / hint glow behind everything.
    if is_selected {
        draw_rounded_rect(sx - 2.0, sy - 2.0, w + 4.0, h + 4.0, r + 2.0, rgb(GOLD.0, GOLD.1, GOLD.2));
    } else if is_hinted {
        draw_rounded_rect(
            sx - 2.0,
            sy - 2.0,
            w + 4.0,
            h + 4.0,
            r + 2.0,
            rgb(80, 220, 255),
        );
    }

    // Soft drop shadow.
    draw_rounded_rect(sx + 3.0, sy + 5.0, w, h, r, rgba(0, 0, 0, 95));
    // Procedural thickness: two stepped offset layers = right/bottom sides.
    draw_rounded_rect(sx + t, sy + t, w, h, r, rgb(SIDE_DARK.0, SIDE_DARK.1, SIDE_DARK.2));
    draw_rounded_rect(
        sx + t * 0.5,
        sy + t * 0.5,
        w,
        h,
        r,
        rgb(SIDE_MID.0, SIDE_MID.1, SIDE_MID.2),
    );
    // Face.
    draw_rounded_rect(sx, sy, w, h, r, rgb(FACE.0, FACE.1, FACE.2));
    // Top gloss.
    draw_rounded_rect(
        sx + 3.0,
        sy + 2.0,
        w - 6.0,
        (h * 0.20).min(14.0),
        r * 0.6,
        rgba(255, 255, 255, 70),
    );

    // Logo: current PNGs are full-bleed placeholders, so draw them inset on the
    // cream face. Once logo-with-alpha assets arrive this becomes a clean icon.
    let tile_type = theme.tile_types.iter().find(|tt| tt.id == tile.type_id);
    let name = tile_type.map(|tt| tt.name).unwrap_or("?");
    let logo_box_h = h * 0.60;
    let logo_s = (w * 0.68).min(logo_box_h).clamp(8.0, 200.0);
    let logo_x = sx + (w - logo_s) / 2.0;
    let logo_y = sy + h * 0.08 + (logo_box_h - logo_s) / 2.0;
    if let Some(tex) = textures.get(name) {
        // Inset the art with a small cream margin so old full-bleed
        // placeholders don't cover the whole face.
        let inset = (w * 0.10).min(8.0);
        let dw = (logo_s - inset).max(6.0);
        let dx = logo_x + inset / 2.0;
        let dy = logo_y;
        draw_texture_ex(
            tex,
            dx,
            dy,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(dw, dw.min(logo_box_h))),
                ..Default::default()
            },
        );
    } else {
        // Fallback: dark badge + procedural white icon so it reads on cream.
        let cx = sx + w / 2.0;
        let cy = sy + h * 0.36;
        let s = w.min(h) * 0.26;
        draw_circle(cx, cy, s * 1.25, rgb(45, 42, 60));
        crate::renderer::icons::draw_tile_icon(name, cx, cy, s * 0.9);
    }

    // Caption: dark ink on cream, like the reference.
    let label = name.to_uppercase();
    let font_size = (h * 0.16).clamp(8.0, 15.0) as u16;
    let text_w = measure_text(&label, Some(font), font_size, 1.0).width;
    // Shrink long names (e.g. FIREDANCER) to fit.
    let mut fs = font_size;
    let mut tw = text_w;
    while tw > w - 8.0 && fs > 7 {
        fs -= 1;
        tw = measure_text(&label, Some(font), fs, 1.0).width;
    }
    draw_text_ex(
        &label,
        sx + (w - tw) / 2.0,
        sy + h * 0.80 + fs as f32 * 0.35,
        TextParams {
            font: Some(font),
            font_size: fs,
            color: rgb(INK.0, INK.1, INK.2),
            ..Default::default()
        },
    );

    // Blocked tiles are dimmed so free pairs read instantly.
    if !selectable {
        draw_rounded_rect(sx, sy, w, h, r, rgba(12, 12, 35, 105));
    }
    // Thin face edge for crispness.
    draw_rectangle_lines(sx + 0.5, sy + 0.5, w - 1.0, h - 1.0, 1.0, rgba(90, 75, 55, 120));
}

fn draw_button_widget(font: &Font, text: &str, rect: &ButtonRect, bg: Color, hover_bg: Color) {
    let mouse = mouse_position();
    let hovered = rect.contains(mouse.0, mouse.1);

    draw_rectangle(rect.x, rect.y, rect.w, rect.h, if hovered { hover_bg } else { bg });
    draw_rectangle_lines(rect.x, rect.y, rect.w, rect.h, 2.0, WHITE);

    let font_size = 22u16;
    let text_w = measure_text(text, Some(font), font_size, 1.0).width;
    draw_text_ex(
        text,
        rect.x + (rect.w - text_w) / 2.0,
        rect.y + rect.h / 2.0 + font_size as f32 / 3.0,
        TextParams {
            font: Some(font),
            font_size,
            color: WHITE,
            ..Default::default()
        },
    );
}

pub struct StartScreenButtons {
    pub start: ButtonRect,
    pub reset_progress: ButtonRect,
}

pub fn draw_start_screen(font: &Font, screen_w: f32, screen_h: f32) -> StartScreenButtons {
    clear_background(rgb(BG.0, BG.1, BG.2));

    let title = "Solana Mahjong";
    let title_size = 42u16;
    let title_w = measure_text(title, Some(font), title_size, 1.0).width;
    draw_text_ex(
        title,
        (screen_w - title_w) / 2.0,
        screen_h * 0.3,
        TextParams {
            font: Some(font),
            font_size: title_size,
            color: rgb(150, 50, 200),
            ..Default::default()
        },
    );

    let btn_w = 200.0;
    let btn_h = 55.0;
    let btn_gap = 15.0;
    let btn_x = (screen_w - btn_w) / 2.0;
    let start_y = screen_h * 0.5;

    let start = ButtonRect {
        x: btn_x,
        y: start_y,
        w: btn_w,
        h: btn_h,
    };
    let reset = ButtonRect {
        x: btn_x,
        y: start_y + btn_h + btn_gap,
        w: btn_w,
        h: btn_h,
    };

    draw_button_widget(font, "START", &start, rgb(0, 150, 80), rgb(0, 200, 100));
    draw_button_widget(
        font,
        "Reset Progress",
        &reset,
        rgb(150, 50, 50),
        rgb(200, 80, 80),
    );

    StartScreenButtons {
        start,
        reset_progress: reset,
    }
}

pub struct TransitionButtons {
    pub next: ButtonRect,
}

pub fn draw_transition_screen(
    font: &Font,
    screen_w: f32,
    screen_h: f32,
    completed_level: usize,
) -> TransitionButtons {
    clear_background(rgb(BG.0, BG.1, BG.2));

    let text = format!("Level {} Complete!", completed_level);
    let text_size = 36u16;
    let text_w = measure_text(&text, Some(font), text_size, 1.0).width;
    draw_text_ex(
        &text,
        (screen_w - text_w) / 2.0,
        screen_h * 0.35,
        TextParams {
            font: Some(font),
            font_size: text_size,
            color: rgb(0, 200, 100),
            ..Default::default()
        },
    );

    let btn_w = 220.0;
    let btn_h = 55.0;
    let btn_x = (screen_w - btn_w) / 2.0;
    let btn_y = screen_h * 0.55;
    let rect = ButtonRect {
        x: btn_x,
        y: btn_y,
        w: btn_w,
        h: btn_h,
    };

    draw_button_widget(
        font,
        "Next Level",
        &rect,
        rgb(0, 150, 80),
        rgb(0, 200, 100),
    );

    TransitionButtons { next: rect }
}

pub struct CompletionButtons {
    pub start_again: ButtonRect,
    pub exit: ButtonRect,
}

pub fn draw_completion_screen(font: &Font, screen_w: f32, screen_h: f32) -> CompletionButtons {
    clear_background(rgb(BG.0, BG.1, BG.2));

    let line1 = "Congratulations!";
    let line2 = "You completed all levels.";

    let s1 = 40u16;
    let w1 = measure_text(line1, Some(font), s1, 1.0).width;
    draw_text_ex(
        line1,
        (screen_w - w1) / 2.0,
        screen_h * 0.3,
        TextParams {
            font: Some(font),
            font_size: s1,
            color: rgb(255, 215, 0),
            ..Default::default()
        },
    );

    let s2 = 26u16;
    let w2 = measure_text(line2, Some(font), s2, 1.0).width;
    draw_text_ex(
        line2,
        (screen_w - w2) / 2.0,
        screen_h * 0.4,
        TextParams {
            font: Some(font),
            font_size: s2,
            color: WHITE,
            ..Default::default()
        },
    );

    let btn_w = 200.0;
    let btn_h = 50.0;
    let btn_gap = 20.0;
    let total = btn_w * 2.0 + btn_gap;
    let start_x = (screen_w - total) / 2.0;
    let btn_y = screen_h * 0.55;

    let start_again = ButtonRect {
        x: start_x,
        y: btn_y,
        w: btn_w,
        h: btn_h,
    };
    let exit = ButtonRect {
        x: start_x + btn_w + btn_gap,
        y: btn_y,
        w: btn_w,
        h: btn_h,
    };

    draw_button_widget(
        font,
        "Start Again",
        &start_again,
        rgb(0, 150, 80),
        rgb(0, 200, 100),
    );
    draw_button_widget(font, "Exit", &exit, rgb(150, 50, 50), rgb(200, 80, 80));

    CompletionButtons { start_again, exit }
}
