use crate::rsystem;
use rsystem::{RErrorCode, RPlayer, RPlayerHandle};

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
