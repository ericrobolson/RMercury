extern crate sdl2;
use sdl2::{event::Event, keyboard::Keycode, pixels::Color, rect::Rect};

use time::{Duration, Instant};

extern crate rmercury;
use rmercury::{RMercuryGameInterface, RMercuryInput};

pub struct GameInterface {
    game_state: GameState,
}

const MILLISECONDS_IN_SECOND: u64 = 1000;
const GFX_FPS: u64 = 60;

pub const ARENA_X_START: i32 = 80;
pub const ARENA_Y_START: i32 = 60;

pub const ARENA_WIDTH: i32 = 640;
pub const ARENA_HEIGHT: i32 = 480;

pub const WINDOW_WIDTH: u32 = 800;
pub const WINDOW_HEIGHT: u32 = 600;

const PLAYER_MOVE_SPEED: i32 = 4;

const BALL_RADIUS: usize = 4;
const PADDLE_WIDTH: usize = 5;
const PADDLE_HEIGHT: usize = 30;

impl GameInterface {
    pub fn new() -> Self {
        let player_offset = 20;

        let y_start = ARENA_Y_START + (ARENA_HEIGHT) / 2;

        let player1_start = Coordinate::new(ARENA_X_START + player_offset, y_start);

        let player_2_x = ARENA_X_START + ARENA_WIDTH - player_offset;

        let player2_start = Coordinate::new(player_2_x, y_start);
        let ball_start = Coordinate::new(ARENA_X_START + (ARENA_WIDTH / 2), y_start);

        return Self {
            game_state: GameState {
                player1_coordinates: player1_start,
                player1_velocity: Coordinate::new(0, 0),
                player2_coordinates: player2_start,
                player2_velocity: Coordinate::new(0, 0),
                ball_coordinates: ball_start,
                ball_velocity: Coordinate::new(-2, 2),
            },
        };
    }
}

impl RMercuryGameInterface<GameState, GameInput> for GameInterface {
    fn current_game_state(&self) -> GameState {
        return self.game_state;
    }

