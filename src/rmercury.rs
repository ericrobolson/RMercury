use super::*;
use std::marker::PhantomData;

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

pub struct RMercury<'a, TGameInterface, TGameInput, TGameState>
where
    TGameInterface: RMercuryGameInterface<TGameState, TGameInput>,
    TGameInput: RMercuryInput,
{
    pub m_type: MercuryType,
    number_of_players: usize,
    max_spectators: usize,
    sim_executions_per_second: usize,
    local_input_frame_delay: usize,
    game_interface: &'a TGameInterface,
    inputs: Vec<TGameInput>,
    phantom_state: PhantomData<TGameState>,
}

impl<'a, TGameInterface, TGameInput, TGameState>
    RMercury<'a, TGameInterface, TGameInput, TGameState>
where
    TGameInterface: RMercuryGameInterface<TGameState, TGameInput>,
    TGameInput: RMercuryInput,
{
    pub fn new(
        m_type: MercuryType,
        num_players: usize,
        max_spectators: usize,
        sim_executions_per_second: usize,
        local_input_frame_delay: usize,
        game_interface: &'a TGameInterface,
    ) -> Self {
        return Self {
            m_type: m_type,
            number_of_players: num_players,
            max_spectators: max_spectators,
            sim_executions_per_second: sim_executions_per_second,
            local_input_frame_delay: local_input_frame_delay,
            game_interface: game_interface,
            inputs: vec![],
            phantom_state: PhantomData,
        };
    }

    pub fn log_local_input(&mut self, inputs: Vec<TGameInput>) {
        unimplemented!();
    }
    pub fn execute(&mut self) {
        unimplemented!();
    }

    pub fn get_game_state(&self) -> TGameState {
        return self.game_interface.current_game_state();
    }

    pub fn game_interface(&self) -> &TGameInterface {
        return self.game_interface;
    }
}
