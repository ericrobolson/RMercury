/// Trait required to link up game input to a form that RMercury can utilize.
pub trait RMercuryInput {
    /// Serialize the input to bits.
    fn to_bits(&self) -> Vec<u8>;

    /// Deserialize the input from bits.
    fn from_bits(bytes: Vec<u8>) -> Self;
}
