extern crate rmercury;
use rmercury::{MercuryType, RMercuryBuilder};

pub mod pong_lib;
use pong_lib::*;

fn main() {
    // Init GFX
    let mut pong_gfx = PongGfx::new();

    // Init RMercury
    let game_interface = GameInterface::new();

    let mut builder = RMercuryBuilder::<GameInterface, GameInput, GameState>::new(&game_interface)
        .with_type(MercuryType::Peer2Peer);

    let mut r_mercury = builder.build();

    loop {
        {
            // Parse player input
            let player_actions = pong_gfx.get_player_input();
            if should_quit(&player_actions) {
                break;
            }

            let mut player_input = player_actions
                .iter()
                .map(|i| GameInput::from_action(*i))
                .collect();

            r_mercury.add_local_input(&mut player_input);
            r_mercury.execute();

            let current_state = r_mercury.get_game_state();

            pong_gfx.render(&current_state); // NOTE: gfx/sound should be divorced from actual game processing
        }
    }
}
