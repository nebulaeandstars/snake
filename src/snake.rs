use crate::colors::Color;

/// Represents the player. Contains a vector of squares, a direction, and a colour. Responsible for
/// updating its own position on the board.
pub struct Snake {
    squares: Vec<Square>,
    direction: Direction,
    colors: (Color, Color),
}


use crate::direction::Direction;
use crate::square::Square;

// size of the squares that make up a snake.
const HEAD_SIZE: f64 = 1.0;
const TAIL_SIZE: f64 = 0.9375;

impl Snake {
    // returns a new Snake, given an intial position, a direction, and a color.
    pub fn new(position: (f64, f64), direction: Direction, colors: (Color, Color)) -> Snake {
        // start the snake with two body parts occupying the same position.
        // it will "uncoil" when the player moves and the game starts.
        let squares: Vec<Square> = vec![
            Square::new(position, colors.0, HEAD_SIZE),
            Square::new(position, colors.1, TAIL_SIZE),
        ];

        Snake {
            squares: squares,
            direction: direction,
            colors: colors,
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
            let tail_position: (f64, f64) =
                self.squares[self.squares.len() - 1].get_position().clone();

            // add a new square, overlapping the tail.
            self.squares
                .push(Square::new(tail_position, self.colors.1, TAIL_SIZE));
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
        self.squares[0].set_color(self.colors.1);
        self.squares[0].set_size(TAIL_SIZE);

        // create a new head by addding a new square to the front of the snake.
        self.squares
            .insert(0, Square::new(position, self.colors.0, HEAD_SIZE));

        // pop the tail of the snake out of the list.
        self.squares.remove(self.squares.len() - 1);
    }


    pub fn overlaps(&self, position: (f64, f64)) -> bool {
        for square in self.get_squares()[1..].iter() {
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
    pub fn get_head_pos(&self) -> (f64, f64) {
        return self.squares[0].get_position();
    }

    pub fn get_direction(&self) -> Direction {
        return self.direction.clone();
    }
}
