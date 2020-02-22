mod rmercury;
mod rmercury_builder;
mod rmercury_input;

pub use crate::{
    rmercury::MercuryType, rmercury::RMercury, rmercury_builder::RMercuryBuilder,
    rmercury_input::RMercuryInput,
};

struct NetworkInfo {}

pub trait RMercuryGameInterface<TGameState, TGameInput> {
    fn save_game_state() {}
    fn load_game_state() {}
    fn log_game_state() {}
    fn advance_frame(&mut self, inputs: Vec<TGameInput>);
    fn current_game_state(&self) -> TGameState;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[should_panic]
    fn it_should_panic_if_not_built() {
        assert_eq!(true, false);
    }

    #[test]
    fn it_should_build_with_proper_type() {
        assert_eq!(false, true);
    }
}
