use super::*;

#[derive(Copy, Clone, Debug, PartialEq)]
/// The various states a RMercury session can be run in.
pub enum MercuryType {
    /// Start a new Peer 2 Peer session
    Peer2Peer,
    /// Start spectating a game in progress
    Spectator,
    /// Start a session where each frame is rolled back, and then the serialized game state is checked against it. Sucessful runs means no non-deterministic behavior was introduced.
    SyncTest,
    /// Start a replay of a previous match
    Replay,
    /// Start a session where packet loss is simulated to extremes.
    PacketLoss,
}

/// RMercury session object. This is the interface that your main loop must use.
pub struct RMercury<'a, TGameInterface, TGameInput, TGameState>
where
    TGameInterface: RMercuryGameInterface<TGameState, TGameInput>,
    TGameInput: RMercuryInput,
    TGameInput: Copy,
{
    pub m_type: MercuryType,
    number_of_players: usize,
    max_spectators: usize,
    sim_executions_per_second: usize,
    local_input_frame_delay: usize,
    game_interface: &'a TGameInterface,
    current_frame: usize,
    inputs: Vec<RMercuryInputWrapper<TGameInput>>,
    last_confirmed_game_state: TGameState,
}

impl<'a, TGameInterface, TGameInput, TGameState>
    RMercury<'a, TGameInterface, TGameInput, TGameState>
where
    TGameInterface: RMercuryGameInterface<TGameState, TGameInput>,
    TGameInput: RMercuryInput,
    TGameInput: Copy,
{
    /// Initialize a new RMercury session.
    pub fn new(
        m_type: MercuryType,
        num_players: usize,
        max_spectators: usize,
        sim_executions_per_second: usize,
        local_input_frame_delay: usize,
        game_interface: &'a TGameInterface,
    ) -> Self {
        let initial_game_state = game_interface.save_game_state();

        return Self {
            m_type: m_type,
            number_of_players: num_players,
            max_spectators: max_spectators,
            sim_executions_per_second: sim_executions_per_second,
            local_input_frame_delay: local_input_frame_delay,
            game_interface: game_interface,
            inputs: vec![],
            current_frame: 0,
            last_confirmed_game_state: initial_game_state,
        };
    }

    pub fn get_local_player_id(&self) -> usize {
        unimplemented!();
    }

    /// Add the local player's input to the queue.
    pub fn add_local_input(&mut self, inputs: &mut Vec<TGameInput>) {
        let frame = self.current_frame + self.local_input_frame_delay;
        let local_player_id = self.get_local_player_id();

        let mut wrapped_inputs: Vec<RMercuryInputWrapper<TGameInput>> = inputs
            .iter_mut()
            .map(|i| {
                // Set the local inputs to execute in the future
                let mut input = i.clone();
                input.set_player_id(local_player_id);

                let wrapped_input = RMercuryInputWrapper::new(input, frame);

                return wrapped_input;
            })
            .collect();

        self.inputs.append(&mut wrapped_inputs);
    }

    /// Execute RMercury. If enough time has passed, will execute the simulation. Otherwise will process outstanding network operations.
    pub fn execute(&mut self) {
        unimplemented!();
    }

    /// Retrieve the current game state. Used for non-simulation purposes, such as audio or rendering.
    pub fn get_game_state(&self) -> TGameState {
        return self.game_interface.current_game_state();
    }
}

struct RMercuryInputWrapper<TGameInput>
where
    TGameInput: RMercuryInput,
{
    /// The input to execute
    input: TGameInput,
    /// The frame the input will be executed for
    frame: usize,
}

impl<TGameInput> RMercuryInputWrapper<TGameInput>
where
    TGameInput: RMercuryInput,
{
    pub fn new(input: TGameInput, frame: usize) -> Self {
        return Self {
            frame: frame,
            input: input,
        };
    }
}
