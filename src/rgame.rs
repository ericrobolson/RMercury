/*
 * Trait used for RMercury to sucessfully network, calculate game states, and rollback any discrepancies between player inputs while replaying them with the correct inputs.
 */
pub trait RGame {
    /*
     * Begin game function. Success returns true, false denotes an error.
     */
    fn begin_game() -> bool;
    /*
     * The client should copy the contents of the current game state into the buffer, as well as optionally provide a checksum of the data.
     */
    fn save_game_state() -> bool;
    /*
     * Called at the beggining of a rollback. Loads the last previously confirmed gamestate. Client should replace actual gamestate with this loaded gamestate.
     */
    fn load_game_state() -> bool;
    /*
     * Used in diagnostic testing. Client should use this the Log function to write the contents of the specified state in a human readible form.
     */
    fn log_game_state() -> bool;
    /*
     * Frees the gamestate allocated in the save_game_state().
     */
    fn free_buffer();
    /*
     * Called during a rollback. Advances that gamestate by exactly one frame. Before each frame, call synchronize_input to retrieve the inputs to use for that frame.
     */
    fn advance_frame() -> bool;
    /*
     * Notification that something has happened. See the RErrorCodes for more information.
     */
    fn on_event() -> bool;
}
