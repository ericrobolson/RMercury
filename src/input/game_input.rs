use crate::network;
use network::bitvector::NIBBLE_SIZE;

use crate::utility;
use utility::pow;

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
    /// Initializes a new GameInput. Will panic if 2^NIBBLE_SIZE <= max_bytes * max_players * BITS_IN_BYTES.
    pub fn new(
        max_bytes: usize,
        max_players: usize,
        byte_size_of_all_input_for_all_players: usize,
    ) -> Self {
        if pow(2, NIBBLE_SIZE) <= (max_bytes * max_players * BITS_IN_BYTE) {
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

    /// Initialize a new frame. If the byte_size_of_all_input > max_bytes * max_players, will panic.
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

    /// Initialize a new frame with an offset. If the byte_size_of_all_input > max_bytes, will panic.
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

    /// Check whether the given frame is null or not.
    pub fn is_null(&self) -> bool {
        return self.frame == NULL_FRAME;
    }

    /// Returns the value at the given index
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
    fn game_input_new_sets_proper_values() {
        let input = GameInput::new(3, 4, 5);

        assert_eq!(3, input.max_bytes);
        assert_eq!(4, input.max_players);
        assert_eq!(5, input.byte_size_of_all_input_for_all_players);
        assert_eq!(0, input.frame);
        assert_eq!(get_empty_bits(input.max_bytes), input.bits);
    }

    #[test]
    #[should_panic]
    fn game_input_new_panics_with_bad_values() {
        let input = GameInput::new(99, 9, 5);

        assert_eq!(true, true);
    }

    // init tests
    #[test]
    fn game_input_init_no_bits_sets_proper_values() {
        let max_bytes = 3;
        let max_players = 2;
        let byte_size_of_all_input_for_all_players = 3;

        let mut input = GameInput::new(
            max_bytes,
            max_players,
            byte_size_of_all_input_for_all_players,
        );

        let frame = 2;
        let bits = None;
        let byte_size_of_all_input_for_all_players = 4;

        input.init(frame, bits, byte_size_of_all_input_for_all_players);

        assert_eq!(frame, input.frame);
        assert_eq!(
            byte_size_of_all_input_for_all_players,
            input.byte_size_of_all_input_for_all_players
        );
        assert_eq!(get_empty_bits(input.max_bytes), input.bits);
    }

    #[test]
    fn game_input_init_has_bits_sets_proper_values() {
        let max_bytes = 3;
        let max_players = 2;
        let byte_size_of_all_input_for_all_players = 3;

        let mut input = GameInput::new(
            max_bytes,
            max_players,
            byte_size_of_all_input_for_all_players,
        );

        let frame = 2;
        let bits = vec![true, false, true, false];

        let mut expected_bits = get_empty_bits(max_bytes);

        for (i, bit) in bits.iter().enumerate() {
            expected_bits[i] = *bit;
        }

        let byte_size_of_all_input_for_all_players = 4;
        input.init(frame, Some(bits), byte_size_of_all_input_for_all_players);

        assert_eq!(frame, input.frame);
        assert_eq!(
            byte_size_of_all_input_for_all_players,
            input.byte_size_of_all_input_for_all_players
        );
        assert_eq!(expected_bits, input.bits);
    }

    #[test]
    #[should_panic]
    fn game_input_init_byte_size_all_inputs_greater_than_max_bytes_x_max_players_panics() {
        let max_bytes = 3;
        let max_players = 2;
        let byte_size_of_all_input_for_all_players = 3;

        let mut input = GameInput::new(
            max_bytes,
            max_players,
            byte_size_of_all_input_for_all_players,
        );

        let frame = 2;
        let bits = vec![true, false, true, false];

        let mut expected_bits = get_empty_bits(max_bytes);

        for (i, bit) in bits.iter().enumerate() {
            expected_bits[i] = *bit;
        }

        let byte_size_of_all_input_for_all_players = 999999;
        input.init(frame, Some(bits), byte_size_of_all_input_for_all_players);
        assert_eq!(true, true);
    }

    // init_offset tests
    #[test]
    fn game_input_init_offset_no_bits_sets_proper_values() {
        let max_bytes = 3;
        let max_players = 2;
        let byte_size_of_all_input_for_all_players = 2;

        let mut input = GameInput::new(
            max_bytes,
            max_players,
            byte_size_of_all_input_for_all_players,
        );

        let offset = 0;
        let frame = 2;
        let bits = None;
        let byte_size_of_all_input_for_all_players = 3;

        input.init_offset(frame, bits, byte_size_of_all_input_for_all_players, offset);

        assert_eq!(frame, input.frame);
        assert_eq!(
            byte_size_of_all_input_for_all_players,
            input.byte_size_of_all_input_for_all_players
        );
        assert_eq!(get_empty_bits(input.max_bytes), input.bits);
    }

    #[test]
    fn game_input_init_offset_has_bits_sets_proper_values() {
        let max_bytes = 3;
        let max_players = 2;
        let byte_size_of_all_input_for_all_players = 2;

        let mut input = GameInput::new(
            max_bytes,
            max_players,
            byte_size_of_all_input_for_all_players,
        );

        let offset = 0;
        let frame = 2;
        let bits = vec![true, false, true, false];

        let mut expected_bits = get_empty_bits(max_bytes);

        for (i, bit) in bits.iter().enumerate() {
            expected_bits[i] = *bit;
        }

        let byte_size_of_all_input_for_all_players = 3;
        input.init_offset(
            frame,
            Some(bits),
            byte_size_of_all_input_for_all_players,
            offset,
        );

        assert_eq!(frame, input.frame);
        assert_eq!(
            byte_size_of_all_input_for_all_players,
            input.byte_size_of_all_input_for_all_players
        );
        assert_eq!(expected_bits, input.bits);
    }

    #[test]
    fn game_input_init_offset_with_offset_no_bits_sets_proper_values() {
        let max_bytes = 3;
        let max_players = 2;
        let byte_size_of_all_input_for_all_players = 2;

        let mut input = GameInput::new(
            max_bytes,
            max_players,
            byte_size_of_all_input_for_all_players,
        );

        let offset = 2;
        let frame = 2;
        let bits = None;
        let byte_size_of_all_input_for_all_players = 3;

        input.init_offset(frame, bits, byte_size_of_all_input_for_all_players, offset);

        assert_eq!(frame, input.frame);
        assert_eq!(
            byte_size_of_all_input_for_all_players,
            input.byte_size_of_all_input_for_all_players
        );
        assert_eq!(get_empty_bits(input.max_bytes), input.bits);
    }

    #[test]
    fn game_input_init_offset_has_bits_with_offset3_sets_proper_values() {
        let max_bytes = 5;
        let max_players = 2;
        let byte_size_of_all_input_for_all_players = 5;

        let mut input = GameInput::new(
            max_bytes,
            max_players,
            byte_size_of_all_input_for_all_players,
        );

        let offset = 3;
        let frame = 2;
        let bits = vec![true, false, true, false];

        let mut expected_bits = get_empty_bits(max_bytes);

        for (i, bit) in bits.iter().enumerate() {
            expected_bits[i + (offset * byte_size_of_all_input_for_all_players)] = *bit;
        }

        let byte_size_of_all_input_for_all_players = 5;
        input.init_offset(
            frame,
            Some(bits),
            byte_size_of_all_input_for_all_players,
            offset,
        );

        assert_eq!(frame, input.frame);
        assert_eq!(
            byte_size_of_all_input_for_all_players,
            input.byte_size_of_all_input_for_all_players
        );
        assert_eq!(expected_bits, input.bits);
    }

    #[test]
    fn game_input_init_offset_has_bits_with_offset1_sets_proper_values() {
        let max_bytes = 5;
        let max_players = 2;
        let byte_size_of_all_input_for_all_players = 5;

        let mut input = GameInput::new(
            max_bytes,
            max_players,
            byte_size_of_all_input_for_all_players,
        );

        let offset = 1;
        let frame = 2;
        let bits = vec![true, false, true, false];

        let mut expected_bits = get_empty_bits(max_bytes);

        for (i, bit) in bits.iter().enumerate() {
            expected_bits[i + (offset * byte_size_of_all_input_for_all_players)] = *bit;
        }

        let byte_size_of_all_input_for_all_players = 5;
        input.init_offset(
            frame,
            Some(bits),
            byte_size_of_all_input_for_all_players,
            offset,
        );

        assert_eq!(frame, input.frame);
        assert_eq!(
            byte_size_of_all_input_for_all_players,
            input.byte_size_of_all_input_for_all_players
        );
        assert_eq!(expected_bits, input.bits);
    }

    #[test]
    #[should_panic]
    fn game_input_init_offset_byte_size_all_inputs_greater_than_max_bytes_x_max_players_panics() {
        let max_bytes = 3;
        let max_players = 2;
        let byte_size_of_all_input_for_all_players = 3;

        let mut input = GameInput::new(
            max_bytes,
            max_players,
            byte_size_of_all_input_for_all_players,
        );

        let frame = 2;
        let bits = vec![true, false, true, false];
        let offset = 0;

        let mut expected_bits = get_empty_bits(max_bytes);

        for (i, bit) in bits.iter().enumerate() {
            expected_bits[i] = *bit;
        }

        let byte_size_of_all_input_for_all_players = 999999;
        input.init_offset(
            frame,
            Some(bits),
            byte_size_of_all_input_for_all_players,
            offset,
        );
        assert_eq!(true, true);
    }

    // is_null tests
    #[test]
    fn game_input_is_null_not_null_frame_returns_false() {
        let max_bytes = 5;
        let max_players = 2;
        let byte_size_of_all_input_for_all_players = 5;

        let mut input = GameInput::new(
            max_bytes,
            max_players,
            byte_size_of_all_input_for_all_players,
        );

        let offset = 1;
        let frame = 2;
        let bits = vec![true, false, true, false];

        let mut expected_bits = get_empty_bits(max_bytes);

        for (i, bit) in bits.iter().enumerate() {
            expected_bits[i + (offset * byte_size_of_all_input_for_all_players)] = *bit;
        }

        let byte_size_of_all_input_for_all_players = 5;
        input.init_offset(
            frame,
            Some(bits),
            byte_size_of_all_input_for_all_players,
            offset,
        );

        let actual = input.is_null();
        let expected = false;

        assert_eq!(expected, actual);
    }

    #[test]
    fn game_input_is_null_null_frame_returns_true() {
        let max_bytes = 5;
        let max_players = 2;
        let byte_size_of_all_input_for_all_players = 5;

        let mut input = GameInput::new(
            max_bytes,
            max_players,
            byte_size_of_all_input_for_all_players,
        );

        let offset = 1;
        let frame = NULL_FRAME;
        let bits = vec![true, false, true, false];

        let mut expected_bits = get_empty_bits(max_bytes);

        for (i, bit) in bits.iter().enumerate() {
            expected_bits[i + (offset * byte_size_of_all_input_for_all_players)] = *bit;
        }

        let byte_size_of_all_input_for_all_players = 5;
        input.init_offset(
            frame,
            Some(bits),
            byte_size_of_all_input_for_all_players,
            offset,
        );

        let actual = input.is_null();
        let expected = true;

        assert_eq!(expected, actual);
    }

    // value tests
    #[test]
    fn game_input_value_index_2_returns_false() {
        let max_bytes = 5;
        let max_players = 2;
        let byte_size_of_all_input_for_all_players = 5;

        let mut input = GameInput::new(
            max_bytes,
            max_players,
            byte_size_of_all_input_for_all_players,
        );

        let frame = NULL_FRAME;
        let bits = vec![true, false, false, false];

        let mut expected_bits = get_empty_bits(max_bytes);

        for (i, bit) in bits.iter().enumerate() {
            expected_bits[i] = *bit;
        }

        let byte_size_of_all_input_for_all_players = 5;
        input.init(frame, Some(bits), byte_size_of_all_input_for_all_players);

        let actual = input.value(2);
        let expected = false;

        assert_eq!(expected, actual);
    }

    #[test]
    fn game_input_value_index_4_returns_true() {
        let max_bytes = 5;
        let max_players = 2;
        let byte_size_of_all_input_for_all_players = 5;

        let mut input = GameInput::new(
            max_bytes,
            max_players,
            byte_size_of_all_input_for_all_players,
        );

        let frame = NULL_FRAME;
        let bits = vec![true, false, false, false, true];

        let mut expected_bits = get_empty_bits(max_bytes);

        for (i, bit) in bits.iter().enumerate() {
            expected_bits[i] = *bit;
        }

        let byte_size_of_all_input_for_all_players = 5;
        input.init(frame, Some(bits), byte_size_of_all_input_for_all_players);

        let actual = input.value(4);
        let expected = true;

        assert_eq!(expected, actual);
    }
}