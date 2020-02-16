mod backends;
mod revent;
mod rmercury;
mod rplayer;
mod rsystem;

pub use self::rmercury::{
    rmercury_start_session, rmercury_start_spectator, rmercury_start_synctest, RSession,
};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
