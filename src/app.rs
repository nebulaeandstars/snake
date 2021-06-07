pub struct App {
    gl: GlGraphics,
    background_color: [f32; 4],
    floor_color: [f32; 4],
    grid_size: f32,
    board_size: (f32, f32),
    input_buffer: InputBuffer,
    snake: Snake,
    food: Food,
}

use opengl_graphics::GlGraphics;
use piston::input::{Button, Key, RenderArgs};

use crate::direction::Direction;
use crate::input::InputBuffer;
use crate::snake::Snake;
use crate::square::Food;
use crate::square::Square;

impl App {
    pub fn new(
        gl: GlGraphics,
        background_color: [f32; 4],
        floor_color: [f32; 4],
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
            background_color: background_color,
            floor_color: floor_color,
            grid_size: grid_size,
            board_size: board_size,
            input_buffer: InputBuffer::new(),
            snake: snake,
            food: food,
        }
    }

    pub fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let background: [f32; 4] = self.background_color;
        let floor_color: [f32; 4] = self.floor_color;
        let grid_size: f32 = self.grid_size;
        let board_size: (f32, f32) = self.board_size;
        let snake_squares: &Vec<Square> = self.snake.get_squares();
        let food: &Food = &self.food;
        const FLOOR_BUFFER: f32 = 3.0;

        // let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // draw the background and floor
            clear(background, gl);

            let board_offset = (2.0, 2.0);
            let floor_render = rectangle::rectangle_by_corners(
                (0.0 - FLOOR_BUFFER).into(),
                (0.0 - FLOOR_BUFFER).into(),
                ((board_size.0 + 1.0) * grid_size + FLOOR_BUFFER).into(),
                ((board_size.1 + 1.0) * grid_size + FLOOR_BUFFER).into(),
            );
            let transform = c.transform.trans(
                (board_offset.0 * grid_size).into(),
                (board_offset.1 * grid_size).into(),
            );
            rectangle(floor_color, floor_render, transform, gl);

            // draw the snake's tail
            for snake_square in snake_squares[1..].iter() {
                let size = snake_square.get_size();
                let x = (snake_square.get_position().0 + (1.0 - size) / 2.0) * grid_size;
                let y = (snake_square.get_position().1 + (1.0 - size) / 2.0) * grid_size;

                let snake_square_render =
                    rectangle::square(x.into(), y.into(), (grid_size * size).into());

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
            let x = (snake_head.get_position().0 + (1.0 - size) / 2.0) * grid_size;
            let y = (snake_head.get_position().1 + (1.0 - size) / 2.0) * grid_size;

            let snake_square_render =
                rectangle::square(x.into(), y.into(), (grid_size * size).into());
            rectangle(
                snake_head.get_color().clone(),
                snake_square_render,
                transform,
                gl,
            );

            // draw the food
            let size = food.get_size();
            let x = (food.get_position().0 + (1.0 - size) / 2.0) * grid_size;
            let y = (food.get_position().1 + (1.0 - size) / 2.0) * grid_size;

            let food_square_render =
                rectangle::square(x.into(), y.into(), (grid_size * size).into());
            rectangle(food.get_color().clone(), food_square_render, transform, gl);
        });
    }

    pub fn update(&mut self) {
        // update the snake's direction if needed.
        self.update_snake_direction();

        // move the snake.
        self.snake.iterate_movement();

        // if the new head position overlaps the food, grow the snake and spawn new food.
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

        // if the snake overlaps its tail or collides with a wall, reset the game.
        if self.snake.overlaps(snake_head) || self.snake_head_is_outside_game() {
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

    pub fn handle_button(&mut self, button: Button) {
        // match the user input to get the direction
        let direction: Option<Direction> = match button {
            Button::Keyboard(Key::Up) => Some(Direction::Up),
            Button::Keyboard(Key::Down) => Some(Direction::Down),
            Button::Keyboard(Key::Left) => Some(Direction::Left),
            Button::Keyboard(Key::Right) => Some(Direction::Right),
            _ => None,
        };

        if direction.is_some() {
            self.input_buffer
                .push(direction.unwrap(), self.snake.get_direction());
        }
    }

    fn update_snake_direction(&mut self) {
        let direction = self.input_buffer.pop();

        if direction.is_some() {
            self.snake.update_direction(direction.unwrap());
        }
    }

    fn snake_head_is_outside_game(&self) -> bool {
        let head_pos = self.snake.get_head_pos();

        if head_pos.0 < 0.0
            || head_pos.1 < 0.0
            || head_pos.0 > self.board_size.0
            || head_pos.1 > self.board_size.1
        {
            return true;
        }
        else {
            return false;
        }
    }
}


fn get_random_grid_position(board_width: f32, board_height: f32) -> (f32, f32) {
    return (
        (rand::random::<f32>() * board_width).floor(),
        (rand::random::<f32>() * board_height).floor(),
    );
}
