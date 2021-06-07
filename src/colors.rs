pub type Color = [f32; 4];

pub struct GameColors {
    snake_head: Color,
    snake_tail: Color,
    background: Color,
    floor: Color,
    food: Color,
}

impl GameColors {
    pub fn new(
        snake_head: Color,
        snake_tail: Color,
        background: Color,
        floor: Color,
        food: Color,
    ) -> GameColors {
        GameColors {
            snake_head: snake_head,
            snake_tail: snake_tail,
            background: background,
            floor: floor,
            food: food,
        }
    }

    pub fn get_snake_colors(&self) -> (Color, Color) {
        (self.snake_head, self.snake_tail)
    }

    pub fn get_room_colors(&self) -> (Color, Color) {
        (self.background, self.floor)
    }

    pub fn get_food_color(&self) -> Color {
        self.food
    }
}
