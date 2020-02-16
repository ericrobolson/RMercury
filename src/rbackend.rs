use crate::rsystem;
use rsystem::{RErrorCode, RGame, RPlayer, RPlayerHandle};

pub trait RBackend {
    /// virtual GGPOErrorCode DoPoll(int timeout) { return GGPO_OK; }
    fn DoPoll(&self, timeout: i32) -> rsystem::RErrorCode;
    /// virtual GGPOErrorCode AddPlayer(GGPOPlayer *player, GGPOPlayerHandle *handle) = 0;
    fn AddPlayer(&mut self, player: RPlayer, player_handle: RPlayerHandle) -> RErrorCode;
    /// virtual GGPOErrorCode AddLocalInput(GGPOPlayerHandle player, void *values, int size) = 0;
    fn AddLocalInput(&mut self, player_handle: RPlayerHandle) -> RErrorCode;
    /// virtual GGPOErrorCode SyncInput(void *values, int size, int *disconnect_flags) = 0;
    fn SyncInput(&mut self) -> RErrorCode;
    /// virtual GGPOErrorCode IncrementFrame(void) { return GGPO_OK; }
    fn IncrementFrame(&mut self) -> RErrorCode;
    /// virtual GGPOErrorCode Chat(char *text) { return GGPO_OK; }
    fn Chat(&mut self, text: String) -> RErrorCode;
    /// virtual GGPOErrorCode DisconnectPlayer(GGPOPlayerHandle handle) { return GGPO_OK; }
    fn DisconnectPlayer(player_handle: RPlayerHandle) -> RErrorCode;
    /// virtual GGPOErrorCode GetNetworkStats(GGPONetworkStats *stats, GGPOPlayerHandle handle) { return GGPO_OK; }
    fn GetNetworkStats(
        &self,
        player_handle: RPlayerHandle,
    ) -> (RErrorCode, Option<rsystem::RNetworkStats>);
    /// virtual GGPOErrorCode SetFrameDelay(GGPOPlayerHandle player, int delay) { return GGPO_ERRORCODE_UNSUPPORTED; }
    fn SetFrameDelay(&mut self, player_handle: RPlayerHandle, frame_delay: i32) -> RErrorCode;
    /// virtual GGPOErrorCode SetDisconnectTimeout(int timeout) { return GGPO_ERRORCODE_UNSUPPORTED; }
    fn SetDisconnectTimeout(&mut self, timeout: i32) -> RErrorCode;
    /// virtual GGPOErrorCode SetDisconnectNotifyStart(int timeout) { return GGPO_ERRORCODE_UNSUPPORTED; }
    fn SetDisconnectNotifyStart(&mut self, timeout: i32) -> RErrorCode;

    fn CloseSession(&mut self) -> RErrorCode;
}

pub struct Peer2Peer {}

impl RBackend for Peer2Peer {
    fn DoPoll(&self, _: i32) -> rsystem::RErrorCode {
        unimplemented!()
    }
    fn AddPlayer(&mut self, _: rsystem::RPlayer, _: i32) -> rsystem::RErrorCode {
        unimplemented!()
    }
    fn AddLocalInput(&mut self, _: i32) -> rsystem::RErrorCode {
        unimplemented!()
    }
    fn SyncInput(&mut self) -> rsystem::RErrorCode {
        unimplemented!()
    }
    fn IncrementFrame(&mut self) -> rsystem::RErrorCode {
        unimplemented!()
    }
    fn Chat(&mut self, _: std::string::String) -> rsystem::RErrorCode {
        unimplemented!()
    }
    fn DisconnectPlayer(_: i32) -> rsystem::RErrorCode {
        unimplemented!()
    }
    fn GetNetworkStats(
        &self,
        player_handle: RPlayerHandle,
    ) -> (RErrorCode, Option<rsystem::RNetworkStats>) {
        unimplemented!()
    }
    fn SetFrameDelay(
        &mut self,
        player_handle: RPlayerHandle,
        frame_delay: i32,
    ) -> rsystem::RErrorCode {
        unimplemented!()
    }
    fn SetDisconnectTimeout(&mut self, timeout: i32) -> rsystem::RErrorCode {
        unimplemented!()
    }

    fn SetDisconnectNotifyStart(&mut self, timeout: i32) -> RErrorCode {
        unimplemented!()
    }

    fn CloseSession(&mut self) -> rsystem::RErrorCode {
        unimplemented!()
    }
}

impl Peer2Peer {
    pub fn new(
        r_game: impl RGame,
        app_name: String,
        num_players: u8,
        input_size: i32,
        localport: u8,
    ) -> Self {
        let mut p2p = Peer2Peer {};
        return p2p;
    }
}
