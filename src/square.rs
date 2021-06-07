use crate::colors::Color;

/// Represents a single non-empty tile on the board. Holds a position, a color, and a type.
pub struct Square {
    position: (f64, f64),
    color: Color,
    size: f64,
}

impl Square {
    // constructs a new square from a position a color, and a size.
    pub fn new(position: (f64, f64), color: Color, size: f64) -> Square {
        Square {
            position: position,
            color: color,
            size: size,
        }
    }

    // returns the square's position as a tuple.
    pub fn get_position(&self) -> (f64, f64) {
        return self.position.clone();
    }

    // returns the square's color
    pub fn get_color(&self) -> Color {
        return self.color.clone();
    }

    // returns the square's size
    pub fn get_size(&self) -> f64 {
        return self.size.clone();
    }


    // sets the square's color
    pub fn set_position(&mut self, position: (f64, f64)) {
        self.position = position;
    }

    // sets the square's color
    pub fn set_color(&mut self, color: Color) {
        self.color = color;
    }

    // sets the square's size
    pub fn set_size(&mut self, size: f64) {
        self.size = size;
    }
}


/// Variant of a square used for food.
pub type Food = Square;

const FOOD_SIZE: f64 = 0.6;

impl Food {
    pub fn new_food(position: (f64, f64), color: Color) -> Food {
        Food {
            position: position,
            color: color,
            size: FOOD_SIZE,
        }
    }
}
