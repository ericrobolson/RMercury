use crate::rbackend;
use rbackend::RBackend;

pub const RMAXPLAYERS: u8 = 4;
pub const RMAXPREDICTIONFRAMES: u8 = 8;
pub const RMAXSPECTATORS: u8 = 32;
pub const RSPECTATORINPUTINTERVAL: u8 = 4;

pub type RPort = u8;
pub type RIpAddress = String;

pub type RPlayerHandle = i32;
pub type RPlayerNumber = i32;

pub enum RPlayerType {
    Local,
    Remote(RIpAddress, RPort),
    Spectator(RIpAddress, RPort),
}

pub struct RPlayer {
    pub size: i32,
    pub player_type: RPlayerType,
    pub player_number: RPlayerNumber,
}

pub struct RLocalEndpoint {
    pub player_number: RPlayerNumber,
}

pub enum RErrorCode {
    Ok,
    Success,
    GeneralFailure,
    InvalidSession,
    InvalidPlayerHandle,
    PlayerOutOfRange,
    PredictionThreshold,
    Unsupported,
    NotSynchronized,
    InRollback,
    InputDropped,
    PlayerDisconnected,
    TooManySpectators,
    InvalidRequest,
}

pub enum REventCode {
    ConnectedToPeer,
    SynchronizingWithPeer,
    SynchronizedWithPeer,
    Running,
    DisconnectedFromPeer,
    TimeSync,
    ConnectionInterrupted,
    ConnectionResumed,
}

//TODO: fill out
pub struct REvent {
    pub code: REventCode,
    pub connected: Option<RPlayerHandle>,
}

pub trait RGame {
    fn begin_game() -> bool;
    fn save_game_state() -> bool;
    fn load_game_state() -> bool;
    fn log_game_state() -> bool;
    fn free_buffer();
    fn advance_frame() -> bool;
    fn on_event() -> bool;
}

pub trait RSession {
    fn start_session(
        r_game: impl RGame,
        app_name: String,
        num_players: u8,
        input_size: i32,
        localport: u8,
    ) -> Self;

    fn add_player(&mut self, player: RPlayer, player_handle: RPlayerHandle) -> RErrorCode;
}

pub struct RMercury<TBackend: rbackend::RBackend> {
    pub backend: TBackend,
}

impl RSession for RMercury<rbackend::Peer2Peer> {
    fn start_session(
        r_game: impl RGame,
        app_name: String,
        num_players: u8,
        input_size: i32,
        localport: u8,
    ) -> Self {
        let mut backend =
            rbackend::Peer2Peer::new(r_game, app_name, num_players, input_size, localport);

        return Self { backend: backend };
    }

    fn add_player(&mut self, player: RPlayer, player_handle: RPlayerHandle) -> RErrorCode {
        return self.backend.AddPlayer(player, player_handle);
    }
}
/*
impl RMercury {
    pub fn start_synctest() -> RErrorCode {
        panic!();
    }

    pub fn start_spectation() -> RErrorCode {
        panic!();
    }

    pub fn close_session() -> RErrorCode {
        panic!();
    }

    pub fn set_frame_delay() -> RErrorCode {
        panic!();
    }

    pub fn idle() -> RErrorCode {
        panic!();
    }

    pub fn add_local_input() -> RErrorCode {
        panic!();
    }

    pub fn synchronize_input() -> RErrorCode {
        panic!();
    }

    pub fn disconnect_player() -> RErrorCode {
        panic!();
    }

    pub fn advance_frame() -> RErrorCode {
        panic!();
    }

    pub fn get_nework_stats() -> RErrorCode {
        panic!();
    }

    pub fn set_disconnect_timeout() -> RErrorCode {
        panic!();
    }

    pub fn set_disconnect_notify_start() -> RErrorCode {
        panic!();
    }

    pub fn log() -> RErrorCode {
        panic!();
    }
}
*/
