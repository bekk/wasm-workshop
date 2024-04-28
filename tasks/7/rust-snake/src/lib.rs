use wasm_bindgen::{prelude::*};

#[wasm_bindgen]
pub enum Direction {
    Up, Down, Left, Right
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize
}

#[wasm_bindgen]
pub struct World {
    width: usize,
    height: usize,
    // Assume first position of the vector is the head of the snake, and the rest is the body
    body: Vec<Position>,
    snake_direction: Direction
}

#[wasm_bindgen]
impl World {
    pub fn new() -> World {
        World {
            width: 50,
            height: 50,
            body: vec!(Position {x: 0, y: 0}),
            snake_direction: Direction::Right
        }
    }

    pub fn get_height(&self) -> usize {
        return self.height;
    }

    pub fn get_width(&self) -> usize {
        return self.width;
    }

    #[wasm_bindgen]
    pub fn get_body(&self) -> Vec<Position> {
        return self.body.clone();
    }

    pub fn update(&mut self) {
        self.body = match self.body.as_slice() {
            [head] => vec!(self.calculate_next_head_position(head)),
            [head, body @ .., _tail] => 
                [
                    vec!(self.calculate_next_head_position(head), *head),
                    body.iter().cloned().collect()
                ].concat(),
            _ => panic!("Impossible case")
        };
    }


    
    pub fn set_direction(&mut self, direction: Direction) {
        self.snake_direction = direction;
    }

    fn calculate_next_head_position(&self, previous_head: &Position) -> Position {
        match self.snake_direction {
            Direction::Up => Position {x: previous_head.x, y: (self.height + previous_head.y - 1) % self.height},
            Direction::Down => Position {x: previous_head.x, y: (previous_head.y + 1) % self.height},
            Direction::Left => Position {x: (self.width + previous_head.x - 1) % self.width, y: previous_head.y},
            Direction::Right => Position {x: (previous_head.x + 1) % self.width, y: previous_head.y}
        }
    }
}