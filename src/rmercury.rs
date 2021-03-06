use super::*;

use rmercury_channel::RChannelManager;
use rmercury_input::{RMercuryInput, RMercuryInputWrapper};
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
    TGameState: Copy,
    TGameInput: RMercuryInput,
    TGameInput: Copy,
    TGameInput: PartialEq,
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
    last_confirmed_frame: usize,
    frame_duration: time::Duration,
    last_frame_execution: time::Instant,
    channel_manager: RChannelManager<TGameInput>,
}

impl<'a, TGameInterface, TGameInput, TGameState>
    RMercury<'a, TGameInterface, TGameInput, TGameState>
where
    TGameInterface: RMercuryGameInterface<TGameState, TGameInput>,
    TGameInput: RMercuryInput,
    TGameInput: Copy,
    TGameInput: PartialEq,
    TGameState: Copy,
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
            last_confirmed_frame: 0,
            last_confirmed_game_state: initial_game_state,
            frame_duration: frame_duration,
            last_frame_execution: start,
            channel_manager: RChannelManager::new(),
        };
    }

    pub fn get_local_player_id(&self) -> usize {
        //TODO: implement based on connection time. E.g. if one player creates the host first, they get first character.
        return 1;
    }

    /// Get a mutable reference to the game interface.
    pub fn get_game_interface_mut(&mut self) -> &mut TGameInterface {
        return self.game_interface;
    }

    /// Get a non-mutable reference to the game interface.
    pub fn get_game_interface(&self) -> &TGameInterface {
        return self.game_interface;
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

        self.channel_manager.queue_local_input(&wrapped_inputs);

        self.inputs.append(&mut wrapped_inputs);
    }

    /// Whether RMercury is ready to execute. When true, ready to sync inputs and execute.
    pub fn ready_to_run(&self) -> bool {
        let now = self.last_frame_execution - Instant::now();
        let run_game_sim = self.frame_duration <= now;

        return run_game_sim;
    }

    /// Execute RMercury. If enough time has passed, will execute the simulation. Otherwise will process outstanding network operations.
    pub fn execute(&mut self) -> RMercuryExecutionResults {
        // Sync up network
        let should_rollback;
        {
            let mut remote_inputs = self.channel_manager.execute();
            remote_inputs.sort_by(|a, b| b.frame.cmp(&a.frame));

            let earliest_received_input = remote_inputs.first();

            if earliest_received_input.is_some() {
                let earliest_received_input = earliest_received_input.unwrap();
                should_rollback = earliest_received_input.frame < self.current_frame;
            } else {
                should_rollback = false;
            }

            self.inputs.append(&mut remote_inputs);
            self.inputs.sort_by(|a, b| b.frame.cmp(&a.frame));
            self.inputs.dedup();
        }

        let run_game_sim = self.ready_to_run();

        if run_game_sim {
            if should_rollback {
                self.game_interface
                    .load_game_state(self.last_confirmed_game_state.clone());

                let mut rollback_frame = self.last_confirmed_frame;
                while rollback_frame < self.current_frame {
                    let current_frame_inputs = self
                        .inputs
                        .iter()
                        .filter(|x| x.frame == rollback_frame)
                        .map(|x| x.input)
                        .collect();

                    self.game_interface.advance_frame(current_frame_inputs);

                    if rollback_frame == self.channel_manager.last_confirmed_frame() {
                        // Save the confirmed frame
                        let game_state = self.game_interface.current_game_state();
                        self.last_confirmed_game_state = game_state;
                    }

                    rollback_frame += 1;
                }
            }

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
