use crate::backends;
use backends::{Peer2Peer, RBackend, SpectatorBackend, SyncTest};

use crate::rsystem;
use rsystem::{RErrorCode, RGame, RNetworkStats, RPlayer, RPlayerHandle};

pub trait RSession {
    fn add_player(&mut self, player: RPlayer, player_handle: RPlayerHandle) -> RErrorCode;
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

pub fn rmercury_start_synctest(
    r_game: impl RGame,
    app_name: String,
    num_players: u8,
    input_size: i32,
    frames: i32,
) -> RMercury<SyncTest> {
    let backend = SyncTest::new(r_game, app_name, num_players, input_size, frames);

    return RMercury {
        backend: backend,
        is_closed: false,
    };
}

pub fn rmercury_start_spectator(
    r_game: impl RGame,
    app_name: String,
    num_players: u8,
    input_size: i32,
    localport: u8,
    host_ip: String,
    host_port: u8,
) -> RMercury<SpectatorBackend> {
    let backend = SpectatorBackend::new(
        r_game,
        app_name,
        num_players,
        input_size,
        localport,
        host_ip,
        host_port,
    );

    return RMercury {
        backend: backend,
        is_closed: false,
    };
}

pub fn rmercury_start_session(
    r_game: impl RGame,
    app_name: String,
    num_players: u8,
    input_size: i32,
    localport: u8,
) -> RMercury<Peer2Peer> {
    let backend = Peer2Peer::new(r_game, app_name, num_players, input_size, localport);

    return RMercury {
        backend: backend,
        is_closed: false,
    };
}

pub struct RMercury<TBackend: RBackend> {
    pub backend: TBackend,
    pub is_closed: bool,
}

impl<T> RSession for RMercury<T>
where
    T: RBackend,
{
    fn add_player(&mut self, player: RPlayer, player_handle: RPlayerHandle) -> RErrorCode {
        if self.is_closed {
            return RErrorCode::InvalidSession;
        }

        return self.backend.AddPlayer(player, player_handle);
    }

    fn set_frame_delay(&mut self, player_handle: RPlayerHandle, frame_delay: i32) -> RErrorCode {
        if self.is_closed {
            return RErrorCode::InvalidSession;
        }

        return self.backend.SetFrameDelay(player_handle, frame_delay);
    }

    fn idle(&mut self, timeout: i32) -> RErrorCode {
        if self.is_closed {
            return RErrorCode::InvalidSession;
        }
        return self.backend.DoPoll(timeout);
    }

    fn add_local_input(&mut self, player_handle: RPlayerHandle) -> RErrorCode {
        if self.is_closed {
            return RErrorCode::InvalidSession;
        }
        return self.backend.AddLocalInput(player_handle);
    }

    fn synchronize_input(&mut self) -> RErrorCode {
        if self.is_closed {
            return RErrorCode::InvalidSession;
        }
        return self.backend.SyncInput();
    }

    fn advance_frame(&mut self) -> RErrorCode {
        if self.is_closed {
            return RErrorCode::InvalidSession;
        }
        return self.backend.IncrementFrame();
    }

    fn client_chat(&mut self, text: String) -> RErrorCode {
        if self.is_closed {
            return RErrorCode::InvalidSession;
        }
        return self.backend.Chat(text);
    }

    fn get_network_stats(
        &self,
        player_handle: RPlayerHandle,
    ) -> (RErrorCode, Option<RNetworkStats>) {
        if self.is_closed {
            return (RErrorCode::InvalidSession, None);
        }
        return self.backend.GetNetworkStats(player_handle);
    }

    fn close_session(&mut self) -> RErrorCode {
        if self.is_closed {
            return RErrorCode::InvalidSession;
        }

        self.is_closed = true;
        return self.backend.CloseSession();
    }

    fn set_disconnect_timeout(&mut self, timeout: i32) -> RErrorCode {
        if self.is_closed {
            return RErrorCode::InvalidSession;
        }
        return self.backend.SetDisconnectTimeout(timeout);
    }

    fn set_disconnect_notify_start(&mut self, timeout: i32) -> RErrorCode {
        if self.is_closed {
            return RErrorCode::InvalidSession;
        }
        return self.backend.SetDisconnectNotifyStart(timeout);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn rmercury_test() -> RMercury<MockRBackend> {
        let backend = MockRBackend::new();

        return RMercury {
            backend: backend,
            is_closed: false,
        };
    }

    #[test]
    fn RMercury_T_CloseSession_SetsClosedToTrue() {
        let mut rmercury = rmercury_test();

        let actual = rmercury.close_session();
        let expected = RErrorCode::Success;
        assert_eq!(expected, actual);
        assert_eq!(true, rmercury.is_closed);
    }

    #[test]
    fn RMercury_T_CloseSession_InvalidErrorAfterCallingCloseTwice() {
        let mut rmercury = rmercury_test();

        rmercury.close_session();
        let actual = rmercury.close_session();

        let expected = RErrorCode::InvalidSession;
        assert_eq!(expected, actual);
        assert_eq!(true, rmercury.is_closed);
    }

    /// Mocks
    pub struct MockRBackend {}

    impl MockRBackend {
        pub fn new() -> Self {
            return Self {};
        }
    }

    impl RBackend for MockRBackend {
        fn DoPoll(&self, _: i32) -> rsystem::RErrorCode {
            return RErrorCode::Success;
        }
        fn AddPlayer(&mut self, _: rsystem::RPlayer, _: i32) -> rsystem::RErrorCode {
            return RErrorCode::Success;
        }
        fn AddLocalInput(&mut self, _: i32) -> rsystem::RErrorCode {
            return RErrorCode::Success;
        }
        fn SyncInput(&mut self) -> rsystem::RErrorCode {
            return RErrorCode::Success;
        }
        fn IncrementFrame(&mut self) -> rsystem::RErrorCode {
            return RErrorCode::Success;
        }
        fn Chat(&mut self, _: std::string::String) -> rsystem::RErrorCode {
            return RErrorCode::Success;
        }
        fn DisconnectPlayer(_: i32) -> rsystem::RErrorCode {
            return RErrorCode::Success;
        }
        fn GetNetworkStats(
            &self,
            player_handle: RPlayerHandle,
        ) -> (RErrorCode, Option<rsystem::RNetworkStats>) {
            return (RErrorCode::Success, None);
        }
        fn SetFrameDelay(
            &mut self,
            player_handle: RPlayerHandle,
            frame_delay: i32,
        ) -> rsystem::RErrorCode {
            return RErrorCode::Success;
        }
        fn SetDisconnectTimeout(&mut self, timeout: i32) -> rsystem::RErrorCode {
            return RErrorCode::Success;
        }

        fn SetDisconnectNotifyStart(&mut self, timeout: i32) -> RErrorCode {
            return RErrorCode::Success;
        }

        fn CloseSession(&mut self) -> rsystem::RErrorCode {
            return RErrorCode::Success;
        }
    }
}
