use crate::rsystem;
use rsystem::{RErrorCode, RGame, RPlayer, RPlayerHandle};

use crate::rbackend;
use rbackend::RBackend;

pub struct SyncTest {}

impl RBackend for SyncTest {
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

impl SyncTest {
    pub fn new(
        r_game: impl RGame,
        app_name: String,
        num_players: u8,
        input_size: i32,
        frames: i32,
    ) -> Self {
        let mut sync_test = SyncTest {};
        return sync_test;
    }
}
