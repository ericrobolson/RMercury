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
