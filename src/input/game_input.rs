use crate::network;
use network::bitvector::NIBBLE_SIZE;

use crate::utility;
use utility::pow;

const BITS_IN_BYTE: usize = 8;

/// Byte array representing the game input.
pub struct GameInput {
    max_bytes: usize,
    max_players: usize,
    pub frame: Option<usize>,
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
            frame: Some(0),
            byte_size_of_all_input_for_all_players: byte_size_of_all_input_for_all_players,
            bits: get_empty_bits(max_bytes),
        };
    }

    /// Initialize a new frame. If the byte_size_of_all_input > max_bytes * max_players, will panic.
    pub fn init(
        &mut self,
        frame: Option<usize>,
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
        frame: Option<usize>,
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
        return self.frame.is_none();
    }

    /// Returns the value at the given index.
    pub fn value(&self, i: usize) -> bool {
        return self.bits[i];
    }

    /// Set the bit at the given index.
    pub fn set(&mut self, i: usize) {
        self.bits[i] = true;
    }

    /// Clear the bit at the given index.
    pub fn clear(&mut self, i: usize) {
        self.bits[i] = false;
    }

    /// Erase all bits.
    pub fn erase(&mut self) {
        self.bits = get_empty_bits(self.max_bytes);
    }

    pub fn desc(&self, i: usize) {
        unimplemented!();
    }

    pub fn log(&self, i: usize) {
        unimplemented!();
    }

    /// Return whether the inputs are equal. If bits_only is set, just compares the bits.
    pub fn equal(&self, input: &GameInput, bits_only: bool) -> bool {
        let frames_match = self.frame == input.frame;
        if !bits_only && !frames_match {
            // FRAMES DON'T MATCH
        }

        let sizes_match = self.byte_size_of_all_input_for_all_players
            == input.byte_size_of_all_input_for_all_players;

        // if !sizes_match {
        //    // SIZES DON'T MATCH
        // }

        let bits_match = self.bits == input.bits;

        // if !bits_match {
        //     // BITS DON"T MATCH
        // }

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
        assert_eq!(Some(0), input.frame);
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

        let frame = Some(2);
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

        let frame = Some(2);
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

        let frame = Some(2);
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
        let frame = Some(2);
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
        let frame = Some(2);
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
        let frame = Some(2);
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
        let frame = Some(2);
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
        let frame = Some(2);
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

        let frame = Some(2);
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
        let frame = Some(2);
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
        let frame = None;
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

        let frame = None;
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

        let frame = None;
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

    // set tests
    #[test]
    fn game_input_set_index_2_sets_proper_value() {
        let max_bytes = 5;
        let max_players = 2;
        let byte_size_of_all_input_for_all_players = 5;

        let mut input = GameInput::new(
            max_bytes,
            max_players,
            byte_size_of_all_input_for_all_players,
        );

        let i = 2;

        input.set(i);

        let actual = input.value(i);
        let expected = true;

        assert_eq!(expected, actual);
    }

    #[test]
    fn game_input_set_index_5_sets_proper_value() {
        let max_bytes = 5;
        let max_players = 2;
        let byte_size_of_all_input_for_all_players = 5;

        let mut input = GameInput::new(
            max_bytes,
            max_players,
            byte_size_of_all_input_for_all_players,
        );

        let i = 5;

        input.set(i);

        let actual = input.value(i);
        let expected = true;

        assert_eq!(expected, actual);
    }

    // clear tests
    #[test]
    fn game_input_clear_index_2_clears_value() {
        let max_bytes = 5;
        let max_players = 2;
        let byte_size_of_all_input_for_all_players = 5;

        let mut input = GameInput::new(
            max_bytes,
            max_players,
            byte_size_of_all_input_for_all_players,
        );

        let i = 2;
        input.set(i);
        input.clear(i);

        let actual = input.value(i);
        let expected = false;

        assert_eq!(expected, actual);
    }

    #[test]
    fn game_input_clear_index_6_clears_value() {
        let max_bytes = 5;
        let max_players = 2;
        let byte_size_of_all_input_for_all_players = 5;

        let mut input = GameInput::new(
            max_bytes,
            max_players,
            byte_size_of_all_input_for_all_players,
        );

        let i = 6;
        input.set(i);
        input.clear(i);

        let actual = input.value(i);
        let expected = false;

        assert_eq!(expected, actual);
    }

    // erase tests
    #[test]
    fn game_input_erase_clears_all_input() {
        let max_bytes = 5;
        let max_players = 2;
        let byte_size_of_all_input_for_all_players = 5;

        let mut input = GameInput::new(
            max_bytes,
            max_players,
            byte_size_of_all_input_for_all_players,
        );

        let frame = None;
        let bits = vec![true, false, false, false, true];

        let mut expected_bits = get_empty_bits(max_bytes);

        let byte_size_of_all_input_for_all_players = 5;
        input.init(frame, Some(bits), byte_size_of_all_input_for_all_players);

        input.erase();
        let actual = input.bits;

        assert_eq!(expected_bits, actual);
    }

    // equal tests
    #[test]
    fn game_input_equal_bits_only_matches_returns_true() {
        let max_bytes = 5;
        let byte_size_of_all_input_for_all_players = 5;

        let mut input1 = GameInput::new(max_bytes, 2, 4);

        let mut input2 = GameInput::new(max_bytes, 3, 5);

        let bits = vec![true, false, false, false, true];

        input1.init(
            Some(2),
            Some(bits.clone()),
            byte_size_of_all_input_for_all_players,
        );
        input2.init(Some(3), Some(bits), byte_size_of_all_input_for_all_players);

        assert_eq!(true, input1.equal(&input2, true));
        assert_eq!(true, input2.equal(&input1, true));
    }

    #[test]
    fn game_input_equal_bits_only_no_match_returns_false() {
        let max_bytes = 5;
        let byte_size_of_all_input_for_all_players = 5;

        let mut input1 = GameInput::new(max_bytes, 2, 4);

        let mut input2 = GameInput::new(max_bytes, 3, 5);

        let bits1 = vec![true, false, false, false, true];
        let bits2 = vec![true, true, false, false, true];

        input1.init(Some(2), Some(bits1), byte_size_of_all_input_for_all_players);
        input2.init(Some(3), Some(bits2), byte_size_of_all_input_for_all_players);

        assert_eq!(false, input1.equal(&input2, true));
        assert_eq!(false, input2.equal(&input1, true));
    }

    #[test]
    fn game_input_equal_matches_returns_true() {
        let max_bytes = 5;
        let byte_size_of_all_input_for_all_players = 5;

        let mut input1 = GameInput::new(max_bytes, 2, 5);
        let mut input2 = GameInput::new(max_bytes, 3, 5);

        let bits = vec![true, false, false, false, true];

        input1.init(
            Some(2),
            Some(bits.clone()),
            byte_size_of_all_input_for_all_players,
        );
        input2.init(Some(2), Some(bits), byte_size_of_all_input_for_all_players);

        assert_eq!(true, input1.equal(&input2, false));
        assert_eq!(true, input2.equal(&input1, false));
    }

    #[test]
    fn game_input_equal_frames_dont_matches_returns_false() {
        let max_bytes = 5;
        let byte_size_of_all_input_for_all_players = 5;

        let mut input1 = GameInput::new(max_bytes, 2, 5);
        let mut input2 = GameInput::new(max_bytes, 3, 5);

        let bits = vec![true, false, false, false, true];

        input1.init(
            Some(2),
            Some(bits.clone()),
            byte_size_of_all_input_for_all_players,
        );
        input2.init(Some(3), Some(bits), byte_size_of_all_input_for_all_players);

        assert_eq!(false, input1.equal(&input2, false));
        assert_eq!(false, input2.equal(&input1, false));
    }

    #[test]
    fn game_input_equal_bits_dont_matches_returns_false() {
        let max_bytes = 5;
        let byte_size_of_all_input_for_all_players = 5;

        let mut input1 = GameInput::new(max_bytes, 2, 5);
        let mut input2 = GameInput::new(max_bytes, 3, 5);

        let bits1 = vec![true, false, false, false, true];
        let bits2 = vec![true, false, false, false, false];

        input1.init(Some(2), Some(bits1), byte_size_of_all_input_for_all_players);
        input2.init(Some(2), Some(bits2), byte_size_of_all_input_for_all_players);

        assert_eq!(false, input1.equal(&input2, false));
        assert_eq!(false, input2.equal(&input1, false));
    }

    #[test]
    fn game_input_equal_sizes_dont_matches_returns_false() {
        let max_bytes = 5;
        let byte_size_of_all_input_for_all_players = 5;

        let mut input1 = GameInput::new(max_bytes, 2, 5);
        let mut input2 = GameInput::new(max_bytes, 3, 5);

        let bits = vec![true, false, false, false, true];

        input1.init(Some(2), Some(bits.clone()), 4);
        input2.init(Some(2), Some(bits), 3);

        assert_eq!(false, input1.equal(&input2, false));
        assert_eq!(false, input2.equal(&input1, false));
    }
}
