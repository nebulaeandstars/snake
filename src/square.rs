/// Represents a single non-empty tile on the board. Holds a position, a color, and a type.
pub struct Square {
    position: (f32, f32),
    color: [f32; 4],
    size: f32,
}

impl Square {
    // constructs a new square from a position a color, and a size.
    pub fn new(position: (f32, f32), color: [f32; 4], size: f32) -> Square {
        Square {
            position: position,
            color: color,
            size: size,
        }
    }

    // returns the square's position as a tuple.
    pub fn get_position(&self) -> (f32, f32) {
        return self.position.clone();
    }

    // returns the square's color
    pub fn get_color(&self) -> [f32; 4] {
        return self.color.clone();
    }

    // returns the square's size
    pub fn get_size(&self) -> f32 {
        return self.size.clone();
    }


    // sets the square's color
    pub fn set_position(&mut self, position: (f32, f32)) {
        self.position = position;
    }

    // sets the square's color
    pub fn set_color(&mut self, color: [f32; 4]) {
        self.color = color;
    }

    // sets the square's size
    pub fn set_size(&mut self, size: f32) {
        self.size = size;
    }
}


/// Variant of a square used for food.
pub type Food = Square;

const FOOD_COLOR: [f32; 4] = [0.9, 0.4, 0.4, 1.0];
const FOOD_SIZE: f32 = 0.6;

impl Food {
    pub fn new_food(position: (f32, f32)) -> Food {
        Food {
            position: position,
            color: FOOD_COLOR,
            size: FOOD_SIZE,
        }
    }
}