    fn advance_frame(&mut self, inputs: std::vec::Vec<GameInput>) {
        //
        // Controls
        //
        {
            for input in inputs.iter() {
                let mut move_up = false;
                let mut move_down = false;

                if input.action == Actions::MoveUp {
                    move_up = true;
                    move_down = false;
                } else if input.action == Actions::MoveDown {
                    move_up = false;
                    move_down = true;
                }

                let mut vel_change = 0;
                if move_up {
                    vel_change = -PLAYER_MOVE_SPEED;
                } else if move_down {
                    vel_change = PLAYER_MOVE_SPEED;
                }

                if input.get_player_id() == 1 {
                    // Player 1
                    self.game_state.player1_velocity.y = vel_change;
                } else {
                    // Player 2
                    self.game_state.player2_velocity.y = vel_change;
                }
            }
        }

        //
        // Collision Checks
        //
        {
            // Ball arena bounds checks
            {
                // Y arena collision
                if ((self.game_state.ball_coordinates.y - BALL_RADIUS as i32) < ARENA_Y_START)
                    || ((self.game_state.ball_coordinates.y + BALL_RADIUS as i32)
                        > (ARENA_Y_START + ARENA_HEIGHT))
                {
                    // Flip ball y dir
                    self.game_state.ball_velocity.y = -self.game_state.ball_velocity.y;
                }

                // X arena collision
                if ((self.game_state.ball_coordinates.x - BALL_RADIUS as i32) < ARENA_X_START)
                    || ((self.game_state.ball_coordinates.x + BALL_RADIUS as i32)
                        > (ARENA_X_START + ARENA_WIDTH))
                {
                    // Flip ball y dir
                    self.game_state.ball_velocity.x = -self.game_state.ball_velocity.x;
                }
            }

            // Ball paddle checks
            let ball_min_x = self.game_state.ball_coordinates.x - BALL_RADIUS as i32;
            let ball_min_y = self.game_state.ball_coordinates.y - BALL_RADIUS as i32;
            let ball_max_x = self.game_state.ball_coordinates.x + BALL_RADIUS as i32;
            let ball_max_y = self.game_state.ball_coordinates.y + BALL_RADIUS as i32;
            {
                // Player 1 checks
                let player_min_x = self.game_state.player1_coordinates.x - PADDLE_WIDTH as i32;
                let player_min_y = self.game_state.player1_coordinates.y - PADDLE_HEIGHT as i32;
                let player_max_x = self.game_state.player1_coordinates.x + PADDLE_WIDTH as i32;
                let player_max_y = self.game_state.player1_coordinates.y + PADDLE_HEIGHT as i32;

                if was_collision(
                    ball_min_x,
                    ball_max_x,
                    ball_min_y,
                    ball_max_y,
                    player_min_x,
                    player_max_x,
                    player_min_y,
                    player_max_y,
                ) {
                    self.game_state.ball_velocity.x = -self.game_state.ball_velocity.x;
                    self.game_state.ball_velocity.y = -self.game_state.ball_velocity.y;
                }

                // Player 2 checks
                let player_min_x = self.game_state.player2_coordinates.x - PADDLE_WIDTH as i32;
                let player_min_y = self.game_state.player2_coordinates.y - PADDLE_HEIGHT as i32;
                let player_max_x = self.game_state.player2_coordinates.x + PADDLE_WIDTH as i32;
                let player_max_y = self.game_state.player2_coordinates.y + PADDLE_HEIGHT as i32;

                if was_collision(
                    ball_min_x,
                    ball_max_x,
                    ball_min_y,
                    ball_max_y,
                    player_min_x,
                    player_max_x,
                    player_min_y,
                    player_max_y,
                ) {
                    self.game_state.ball_velocity.x = -self.game_state.ball_velocity.x;
                    self.game_state.ball_velocity.y = -self.game_state.ball_velocity.y;
                }
            }
        }

        //
        // Velocities
        //
        {
            // Ball velocity changes
            {
                self.game_state.ball_coordinates.x += self.game_state.ball_velocity.x;
                self.game_state.ball_coordinates.y += self.game_state.ball_velocity.y;
            }

            // Player 1 velocity
            let proposed_move =
                self.game_state.player1_coordinates.y + self.game_state.player1_velocity.y;

            if (proposed_move > ARENA_Y_START + PADDLE_HEIGHT as i32)
                && (proposed_move < ARENA_Y_START + ARENA_HEIGHT - PADDLE_HEIGHT as i32)
            {
                self.game_state.player1_coordinates.y += self.game_state.player1_velocity.y;
            }

            // Player 2 velocity
            let proposed_move =
                self.game_state.player2_coordinates.y + self.game_state.player2_velocity.y;

            if (proposed_move < ARENA_Y_START + PADDLE_HEIGHT as i32)
                && (proposed_move > ARENA_Y_START + ARENA_HEIGHT - PADDLE_HEIGHT as i32)
            {
                self.game_state.player2_coordinates.y += self.game_state.player2_velocity.y;
            }
        }
    }

    fn load_game_state(&mut self, game_state: GameState) {
        self.game_state = game_state;
    }
    fn log_game_state(&self) -> std::string::String {
        return format!(
            "
            \nplayer1: [x: {}, y: {}, xvel:{}, yvel{}],
            \nplayer2: [x: {}, y: {}, xvel:{}, yvel{}],       
            \nball: [x: {}, y: {}, xvel:{}, yvel{}]
            ",
            self.game_state.player1_coordinates.x,
            self.game_state.player1_coordinates.y,
            self.game_state.player1_velocity.x,
            self.game_state.player1_velocity.y,
            self.game_state.player2_coordinates.x,
            self.game_state.player2_coordinates.y,
            self.game_state.player2_velocity.x,
            self.game_state.player2_velocity.y,
            self.game_state.ball_coordinates.x,
            self.game_state.ball_coordinates.y,
            self.game_state.ball_velocity.x,
            self.game_state.ball_velocity.y,
        );
    }
}

