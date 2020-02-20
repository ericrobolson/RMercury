use crate::network;
use network::bitvector::NIBBLE_SIZE;

const NULL_FRAME: i32 = -1;

pub struct Bits {}

pub struct GameInput {
    max_bytes: usize,
    max_players: usize,
    frame: i32,
    size_of_all_input_for_all_players: usize,
    bits: Vec<Bits>,
}

impl GameInput {
    pub fn new(max_bytes: usize, max_players: usize, size_of_all_input_for_players: usize) -> Self {
        const BITS_IN_BYTE: usize = 8;
        if 2 ^ NIBBLE_SIZE <= (max_bytes * max_players * BITS_IN_BYTE) {
            panic!("ERROR ALLOCATING MEMORY"); //TODO: expand on why this is bad
        }

        unimplemented!();

        return Self {
            max_bytes: max_bytes,
            max_players: max_players,
            frame: 0,
            size_of_all_input_for_all_players: size_of_all_input_for_players,
            bits: vec![],
        };
    }

    pub fn is_null(&self) -> bool {
        return self.frame == NULL_FRAME;
    }

    pub fn init(&mut self, frame: i32, bits: Vec<Bits>, size_of_all_input_for_players: usize) {
        unimplemented!();
    }
    pub fn init_offset(
        &mut self,
        frame: i32,
        bits: Vec<Bits>,
        size_of_all_input_for_players: usize,
    ) {
        unimplemented!();
    }

    pub fn value(&self, i: usize) -> bool {
        unimplemented!();
        // return (self.bits[i / 8] & (1 << (i % 8))) != 0;
    }

    pub fn set(&mut self, i: usize) {
        unimplemented!();
    }

    pub fn clear(&mut self, i: usize) {
        unimplemented!();
    }

    pub fn erase(&mut self, i: usize) {
        unimplemented!();
    }

    pub fn desc(&self, i: usize) {
        unimplemented!();
    }

    pub fn log(&self, i: usize) {
        unimplemented!();
    }

    pub fn equal(&self, input: &GameInput, bits_only: bool) -> bool {
        unimplemented!();
    }
}
