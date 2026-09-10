use macroquad::prelude::*;

use crate::game_core::GameState;
use crate::renderer::{Renderer, TileRenderInfo};
use crate::theme::Theme;

fn rgb(r: u8, g: u8, b: u8) -> Color {
    Color::from_rgba(r, g, b, 255)
}

fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
    Color::from_rgba(r, g, b, a)
}

pub struct UiButtons {
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
}

pub fn draw_game_screen(
    renderer: &Renderer,
    state: &GameState,
    theme: &Theme,
    font: &Font,
    screen_w: f32,
    screen_h: f32,
) -> UiButtons {
    clear_background(WHITE);

    let level_text = format!("Level {}", state.current_level);
    let level_w = measure_text(&level_text, Some(font), 30u16, 1.0).width;
    draw_text_ex(
        &level_text,
        (screen_w - level_w) / 2.0,
        32.0,
        TextParams {
            font: Some(font),
            font_size: 30,
            color: rgb(40, 40, 55),
            ..Default::default()
        },
    );

    let remaining = state.active_tile_count();
    let rem_text = format!("Remaining: {}", remaining);
    let rem_w = measure_text(&rem_text, Some(font), 18u16, 1.0).width;
    draw_text_ex(
        &rem_text,
        (screen_w - rem_w) / 2.0,
        58.0,
        TextParams {
            font: Some(font),
            font_size: 18,
            color: rgb(110, 110, 120),
            ..Default::default()
        },
    );

    for info in &renderer.tile_positions {
        draw_tile(info, state, theme, font);
    }

    let btn_h = 35.0f32;
    let btn_gap = 10.0f32;
    let restart_text = "Restart";
    let exit_text = "Exit";
    let rw = measure_text(restart_text, Some(font), 22u16, 1.0).width + 24.0;
    let ew = measure_text(exit_text, Some(font), 22u16, 1.0).width + 24.0;
    let total_w = rw + btn_gap + ew;
    let btn_x = (screen_w - total_w) / 2.0;
    let btn_y = screen_h - btn_h - 10.0;

    let restart_rect = ButtonRect {
        x: btn_x,
        y: btn_y,
        w: rw,
        h: btn_h,
    };
    let exit_rect = ButtonRect {
        x: btn_x + rw + btn_gap,
        y: btn_y,
        w: ew,
        h: btn_h,
    };

    draw_button_widget(font, restart_text, &restart_rect, GRAY, DARKGRAY);
    draw_button_widget(font, exit_text, &exit_rect, GRAY, DARKGRAY);

    UiButtons {
        restart: restart_rect,
        exit: exit_rect,
    }
}

fn draw_tile(info: &TileRenderInfo, state: &GameState, theme: &Theme, font: &Font) {
    let tile = match state.get_tile(info.tile_id) {
        Some(t) => t,
        None => return,
    };

    let tile_type = theme.tile_types.iter().find(|t| t.id == tile.type_id);
    let color = tile_type
        .map(|t| rgb(t.color.0, t.color.1, t.color.2))
        .unwrap_or(GRAY);

    let is_selected = state.selected_tile_id == Some(info.tile_id);

    let x = info.screen_x;
    let y = info.screen_y;
    let w = info.width;
    let h = info.height;

    // Cast shadow for depth
    let shadow_offset = 3.0f32;
    draw_rectangle(
        x + shadow_offset,
        y + shadow_offset,
        w,
        h,
        rgb(10, 10, 15),
    );

    // Chunky 3D bevel: darker bottom/right edge
    let edge = 3.0f32.min(h * 0.22);
    let dark = rgb(20, 20, 28);
    draw_rectangle(x + w - edge, y + edge, edge, h - edge, dark);
    draw_rectangle(x + edge, y + h - edge, w - edge, edge, dark);

    let base_color = if is_selected {
        let r = tile_type.map(|t| (t.color.0 as u16 + 60).min(255) as u8).unwrap_or(200);
        let g = tile_type.map(|t| (t.color.1 as u16 + 60).min(255) as u8).unwrap_or(200);
        let b = tile_type.map(|t| (t.color.2 as u16 + 60).min(255) as u8).unwrap_or(200);
        rgb(r, g, b)
    } else {
        color
    };

    // Tile face
    draw_rectangle(x + edge, y, w - edge, h - edge, base_color);

    if is_selected {
        draw_rectangle_lines(x - 1.0, y - 1.0, w + 2.0, h + 2.0, 2.0, YELLOW);
    }

    // Face highlight
    draw_rectangle(x + edge, y, w - edge, h * 0.18, rgba(255, 255, 255, 45));
    draw_rectangle_lines(x, y, w, h, 1.0, rgba(0, 0, 0, 90));

    if let Some(tt) = tile_type {
        let name = tt.name;

        if h >= 16.0 {
            let icon_half = (w.min(h) * 0.36).min(26.0).max(4.0);
            let icon_cx = x + (w - edge) / 2.0;
            let icon_cy = y + h * 0.38;
            crate::renderer::icons::draw_tile_icon(name, icon_cx, icon_cy, icon_half);
        }

        if h >= 10.0 {
            let font_size = (h * 0.22).min(17.0).max(7.0) as u16;
            let text_w = measure_text(name, Some(font), font_size, 1.0).width;
            let text_x = x + (w - text_w) / 2.0;
            let text_y = y + h * 0.82 + font_size as f32 * 0.4;
            draw_text_ex(
                name,
                text_x,
                text_y,
                TextParams {
                    font: Some(font),
                    font_size,
                    color: WHITE,
                    ..Default::default()
                },
            );
        }
    }
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
    clear_background(rgb(20, 20, 30));

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
    clear_background(rgb(20, 20, 30));

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
    clear_background(rgb(20, 20, 30));

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
