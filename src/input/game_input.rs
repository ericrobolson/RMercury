use crate::network;
use network::bitvector::NIBBLE_SIZE;

const NULL_FRAME: i32 = -1;
const BITS_IN_BYTE: usize = 8;

pub struct GameInput {
    max_bytes: usize,
    max_players: usize,
    frame: i32,
    byte_size_of_all_input_for_all_players: usize,
    bits: Vec<bool>,
}

fn get_empty_bits(max_bytes: usize) -> Vec<bool> {
    let mut bits = vec![];
    for i in 0..(max_bytes * BITS_IN_BYTE) {
        bits.push(false);
    }

    return bits;
}
impl GameInput {
    pub fn new(
        max_bytes: usize,
        max_players: usize,
        byte_size_of_all_input_for_all_players: usize,
    ) -> Self {
        if 2 ^ NIBBLE_SIZE <= (max_bytes * max_players * BITS_IN_BYTE) {
            panic!("ERROR ALLOCATING MEMORY"); //TODO: expand on why this is bad
        }

        return Self {
            max_bytes: max_bytes,
            max_players: max_players,
            frame: 0,
            byte_size_of_all_input_for_all_players: byte_size_of_all_input_for_all_players,
            bits: get_empty_bits(max_bytes),
        };
    }

    pub fn is_null(&self) -> bool {
        return self.frame == NULL_FRAME;
    }

    pub fn init(
        &mut self,
        frame: i32,
        bits: Option<Vec<bool>>,
        byte_size_of_all_input_for_all_players: usize,
    ) {
        if byte_size_of_all_input_for_all_players > (self.max_bytes * self.max_players) {
            panic!("ERROR ALLOCATING INPUT; BUFFER OVERFLOW");
        }

        self.frame = frame;
        self.byte_size_of_all_input_for_all_players = byte_size_of_all_input_for_all_players;
        self.bits = get_empty_bits(self.max_bytes);

        if bits.is_some() {
            let bits = bits.unwrap();
            for (i, bit) in bits.iter().enumerate() {
                self.bits[i] = *bit;
            }
        }
    }
    pub fn init_offset(
        &mut self,
        frame: i32,
        bits: Option<Vec<bool>>,
        byte_size_of_all_input_for_all_players: usize,
        offset: usize,
    ) {
        if byte_size_of_all_input_for_all_players > self.max_bytes {
            panic!("ERROR ALLOCATING INPUT; BUFFER OVERFLOW");
        }

        self.frame = frame;
        self.byte_size_of_all_input_for_all_players = byte_size_of_all_input_for_all_players;
        self.bits = get_empty_bits(self.max_bytes);

        if bits.is_some() {
            let bits = bits.unwrap();
            for (i, bit) in bits.iter().enumerate() {
                self.bits[i + (offset * byte_size_of_all_input_for_all_players)] = *bit;
            }
        }
    }

    pub fn value(&self, i: usize) -> bool {
        return self.bits[i];
    }

    pub fn set(&mut self, i: usize) {
        self.bits[i] = true;
    }

    pub fn clear(&mut self, i: usize) {
        self.bits[i] = false;
    }

    pub fn erase(&mut self) {
        self.bits = get_empty_bits(self.max_bytes);
    }

    pub fn desc(&self, i: usize) {
        unimplemented!();
    }

    pub fn log(&self, i: usize) {
        unimplemented!();
    }

    pub fn equal(&self, input: &GameInput, bits_only: bool) -> bool {
        let frames_match = self.frame == input.frame;
        if !bits_only && !frames_match {
            // FRAMES DON'T MATCH
        }

        let sizes_match = self.byte_size_of_all_input_for_all_players
            == input.byte_size_of_all_input_for_all_players;

        if !sizes_match {
            // SIZES DON'T MATCH
        }

        let bits_match = self.bits == input.bits;

        if !bits_match {
            // BITS DON"T MATCH
        }

        return (bits_only || frames_match) && sizes_match && bits_match;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // new tests
    #[test]
    fn game_input_new_initializes_proper_values() {
        assert_eq!(true, false);
    }
}
