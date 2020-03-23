use super::*;
use rmercury_input::{RMercuryInput, RMercuryInputWrapper};

pub struct RChannel<TGameInput>
where
    TGameInput: RMercuryInput,
    TGameInput: Copy,
    TGameInput: PartialEq,
{
    local_input_to_send: Vec<RMercuryInputWrapper<TGameInput>>,
}

impl<TGameInput> RChannel<TGameInput>
where
    TGameInput: RMercuryInput,
    TGameInput: Copy,
    TGameInput: PartialEq,
{
    pub fn new() -> Self {
        unimplemented!();
    }

    /// Get the last confirmed frame for the current channel
    pub fn last_confirmed_frame(&self) -> usize {
        unimplemented!();
    }

    /// Queue the local player input to be sent over the channel
    pub fn queue_local_input(&mut self, inputs: &Vec<RMercuryInputWrapper<TGameInput>>) {
        unimplemented!();
    }

    /// Get the received remote player's inputs and send the queued inputs.
    pub fn sync(&mut self) -> Vec<RMercuryInputWrapper<TGameInput>> {
        unimplemented!();
    }
}

pub struct RChannelManager<TGameInput>
where
    TGameInput: RMercuryInput,
    TGameInput: Copy,
    TGameInput: PartialEq,
{
    last_confirmed_local_input_frame: usize,
    channels: Vec<RChannel<TGameInput>>,
}

impl<TGameInput> RChannelManager<TGameInput>
where
    TGameInput: RMercuryInput,
    TGameInput: Copy,
    TGameInput: PartialEq,
{
    pub fn new() -> Self {
        return Self {
            channels: vec![],
            last_confirmed_local_input_frame: 0,
        };
    }

    /// Retrieve the last frame that is confirmed for all players
    pub fn last_confirmed_frame(&self) -> usize {
        let mut last_confirmed_frame = self.last_confirmed_local_input_frame;
        for channel in self.channels.iter() {
            let channel_last_confirmed_frame = channel.last_confirmed_frame();
            if channel_last_confirmed_frame <= last_confirmed_frame {
                last_confirmed_frame = channel_last_confirmed_frame;
            }
        }

        return last_confirmed_frame;
    }

    /// Queue the local inputs to send over the network
    pub fn queue_local_input(&mut self, inputs: &Vec<RMercuryInputWrapper<TGameInput>>) {
        for input in inputs.iter() {
            if input.frame >= self.last_confirmed_local_input_frame {
                self.last_confirmed_local_input_frame = input.frame;
            }
        }

        for channel in self.channels.iter_mut() {
            channel.queue_local_input(&inputs);
        }
    }

    /// Sync up all inputs, getting remote player's inputs + sending current inputs.
    pub fn execute(&mut self) -> Vec<RMercuryInputWrapper<TGameInput>> {
        let mut inputs = vec![];

        for channel in self.channels.iter_mut() {
            let mut remote_inputs = channel.sync();

            if remote_inputs.is_empty() {
                // Add prediction?
            }

            inputs.append(&mut remote_inputs);
        }

        return inputs;
    }
}
