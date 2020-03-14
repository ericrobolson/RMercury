pub struct RChannelManager {}
impl RChannelManager {
    pub fn new() -> Self {
        return Self {};
    }

    /// Queue the local inputs to send over the network
    pub fn queue_local_input(&mut self) {}

    /// Sync up all inputs, getting remote player's inputs + sending current inputs.
    pub fn execute(&mut self) {}
}

pub struct RChannel {}

impl RChannel {
    pub fn new() -> Self {
        unimplemented!();
    }
}
