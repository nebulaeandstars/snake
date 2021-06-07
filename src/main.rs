use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::{ButtonEvent, ButtonState, RenderEvent, UpdateEvent};
use piston::window::WindowSettings;

mod colors;
mod direction;
mod game;
mod input;
mod snake;
mod square;

use colors::*;
use game::Game;

fn main() {
    let opengl = OpenGL::V3_2;

    const GRID_SIZE: f64 = 16.0;
    const BOARD_SIZE: (f64, f64) = (20.0, 10.0);

    let game_colors: GameColors = GameColors::new(
        [0.4, 0.9, 0.75, 1.0], // snake head
        [0.4, 0.75, 0.9, 1.0], // snake tail
        [0.2, 0.2, 0.2, 1.0],  // background
        [0.3, 0.3, 0.3, 1.0],  // floor
        [0.9, 0.4, 0.4, 1.0],  // food
    );

    let mut window: GlutinWindow = WindowSettings::new("snake", [600, 400])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .samples(2)
        .build()
        .unwrap();


    let mut game = Game::new(GlGraphics::new(opengl), game_colors, GRID_SIZE, BOARD_SIZE);


    let mut events = Events::new(EventSettings::new()).ups(4);
    while let Some(event) = events.next(&mut window) {
        if let Some(args) = event.render_args() {
            game.render(&args);
        }

        if let Some(_) = event.update_args() {
            game.update();
        }

        if let Some(args) = event.button_args() {
            if args.state == ButtonState::Press {
                game.handle_button(args.button);
            }
        }
    }
}
