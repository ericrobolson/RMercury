use super::*;
use std::marker::PhantomData;

/// A builder for starting an RMercury session. Requires a game interface to execute game logic, the type for inputs, as well as the type for game states.
pub struct RMercuryBuilder<'a, TGameInterface, TGameInput, TGameState>
where
    TGameInterface: RMercuryGameInterface<TGameState, TGameInput>,
    TGameInput: RMercuryInput,
    TGameInput: Copy,
{
    /// The type of session to build
    m_type: MercuryType,
    /// The number of players for the session
    number_of_players: usize,
    /// The number of spectators to allow
    number_of_spectators: usize,
    /// The number of frames to delay local input
    local_input_delay: usize,
    /// The number of times to advance the game per second    
    sim_executions_per_second: usize,
    /// The game interface RMercury will interact with.
    game_interface: &'a mut TGameInterface,
    /// Whether the builder was consumed or not.
    was_built: bool,
    phantom_input: PhantomData<&'a TGameInput>,
    phantom_state: PhantomData<TGameState>,
}

const DEFAULT_NUM_PLAYERS: usize = 2;
const DEFAULT_NUM_SPECTATORS: usize = 4;
const DEFAULT_LOCAL_INPUT_DELAY: usize = 3;
const DEFAULT_SIM_EXECUTIONS_PER_SECOND: usize = 60;

impl<'a, TGameInterface, TGameInput, TGameState>
    RMercuryBuilder<'a, TGameInterface, TGameInput, TGameState>
where
    TGameInterface: RMercuryGameInterface<TGameState, TGameInput>,
    TGameInput: RMercuryInput,
    TGameInput: Copy,
{
    /// Create a new RMercuryBuilder to initialize the network settings.
    pub fn new(game_interface: &'a mut TGameInterface) -> Self {
        return Self {
            m_type: MercuryType::Peer2Peer,
            number_of_players: DEFAULT_NUM_PLAYERS,
            number_of_spectators: DEFAULT_NUM_SPECTATORS,
            local_input_delay: DEFAULT_LOCAL_INPUT_DELAY,
            sim_executions_per_second: DEFAULT_SIM_EXECUTIONS_PER_SECOND,
            game_interface: game_interface,
            was_built: false,
            phantom_input: PhantomData,
            phantom_state: PhantomData,
        };
    }

    /// Sets the type of the network session.
    pub fn with_type(mut self, m_type: MercuryType) -> Self {
        self.m_type = m_type;

        return self;
    }

    /// Sets the number of players for the network session.
    pub fn with_players(mut self, num_players: usize) -> Self {
        if num_players > 0 {
            self.number_of_players = num_players;
        }
        return self;
    }

    /// Sets the number of spectators allowed for the network session.
    pub fn with_spectators(mut self, num_spectators: usize) -> Self {
        self.number_of_spectators = num_spectators;
        return self;
    }

    /// Sets the number of players for the network session.
    pub fn with_local_input_delay(mut self, local_input_delay: usize) -> Self {
        self.local_input_delay = local_input_delay;
        return self;
    }

    /// Sets the number of frames to execute per second for the simulation. Minimum of 1.
    pub fn with_sim_executions_per_second(mut self, hz: usize) -> Self {
        if hz > 0 {
            self.sim_executions_per_second = hz;
        }
        return self;
    }

    /// Build the configured RMercury instance.
    pub fn build(&mut self) -> RMercury<TGameInterface, TGameInput, TGameState> {
        if self.was_built {
            panic!("Builder already consumed!");
        }

        self.was_built = true;
        let rm = RMercury::<TGameInterface, TGameInput, TGameState>::new(
            self.m_type,
            self.number_of_players,
            self.number_of_spectators,
            self.sim_executions_per_second,
            self.local_input_delay,
            self.game_interface,
        );
        return rm;
    }
}
