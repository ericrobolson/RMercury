mod backends;
mod input;
mod network;
mod revent;
mod rgame;
mod ringbuffer;
mod rmercury;
mod rplayer;
mod rsystem;
mod utility;

pub use self::rmercury::{
    rmercury_start_session, rmercury_start_spectator, rmercury_start_synctest, RSession,
};

pub use self::rgame::RGame;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
