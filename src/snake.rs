/// Represents the player. Contains a vector of squares, a direction, and a colour. Responsible for
/// updating its own position on the board.
pub struct Snake {
    squares: Vec<Square>,
    direction: Direction,
}


use crate::direction::Direction;
use crate::square::Square;

// size of the squares that make up a snake.
const SNAKE_HEAD_SIZE: f32 = 1.0;
const SNAKE_TAIL_SIZE: f32 = 0.9;
const SNAKE_HEAD_COLOR: [f32; 4] = [0.4, 0.9, 0.75, 1.0];
const SNAKE_TAIL_COLOR: [f32; 4] = [0.4, 0.75, 0.9, 1.0];

impl Snake {
    // returns a new Snake, given an intial position, a direction, and a color.
    pub fn new(position: (f32, f32), direction: Direction) -> Snake {
        // start the snake with two body parts occupying the same position.
        // it will "uncoil" when the player moves and the game starts.
        let squares: Vec<Square> = vec![
            Square::new(position, SNAKE_HEAD_COLOR, SNAKE_HEAD_SIZE),
            Square::new(position, SNAKE_TAIL_COLOR, SNAKE_TAIL_SIZE),
        ];

        Snake {
            squares: squares,
            direction: direction,
        }
    }


    // change the snake's direction of movement.
    pub fn update_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }


    // grow the snake (eg. after eating food) by a given amount.
    pub fn grow(&mut self, amount: u8) {
        for _ in 0..amount {
            // find the end of the snake
            let tail_position: (f32, f32) =
                self.squares[self.squares.len() - 1].get_position().clone();

            // add a new square, overlapping the tail.
            self.squares.push(Square::new(
                tail_position,
                SNAKE_TAIL_COLOR,
                SNAKE_TAIL_SIZE,
            ));
        }
    }


    // move the snake once according to its direction.
    pub fn iterate_movement(&mut self) {
        // get the current position of the snake
        // (equal to the position of the first square in the list)
        let mut position = self.squares[0].get_position().clone();

        // update the position according to the snake's current direction.
        match self.direction {
            Direction::Up => position = (position.0, position.1 - 1.0),
            Direction::Down => position = (position.0, position.1 + 1.0),
            Direction::Left => position = (position.0 - 1.0, position.1),
            Direction::Right => position = (position.0 + 1.0, position.1),
        }

        // set the old head square to be a tail square.
        self.squares[0].set_color(SNAKE_TAIL_COLOR);
        self.squares[0].set_size(SNAKE_TAIL_SIZE);

        // create a new head by addding a new square to the front of the snake.
        self.squares
            .insert(0, Square::new(position, SNAKE_HEAD_COLOR, SNAKE_HEAD_SIZE));

        // pop the tail of the snake out of the list.
        self.squares.remove(self.squares.len() - 1);
    }


    pub fn overlaps(&self, position: (f32, f32)) -> bool {
        for square in self.get_squares().iter() {
            if square.get_position() == position {
                return true;
            }
        }

        return false;
    }


    // return the vector of squares
    pub fn get_squares(&self) -> &Vec<Square> {
        return &self.squares;
    }

    // return the vector of squares
    pub fn get_head_pos(&self) -> (f32, f32) {
        return self.squares[0].get_position();
    }

    pub fn get_direction(&self) -> &Direction {
        return &self.direction;
    }
}
