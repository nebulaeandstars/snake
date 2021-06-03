use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventLoop, EventSettings, Events};
use piston::input::{ButtonEvent, ButtonState, RenderEvent, UpdateEvent};
use piston::window::WindowSettings;

mod app;
mod direction;
mod snake;
mod square;

use app::App;
use direction::Direction;
use snake::Snake;


fn main() {
    let opengl = OpenGL::V3_2;

    const BACKGROUND_COLOR: [f32; 4] = [0.2, 0.2, 0.2, 1.0];
    const GRID_SIZE: f32 = 16.0;
    const BOARD_SIZE: (f32, f32) = (10.0, 10.0);

    let mut window: GlutinWindow = WindowSettings::new("snake", [600, 400])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .samples(2)
        .build()
        .unwrap();


    let mut app = App::new(
        GlGraphics::new(opengl),
        BACKGROUND_COLOR,
        GRID_SIZE,
        BOARD_SIZE,
        Snake::new((5.0, 5.0), Direction::Right),
    );


    let mut events = Events::new(EventSettings::new()).ups(4);
    while let Some(event) = events.next(&mut window) {
        if let Some(args) = event.render_args() {
            app.render(&args);
        }

        if let Some(_) = event.update_args() {
            app.update();
        }

        if let Some(args) = event.button_args() {
            if args.state == ButtonState::Press {
                app.handle_button(&args.button);
            }
        }
    }
}
