use macroquad::prelude::*;

mod game_core;
mod input;
mod levels;
mod renderer;
mod theme;

use game_core::*;
use input::Action;
use renderer::Renderer;
use theme::Theme;

fn window_conf() -> Conf {
    Conf {
        window_title: "Solana Mahjong".to_owned(),
        window_width: 1280,
        window_height: 720,
        high_dpi: true,
        ..Default::default()
    }
}

enum Screen {
    Start,
    Game,
    Transition { completed_level: usize },
    Completion,
    GameOver,
}

struct App {
    state: GameState,
    renderer: Renderer,
    theme: Theme,
    font: Font,
    screen: Screen,
}

impl App {
    async fn new() -> Self {
        let font = match load_ttf_font("C:\\Windows\\Fonts\\arial.ttf").await {
            Ok(f) => f,
            Err(_) => get_default_font(),
        };

        let theme = theme::get_theme();
        let current_level = game_core::load_progress();

        let mut state = GameState::new();
        state.current_level = current_level;

        let mut renderer = Renderer::new();
        renderer.load_tile_textures().await;

        Self {
            state,
            renderer,
            theme,
            font,
            screen: Screen::Start,
        }
    }

    fn start_game(&mut self) {
        self.load_current_level();
        self.screen = Screen::Game;
    }

    fn load_current_level(&mut self) {
        self.renderer.clear_fades();
        if let Some(level_data) = levels::load_level(self.state.current_level) {
            self.state.load_level(self.state.current_level, &level_data);
        }
    }

    fn handle_action(&mut self, action: Action) {
        match action {
            Action::StartGame => self.start_game(),
            Action::RestartLevel => {
                self.load_current_level();
                self.screen = Screen::Game;
            }
            Action::NextLevel => {
                if self.state.current_level < levels::total_levels() {
                    self.state.current_level += 1;
                    game_core::save_progress(self.state.current_level);
                    self.load_current_level();
                    self.screen = Screen::Game;
                }
            }
            Action::ResetProgress => {
                game_core::reset_progress();
                self.state.current_level = 1;
                self.screen = Screen::Start;
            }
            Action::StartAgain => {
                game_core::save_progress(1);
                self.state.current_level = 1;
                self.start_game();
            }
            Action::Exit => std::process::exit(0),
            Action::Undo => {
                self.renderer.clear_fades();
                game_core::do_undo(&mut self.state);
                // Leaving GameOver via undo returns to the board.
                if let Screen::GameOver = self.screen {
                    self.screen = Screen::Game;
                }
            }
            Action::SelectTile(tile_id) => {
                let was_paired = game_core::buffer_click(&mut self.state, tile_id);
                // A tile still in the buffer after the click means it was moved
                // off the field; a shared tile_id means the pair was cleared.
                if was_paired || self.state.buffer.contains(&tile_id) {
                    self.renderer.start_fade_of_tile(tile_id);
                }
                match self.state.phase {
                    GamePhase::LevelCompleted => {
                        if self.state.current_level >= levels::total_levels() {
                            self.screen = Screen::Completion;
                        } else {
                            self.screen = Screen::Transition {
                                completed_level: self.state.current_level,
                            };
                        }
                    }
                    GamePhase::Lost => {
                        self.screen = Screen::GameOver;
                    }
                    _ => {}
                }
            }
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut app = App::new().await;

    loop {
        let screen_w = screen_width();
        let screen_h = screen_height();

        match &app.screen {
            Screen::Start => {
                let ui = renderer::draw::draw_start_screen(&app.font, screen_w, screen_h);
                if ui.start.contains(mouse_position().0, mouse_position().1)
                    && is_mouse_button_pressed(MouseButton::Left)
                {
                    app.handle_action(Action::StartGame);
                }
                if ui.reset_progress.contains(mouse_position().0, mouse_position().1)
                    && is_mouse_button_pressed(MouseButton::Left)
                {
                    app.handle_action(Action::ResetProgress);
                }
            }
            Screen::Game => {
                app.renderer.compute_layout(&app.state, screen_w, screen_h);
                let elapsed = (macroquad::time::get_time() - app.state.level_start_sec)
                    .max(0.0) as u64;
                let ui = renderer::draw::draw_game_screen(
                    &mut app.renderer,
                    &app.state,
                    &app.theme,
                    &app.font,
                    screen_w,
                    screen_h,
                    elapsed,
                );

                if is_mouse_button_pressed(MouseButton::Left) {
                    let (mx, my) = mouse_position();
                    if ui.undo.contains(mx, my) {
                        app.handle_action(Action::Undo);
                    } else if ui.restart.contains(mx, my) {
                        app.handle_action(Action::RestartLevel);
                    } else if ui.exit.contains(mx, my) {
                        app.handle_action(Action::Exit);
                    } else if let Some(tile_id) = app.renderer.get_tile_at(mx, my) {
                        app.handle_action(Action::SelectTile(tile_id));
                    }
                }
            }
            Screen::Transition { completed_level } => {
                let ui = renderer::draw::draw_transition_screen(
                    &app.font,
                    screen_w,
                    screen_h,
                    *completed_level,
                );
                if ui.next.contains(mouse_position().0, mouse_position().1)
                    && is_mouse_button_pressed(MouseButton::Left)
                {
                    app.handle_action(Action::NextLevel);
                }
            }
            Screen::Completion => {
                let ui = renderer::draw::draw_completion_screen(&app.font, screen_w, screen_h);
                if ui.start_again.contains(mouse_position().0, mouse_position().1)
                    && is_mouse_button_pressed(MouseButton::Left)
                {
                    app.handle_action(Action::StartAgain);
                }
                if ui.exit.contains(mouse_position().0, mouse_position().1)
                    && is_mouse_button_pressed(MouseButton::Left)
                {
                    app.handle_action(Action::Exit);
                }
            }
            Screen::GameOver => {
                let ui = renderer::draw::draw_game_over_screen(&app.font, screen_w, screen_h);
                if ui.restart.contains(mouse_position().0, mouse_position().1)
                    && is_mouse_button_pressed(MouseButton::Left)
                {
                    app.handle_action(Action::RestartLevel);
                }
                if ui.undo.contains(mouse_position().0, mouse_position().1)
                    && is_mouse_button_pressed(MouseButton::Left)
                {
                    app.handle_action(Action::Undo);
                }
                if ui.exit.contains(mouse_position().0, mouse_position().1)
                    && is_mouse_button_pressed(MouseButton::Left)
                {
                    app.handle_action(Action::Exit);
                }
            }
        }

        next_frame().await;
    }
}

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "C" fn quad_main() {
    main();
}
