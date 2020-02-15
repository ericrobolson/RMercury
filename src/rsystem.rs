pub const RMAXPLAYERS: u8 = 4;
pub const RMAXPREDICTIONFRAMES: u8 = 8;
pub const RMAXSPECTATORS: u8 = 32;
pub const RSPECTATORINPUTINTERVAL: u8 = 4;

pub struct RSession {}

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