fn was_collision(
    a_x_min: i32,
    a_x_max: i32,
    a_y_min: i32,
    a_y_max: i32,
    b_x_min: i32,
    b_x_max: i32,
    b_y_min: i32,
    b_y_max: i32,
) -> bool {
    return a_x_min < b_x_max && a_x_max > b_x_min && a_y_min < b_y_max && a_y_max > b_y_min;
}

#[derive(Copy, Clone, Debug)]
pub struct Coordinate {
    pub x: i32,
    pub y: i32,
}

impl Coordinate {
    pub fn new(x: i32, y: i32) -> Self {
        return Self { x: x, y: y };
    }
}

#[derive(Copy, Clone, Debug)]
pub struct GameState {
    pub player1_coordinates: Coordinate,
    pub player1_velocity: Coordinate,
    pub player2_coordinates: Coordinate,
    pub player2_velocity: Coordinate,
    pub ball_coordinates: Coordinate,
    pub ball_velocity: Coordinate,
}

#[derive(Copy, Clone)]
pub struct GameInput {
    pub action: Actions,
    player_id: usize,
}

impl GameInput {
    pub fn from_action(action: Actions) -> Self {
        return Self {
            action: action,
            player_id: 0,
        };
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
        return self.player_id;
    }
    fn set_player_id(&mut self, player_id: usize) {
        self.player_id = player_id;
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
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
    frame_duration: time::Duration,
    last_frame_execution: time::Instant,
}

impl PongGfx {
    pub fn new() -> Self {
        let sdl_context = sdl2::init().unwrap();

        let video_subsystem = sdl_context.video().unwrap();
        let window = video_subsystem
            .window("Window", WINDOW_WIDTH, WINDOW_HEIGHT)
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        let mut event_pump = sdl_context.event_pump().unwrap();

        let frame_duration = Duration::milliseconds(MILLISECONDS_IN_SECOND as i64 / GFX_FPS as i64);

        let start = Instant::now();

        return Self {
            sdl_context: sdl_context,
            video_subsystem: video_subsystem,
            event_pump: event_pump,
            canvas: canvas,
            frame_duration: frame_duration,
            last_frame_execution: start,
        };
    }

    pub fn render(&mut self, game_state: &GameState) {
        let loop_start = self.last_frame_execution - Instant::now();
        if self.frame_duration <= loop_start {
            // Background color
            self.canvas.set_draw_color(Color::RGB(0, 0, 0));
            self.canvas.clear();

            // Draw arena
            self.canvas.set_draw_color(Color::RGB(255, 210, 0));
            self.canvas
                .fill_rect(Rect::new(
                    ARENA_X_START,
                    ARENA_Y_START,
                    ARENA_WIDTH as u32,
                    ARENA_HEIGHT as u32,
                ))
                .unwrap();

            // Draw players
            // Player 1
            self.canvas.set_draw_color(Color::RGB(255, 0, 0));
            self.canvas
                .fill_rect(Rect::new(
                    game_state.player1_coordinates.x - PADDLE_WIDTH as i32,
                    game_state.player1_coordinates.y - PADDLE_HEIGHT as i32,
                    (PADDLE_WIDTH * 2) as u32,
                    (PADDLE_HEIGHT * 2) as u32,
                ))
                .unwrap();

            // Player 2
            self.canvas.set_draw_color(Color::RGB(0, 0, 255));
            self.canvas
                .fill_rect(Rect::new(
                    game_state.player2_coordinates.x - PADDLE_WIDTH as i32,
                    game_state.player2_coordinates.y - PADDLE_HEIGHT as i32,
                    (PADDLE_WIDTH * 2) as u32,
                    (PADDLE_HEIGHT * 2) as u32,
                ))
                .unwrap();

            // Ball
            self.canvas.set_draw_color(Color::RGB(0, 0, 0));
            self.canvas
                .fill_rect(Rect::new(
                    game_state.ball_coordinates.x - BALL_RADIUS as i32,
                    game_state.ball_coordinates.y - BALL_RADIUS as i32,
                    (BALL_RADIUS * 2) as u32,
                    (BALL_RADIUS * 2) as u32,
                ))
                .unwrap();

            // Final present
            self.canvas.present();

            self.last_frame_execution = Instant::now();
        }
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
