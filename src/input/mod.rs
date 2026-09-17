#[derive(Debug, Clone)]
pub enum Action {
    SelectTile(usize),
    RestartLevel,
    StartGame,
    NextLevel,
    ResetProgress,
    Exit,
    StartAgain,
    Undo,
}
