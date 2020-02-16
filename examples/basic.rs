extern crate sdl2;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect};

fn main() {
    // Init SDL
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let window = video_subsystem.window("Window", 800, 600).build().unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    'gameloop: loop {
        {
            // Parse player input
            let player_actions = get_player_input(&mut event_pump);
            for action in player_actions.iter() {
                match action {
                    Actions::Quit => {
                        break 'gameloop;
                    }
                    _ => {}
                }
            }

            render(&mut canvas); // NOTE: gfx/sound should be divorced from actual game processing
        }
    }

    // Cleanup
}

fn render(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    // Background color
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    // Draw players
    canvas.set_draw_color(Color::RGB(255, 210, 0));
    canvas.fill_rect(Rect::new(20, 20, 20, 20)).unwrap();

    // Final present
    canvas.present();
}

enum Actions {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Quit,
}

fn get_player_input(event_pump: &mut sdl2::EventPump) -> Vec<Actions> {
    let mut player_actions = vec![];
    for event in event_pump.poll_iter() {
        match event {
            // Quit game
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                player_actions.push(Actions::Quit);
            }
            Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            } => {
                player_actions.push(Actions::MoveLeft);
            }
            Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            } => {
                player_actions.push(Actions::MoveRight);
            }
            Event::KeyDown {
                keycode: Some(Keycode::Up),
                ..
            } => {
                player_actions.push(Actions::MoveUp);
            }
            Event::KeyDown {
                keycode: Some(Keycode::Down),
                ..
            } => {
                player_actions.push(Actions::MoveDown);
            }
            _ => {}
        }
    }

    return player_actions;
}
