/// The trait your game simulation must implement to interface with RMercury.
pub trait RMercuryGameInterface<TGameState, TGameInput> {
    fn save_game_state() {}
    fn load_game_state() {}

    /// Log the game state. Used for debugging purposes.
    fn log_game_state() {}

    /// Advance the frame with the given inputs.
    fn advance_frame(&mut self, inputs: Vec<TGameInput>);

    /// Retrieve the current game state.
    fn current_game_state(&self) -> TGameState;
}
