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

enum Screen {
    Start,
    Game,
    Transition { completed_level: usize },
    Completion,
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

        Self {
            state,
            renderer: Renderer::new(),
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
            Action::SelectTile(tile_id) => {
                if let Some((_a, _b)) = game_core::try_select_tile(&mut self.state, tile_id) {
                }
                if self.state.phase == GamePhase::LevelCompleted {
                    if self.state.current_level >= levels::total_levels() {
                        self.screen = Screen::Completion;
                    } else {
                        self.screen = Screen::Transition {
                            completed_level: self.state.current_level,
                        };
                    }
                }
            }
        }
    }
}

#[macroquad::main("Mahjong")]
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
                let ui = renderer::draw::draw_game_screen(
                    &app.renderer,
                    &app.state,
                    &app.theme,
                    &app.font,
                    screen_w,
                    screen_h,
                );

                if ui.restart.contains(mouse_position().0, mouse_position().1)
                    && is_mouse_button_pressed(MouseButton::Left)
                {
                    app.handle_action(Action::RestartLevel);
                }
                if ui.exit.contains(mouse_position().0, mouse_position().1)
                    && is_mouse_button_pressed(MouseButton::Left)
                {
                    app.handle_action(Action::Exit);
                }

                if is_mouse_button_pressed(MouseButton::Left) {
                    let (mx, my) = mouse_position();
                    if let Some(tile_id) = app.renderer.get_tile_at(mx, my) {
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
        }

        next_frame().await;
    }
}

#[cfg(target_os = "android")]
#[no_mangle]
pub extern "C" fn quad_main() {
    main();
}
