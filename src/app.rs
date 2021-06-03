pub struct App {
    gl: GlGraphics,
    background: [f32; 4],
    grid_size: f32,
    board_size: (f32, f32),
    snake: Snake,
    food: Food,
}

use opengl_graphics::GlGraphics;
use piston::input::{Button, Key, RenderArgs};

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
    ) -> App {
        println!("starting...");

        // get an initial snake and food
        let snake_pos = get_random_grid_position(board_size.0, board_size.1);
        let snake = Snake::new(snake_pos, Direction::Right);

        let mut food_pos = get_random_grid_position(board_size.0, board_size.1);
        while snake_pos != food_pos && snake.overlaps(food_pos) {
            food_pos = get_random_grid_position(board_size.0, board_size.1);
        }
        let food = Food::new_food(food_pos);


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
        let snake_squares: &Vec<Square> = self.snake.get_squares();
        let food: &Food = &self.food;

        // let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            clear(background, gl);

            // draw the snake's tail
            for snake_square in snake_squares[1..].iter() {
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

            // draw the snake's head
            let snake_head = &snake_squares[0];
            let size = snake_head.get_size();
            let x = (snake_head.get_position().0 + 1.0 - (size / 2.0)) * grid_size;
            let y = (snake_head.get_position().1 + 1.0 - (size / 2.0)) * grid_size;

            let snake_square_render = rectangle::square(
                snake_head.get_position().0.into(),
                snake_head.get_position().1.into(),
                (grid_size * size).into(),
            );
            let transform = c.transform.trans(x.into(), y.into());
            rectangle(
                snake_head.get_color().clone(),
                snake_square_render,
                transform,
                gl,
            );

            // draw the food
            let size = food.get_size();
            let x = (food.get_position().0 + 1.0 - (size / 2.0)) * grid_size;
            let y = (food.get_position().1 + 1.0 - (size / 2.0)) * grid_size;

            let food_square_render = rectangle::square(
                food.get_position().0.into(),
                food.get_position().1.into(),
                (grid_size * size).into(),
            );
            let transform = c.transform.trans(x.into(), y.into());
            rectangle(food.get_color().clone(), food_square_render, transform, gl);
        });
    }

    pub fn update(&mut self) {
        // move the snake.
        self.snake.iterate_movement();

        // if the new head position overlaps the food,
        let snake_head = self.snake.get_head_pos();
        let mut food_pos = self.food.get_position();

        if snake_head == self.food.get_position() {
            // move the food to somewhere else until it hits a valid position.
            while food_pos == snake_head || self.snake.overlaps(food_pos) {
                food_pos = get_random_grid_position(self.board_size.0, self.board_size.1);
            }
            self.food.set_position(food_pos);

            // and grow the snake
            self.snake.grow(1);
        }

        if self.snake.overlaps(snake_head) {
            // get a new snake and food
            let snake_pos = get_random_grid_position(self.board_size.0, self.board_size.1);
            let snake = Snake::new(snake_pos, Direction::Right);

            let mut food_pos = get_random_grid_position(self.board_size.0, self.board_size.1);
            while snake_pos != food_pos && snake.overlaps(food_pos) {
                food_pos = get_random_grid_position(self.board_size.0, self.board_size.1);
            }
            let food = Food::new_food(food_pos);

            // then set throw away the current snake and food for the new ones
            self.snake = snake;
            self.food = food;
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


fn get_random_grid_position(board_width: f32, board_height: f32) -> (f32, f32) {
    return (
        (rand::random::<f32>() * board_width).floor(),
        (rand::random::<f32>() * board_height).floor(),
    );
}
