use super::game_input;
use game_input::GameInput;

const DEFAULT_INPUT_QUEUE_LENGTH: usize = 128;
const DEFAULT_INPUT_SIZE: usize = 4;

pub struct InputQueue {
    id: usize,
    head: usize,
    tail: usize,
    length: usize,
    first_frame: bool,
    last_user_added_frame: usize,
    last_added_frame: usize,
    first_incorrect_frame: usize,
    last_frame_requested: usize,
    frame_delay: usize,
    inputs: Vec<GameInput>,
    prediction: GameInput,
}

impl InputQueue {
    pub fn new(default_input_size: Option<usize>, default_queue_length: Option<usize>) -> Self {
        let default_input_size = default_input_size.unwrap_or(DEFAULT_INPUT_SIZE);
        let default_queue_length = default_queue_length.unwrap_or(DEFAULT_INPUT_QUEUE_LENGTH);

        unimplemented!();
    }

    pub fn get_last_confirmed_frame(&self) -> usize {
        return self.last_added_frame;
    }

    pub fn get_first_incorrect_frame(&self) -> usize {
        return self.first_incorrect_frame;
    }

    pub fn get_length(&self) -> usize {
        return self.length;
    }

    pub fn set_frame_delay(&mut self, delay: usize) {
        self.frame_delay = delay;
    }

    pub fn reset_prediction(&mut self, frame: usize) {
        unimplemented!();
    }

    pub fn discard_confirmed_frames(&mut self, frame: usize) {
        unimplemented!();
    }

    pub fn get_confirmed_input(&self, frame: usize) -> Option<GameInput> {
        unimplemented!();
    }

    pub fn get_input(&self, frame: usize) -> (bool, Option<GameInput>) {
        unimplemented!();
    }

    pub fn add_input(&mut self, input: GameInput) {
        unimplemented!();
    }

    fn advance_queue_head(&mut self, frame: usize) -> usize {
        unimplemented!();
    }

    fn add_delayed_input_to_queue(&mut self, input: GameInput, i: usize) {
        unimplemented!();
    }

    fn log(&self) {
        unimplemented!();
    }
}
