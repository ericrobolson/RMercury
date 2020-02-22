mod rmercury;
mod rmercury_builder;
mod rmercury_game_interface;
mod rmercury_input;

pub use crate::{
    rmercury::MercuryType, rmercury::RMercury, rmercury_builder::RMercuryBuilder,
    rmercury_game_interface::RMercuryGameInterface, rmercury_input::RMercuryInput,
};

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
