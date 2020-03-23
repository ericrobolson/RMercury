/// Trait required to link up game input to a form that RMercury can utilize.
pub trait RMercuryInput {
    /// Get the player id that the input maps to.
    fn get_player_id(&self) -> usize;

    /// Set the player id that the input will map to.
    fn set_player_id(&mut self, player_id: usize);

    /// Serialize the input to bits.
    fn to_bits(&self) -> Vec<u8>;

    /// Deserialize the input from bits.
    fn from_bits(bytes: Vec<u8>) -> Self;
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RMercuryInputWrapper<TGameInput>
where
    TGameInput: RMercuryInput,
    TGameInput: Copy,
    TGameInput: PartialEq,
{
    /// The input to execute
    pub input: TGameInput,
    /// The frame the input will be executed for
    pub frame: usize,
}

impl<TGameInput> RMercuryInputWrapper<TGameInput>
where
    TGameInput: RMercuryInput,
    TGameInput: Copy,
    TGameInput: PartialEq,
{
    pub fn new(input: TGameInput, frame: usize) -> Self {
        return Self {
            frame: frame,
            input: input,
        };
    }

    /// Get the player id that the input maps to.
    fn get_player_id(&self) -> usize {
        return self.input.get_player_id();
    }

    /// Serialize the input to bits.
    fn to_bits(&self) -> Vec<u8> {
        let input_bits = self.input.to_bits();
        //TODO: add in frame
        unimplemented!();
    }

    /// Deserialize the input from bits.
    fn from_bits(bytes: Vec<u8>) -> Self {
        let frame = 0; //TODO: deserialize frame
        let input_bits = false; //TODO: deserialize inputs;
        unimplemented!();
    }
}
