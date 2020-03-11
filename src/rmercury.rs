use super::*;

use time::{Duration, Instant};
const MILLISECONDS_IN_SECOND: u64 = 1000;

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

/// Enumeration for the results from the execute method.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RMercuryExecutionResults {
    NotExecuted,
    Executed,
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
    game_interface: &'a mut TGameInterface,
    current_frame: usize,
    inputs: Vec<RMercuryInputWrapper<TGameInput>>,
    last_confirmed_game_state: TGameState,
    frame_duration: time::Duration,
    last_frame_execution: time::Instant,
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
        game_interface: &'a mut TGameInterface,
    ) -> Self {
        let initial_game_state = game_interface.current_game_state();

        let frame_duration = Duration::milliseconds(
            MILLISECONDS_IN_SECOND as i64 / sim_executions_per_second as i64,
        );

        let start = Instant::now();

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
            frame_duration: frame_duration,
            last_frame_execution: start,
        };
    }

    pub fn get_local_player_id(&self) -> usize {
        //TODO: implement
        return 1;
    }

    /// Add the local player's input to the queue.
    pub fn add_local_input(&mut self, inputs: &mut Vec<TGameInput>) {
        let frame_to_execute = self.current_frame + self.local_input_frame_delay;
        let local_player_id = self.get_local_player_id();

        let mut wrapped_inputs: Vec<RMercuryInputWrapper<TGameInput>> = inputs
            .iter_mut()
            .map(|i| {
                // Set the local inputs to execute in the future
                let mut input = i.clone();
                input.set_player_id(local_player_id);

                let wrapped_input = RMercuryInputWrapper::new(input, frame_to_execute);

                return wrapped_input;
            })
            .collect();

        self.inputs.append(&mut wrapped_inputs);
    }

    /// Whether RMecury is ready to execute. When true, ready to sync inputs and execute.
    pub fn ready_to_run(&self) -> bool {
        let now = self.last_frame_execution - Instant::now();
        let run_game_sim = self.frame_duration <= now;

        return run_game_sim;
    }

    /// Execute RMercury. If enough time has passed, will execute the simulation. Otherwise will process outstanding network operations.
    pub fn execute(&mut self) -> RMercuryExecutionResults {
        let run_game_sim = self.ready_to_run();

        if run_game_sim {
            let current_frame_inputs = self
                .inputs
                .iter()
                .filter(|x| x.frame == self.current_frame)
                .map(|x| x.input)
                .collect();

            self.game_interface.advance_frame(current_frame_inputs);

            // TODO: optimization: take all previously confirmed inputs, and persist to disk?
            self.current_frame += 1;
            self.last_frame_execution = Instant::now();

            return RMercuryExecutionResults::Executed;
        }

        return RMercuryExecutionResults::NotExecuted;
    }

    /// Get the current game tick.
    pub fn get_current_tick(&self) -> usize {
        return self.current_frame;
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
    pub input: TGameInput,
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
