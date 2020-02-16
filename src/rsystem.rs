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

pub struct RNetworkStats {}

pub trait RSession {
    fn add_player(&mut self, player: RPlayer, player_handle: RPlayerHandle) -> RErrorCode;

    //TODO: start synctest
    //TODO: logging?

    fn set_frame_delay(&mut self, player_handle: RPlayerHandle, frame_delay: i32) -> RErrorCode;

    fn idle(&mut self, timeout: i32) -> RErrorCode;

    fn add_local_input(&mut self, player_handle: RPlayerHandle) -> RErrorCode;

    fn synchronize_input(&mut self) -> RErrorCode;

    fn advance_frame(&mut self) -> RErrorCode;

    fn client_chat(&mut self, text: String) -> RErrorCode;

    fn get_network_stats(
        &self,
        player_handle: RPlayerHandle,
    ) -> (RErrorCode, Option<RNetworkStats>);

    fn close_session(&mut self) -> RErrorCode;

    fn set_disconnect_timeout(&mut self, timeout: i32) -> RErrorCode;

    fn set_disconnect_notify_start(&mut self, timeout: i32) -> RErrorCode;
}

pub struct RMercury<TBackend: rbackend::RBackend> {
    pub backend: TBackend,
}

pub fn RMercury_StartSession(
    r_game: impl RGame,
    app_name: String,
    num_players: u8,
    input_size: i32,
    localport: u8,
) -> RMercury<rbackend::Peer2Peer> {
    let backend = rbackend::Peer2Peer::new(r_game, app_name, num_players, input_size, localport);

    return RMercury { backend: backend };
}

impl<T> RSession for RMercury<T>
where
    T: RBackend,
{
    fn add_player(&mut self, player: RPlayer, player_handle: RPlayerHandle) -> RErrorCode {
        return self.backend.AddPlayer(player, player_handle);
    }

    fn set_frame_delay(&mut self, player_handle: RPlayerHandle, frame_delay: i32) -> RErrorCode {
        return self.backend.SetFrameDelay(player_handle, frame_delay);
    }

    fn idle(&mut self, timeout: i32) -> RErrorCode {
        return self.backend.DoPoll(timeout);
    }

    fn add_local_input(&mut self, player_handle: RPlayerHandle) -> RErrorCode {
        return self.backend.AddLocalInput(player_handle);
    }

    fn synchronize_input(&mut self) -> RErrorCode {
        return self.backend.SyncInput();
    }

    fn advance_frame(&mut self) -> RErrorCode {
        return self.backend.IncrementFrame();
    }

    fn client_chat(&mut self, text: String) -> RErrorCode {
        return self.backend.Chat(text);
    }

    fn get_network_stats(
        &self,
        player_handle: RPlayerHandle,
    ) -> (RErrorCode, Option<RNetworkStats>) {
        return self.backend.GetNetworkStats(player_handle);
    }

    fn close_session(&mut self) -> RErrorCode {
        return self.backend.CloseSession();
    }

    fn set_disconnect_timeout(&mut self, timeout: i32) -> RErrorCode {
        return self.backend.SetDisconnectTimeout(timeout);
    }

    fn set_disconnect_notify_start(&mut self, timeout: i32) -> RErrorCode {
        return self.backend.SetDisconnectNotifyStart(timeout);
    }
}
