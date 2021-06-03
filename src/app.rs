pub struct App {
    gl: GlGraphics,
    background: [f32; 4],
    grid_size: f32,
    board_size: (f32, f32),
    snake: Snake,
    food: Food,
}

use opengl_graphics::GlGraphics;
use piston::input::{Button, Key, RenderArgs, UpdateArgs};

use crate::direction::Direction;
use crate::snake::Snake;
use crate::square::Food;
use crate::square::Square;

impl App {
    pub fn new(
        gl: GlGraphics,
        background: [f32; 4],
        grid_size: f32,
        board_size: (f32, f32),
        snake: Snake,
    ) -> App {
        println!("starting...");

        let rng = rand::thread_rng();
        let food = Food::new_food(calculate_food_pos(board_size.0, board_size.1));

        App {
            gl: gl,
            background: background,
            grid_size: grid_size,
            board_size: board_size,
            snake: snake,
            food: food,
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let background: [f32; 4] = self.background;
        let grid_size: f32 = self.grid_size;
        let snake: &Vec<Square> = self.snake.get_squares();
        let food: &Food = &self.food;

        // let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            clear(background, gl);

            // draw the snake
            for snake_square in snake.iter() {
                let size = snake_square.get_size();
                let x = (snake_square.get_position().0 + 1.0 - (size / 2.0)) * grid_size;
                let y = (snake_square.get_position().1 + 1.0 - (size / 2.0)) * grid_size;

                let snake_square_render = rectangle::square(
                    snake_square.get_position().0.into(),
                    snake_square.get_position().1.into(),
                    (grid_size * size).into(),
                );

                let transform = c.transform.trans(x.into(), y.into());

                rectangle(
                    snake_square.get_color().clone(),
                    snake_square_render,
                    transform,
                    gl,
                );
            }

            // draw the food
            let size = food.get_size();
            let x = (food.get_position().0 + 1.0 - (size / 2.0)) * grid_size;
            let y = (food.get_position().1 + 1.0 - (size / 2.0)) * grid_size;

            let snake_square_render = rectangle::square(
                food.get_position().0.into(),
                food.get_position().1.into(),
                (grid_size * size).into(),
            );

            let transform = c.transform.trans(x.into(), y.into());

            rectangle(food.get_color().clone(), snake_square_render, transform, gl);
        });
    }

    pub fn update(&mut self) {
        // move the snake.
        self.snake.iterate_movement();

        // if the new head position overlaps the food,
        let snake_head = self.snake.get_head_pos();
        if snake_head == self.food.get_position() {
            // move the food to somewhere else
            self.food
                .set_position(calculate_food_pos(self.board_size.0, self.board_size.1));
            // and grow the snake
            self.snake.grow(1);
        }
    }

    pub fn handle_button(&mut self, button: &Button) {
        let current_direction: Direction = self.snake.get_direction().clone();

        self.snake.update_direction(match button {
            &Button::Keyboard(Key::Up) if current_direction != Direction::Down => Direction::Up,
            &Button::Keyboard(Key::Down) if current_direction != Direction::Up => Direction::Down,
            &Button::Keyboard(Key::Left) if current_direction != Direction::Right => {
                Direction::Left
            }
            &Button::Keyboard(Key::Right) if current_direction != Direction::Left => {
                Direction::Right
            }
            _ => current_direction,
        });
    }
}


fn calculate_food_pos(width: f32, height: f32) -> (f32, f32) {
    return (
        (rand::random::<f32>() * width).floor(),
        (rand::random::<f32>() * height).floor(),
    );
}