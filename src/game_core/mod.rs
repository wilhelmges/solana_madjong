pub mod state;
pub mod rules;
pub mod progress;

pub use state::*;
pub use rules::*;
pub use progress::*;

/// Wall-clock for HUD timer. In tests returns 0.
pub fn game_time_now() -> f64 {
    #[cfg(test)]
    {
        0.0
    }
    #[cfg(not(test))]
    {
        macroquad::time::get_time()
    }
}
