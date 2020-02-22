extern crate sdl2;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect};

extern crate rmercury;
use rmercury::{MercuryType, RMercuryBuilder, RMercuryGameInterface, RMercuryInput};

pub struct GameInterface {}

impl GameInterface {
    pub fn new() -> Self {
        return Self {};
    }
}

pub struct GameState {}

impl RMercuryGameInterface<GameState, GameInput> for GameInterface {
    fn current_game_state(&self) -> GameState {
        unimplemented!()
    }

    fn advance_frame(&mut self, _: std::vec::Vec<GameInput>) {
        unimplemented!()
    }
    fn save_game_state(&self) -> GameState {
        unimplemented!()
    }
    fn load_game_state(&mut self, _: GameState) {
        unimplemented!()
    }
    fn log_game_state(&self) -> std::string::String {
        unimplemented!()
    }
}

#[derive(Copy, Clone)]
pub struct GameInput {}

impl GameInput {
    pub fn from_action(action: Actions) -> Self {
        return Self {};
    }
}

impl RMercuryInput for GameInput {
    fn to_bits(&self) -> std::vec::Vec<u8> {
        unimplemented!()
    }
    fn from_bits(_: std::vec::Vec<u8>) -> Self {
        unimplemented!()
    }
    fn get_player_id(&self) -> usize {
        unimplemented!()
    }
    fn set_player_id(&mut self, _: usize) {
        unimplemented!()
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Actions {
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    Quit,
}

/**
 *
 * GFX implementation
 *
 */
pub fn should_quit(inputs: &Vec<Actions>) -> bool {
    for action in inputs.iter() {
        match action {
            Actions::Quit => {
                return true;
            }
            _ => {}
        }
    }

    return false;
}

pub struct PongGfx {
    sdl_context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    pub event_pump: sdl2::EventPump,
    pub canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl PongGfx {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();

        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem.window("Window", 800, 600).build().unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        let mut event_pump = sdl_context.event_pump().unwrap();

        return Self {
            sdl_context: sdl_context,
            video_subsystem: video_subsystem,
            event_pump: event_pump,
            canvas: canvas,
        };
    }

    pub fn render(&mut self, game_state: &GameState) {
        // Background color
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();

        // Draw players
        self.canvas.set_draw_color(Color::RGB(255, 210, 0));
        self.canvas.fill_rect(Rect::new(20, 20, 20, 20)).unwrap();

        // Final present
        self.canvas.present();
    }

    pub fn get_player_input(&mut self) -> Vec<Actions> {
        let mut player_actions = vec![];
        for event in self.event_pump.poll_iter() {
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
}
