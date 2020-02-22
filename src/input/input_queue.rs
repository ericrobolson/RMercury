use super::game_input;
use game_input::GameInput;

const DEFAULT_INPUT_QUEUE_LENGTH: usize = 128;
const DEFAULT_INPUT_SIZE: usize = 4;

pub struct InputQueue {
    id: Option<usize>,
    head: usize,
    tail: usize,
    length: usize,
    input_size: usize,
    first_frame: bool,
    last_user_added_frame: Option<usize>,
    last_added_frame: Option<usize>,
    first_incorrect_frame: Option<usize>,
    last_frame_requested: Option<usize>,
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

    pub fn init(&mut self, id: usize, input_size: usize) {
        self.input_size = input_size;
        self.id = Some(id);
        self.head = 0;
        self.tail = 0;
        self.length = 0;
        self.frame_delay = 0;
        self.first_frame = true;
        self.last_user_added_frame = None;
        self.last_added_frame = None;
        self.first_incorrect_frame = None;
        self.last_frame_requested = None;

        self.prediction.init(None, None, self.input_size);
        self.inputs.clear();
    }

    pub fn get_last_confirmed_frame(&self) -> Option<usize> {
        return self.last_added_frame;
    }

    pub fn get_first_incorrect_frame(&self) -> Option<usize> {
        return self.first_incorrect_frame;
    }

    pub fn get_length(&self) -> usize {
        return self.length;
    }

    pub fn set_frame_delay(&mut self, delay: usize) {
        self.frame_delay = delay;
    }

    pub fn reset_prediction(&mut self, frame: usize) {
        if self.first_incorrect_frame.is_some() && frame > self.first_incorrect_frame.unwrap() {
            panic!("Frame must be lower than or equal to the first incorrect frame.")
        }

        self.prediction.frame = None;
        self.first_incorrect_frame = None;
        self.last_frame_requested = None;
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

    fn add_delayed_input_to_queue(&mut self, input: GameInput, i: usize) {
        unimplemented!();
    }

    fn advance_queue_head(&mut self, frame: usize) -> usize {
        unimplemented!();
    }

    fn log(&self) {
        unimplemented!();
    }
}
