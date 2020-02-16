use crate::rsystem;
use rsystem::{RErrorCode, RGame, RPlayer, RPlayerHandle};

use super::*;

pub struct SpectatorBackend {}

impl RBackend for SpectatorBackend {
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

impl SpectatorBackend {
    pub fn new(
        r_game: impl RGame,
        app_name: String,
        num_players: u8,
        input_size: i32,
        localport: u8,
        host_ip: String,
        host_port: u8,
    ) -> Self {
        let mut spectator = SpectatorBackend {};
        return spectator;
    }
}
