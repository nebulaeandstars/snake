use crate::direction::Direction;
use std::collections::VecDeque;

pub struct InputBuffer {
    motions: VecDeque<Direction>,
}

const QUEUE_CAPACITY: usize = 4;

impl InputBuffer {
    pub fn new() -> InputBuffer {
        InputBuffer {
            motions: VecDeque::with_capacity(QUEUE_CAPACITY),
        }
    }

    pub fn push(&mut self, direction: Direction, mut current_direction: Direction) {
        let next_direction = self.lookahead(0);
        if next_direction.is_some() {
            current_direction = next_direction.unwrap();
        }

        if direction != current_direction && direction != current_direction.opposite() {
            self.motions.push_back(direction);
        }
    }

    pub fn pop(&mut self) -> Option<Direction> {
        return self.motions.pop_front();
    }

    pub fn lookahead(&self, index: usize) -> Option<Direction> {
        if index >= self.motions.len() {
            None
        }
        else {
            Some(self.motions[index])
        }
    }
}
