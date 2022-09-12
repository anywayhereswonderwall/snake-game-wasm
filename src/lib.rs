

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum Direction {
    Up,
    Right,
    Down,
    Left
}
// pub type Position = (usize, usize);

#[wasm_bindgen]
pub struct SnakeGame {
    width: usize,
    height: usize,
    snake: Vec<usize>,
    direction: Direction,
    food: usize,
    lost: bool
}


#[wasm_bindgen]
impl SnakeGame {
    pub fn new(width: usize, height: usize) -> Self {
        Self { 
            width, 
            height, 
            snake: vec![20, 21, 22],
            direction: Direction::Left,
            food: 20,
            lost: false
        }
    }
    pub fn change_direction(&mut self, direction: Direction) {
        match (&self.direction, direction) {
        (Direction::Up, Direction::Right) => self.direction = Direction::Right,
        (Direction::Up, Direction::Left) => self.direction = Direction::Left,
        (Direction::Right, Direction::Up) => self.direction = Direction::Up,
        (Direction::Right, Direction::Down) => self.direction = Direction::Down,
        (Direction::Down, Direction::Right) => self.direction = Direction::Right,
        (Direction::Down, Direction::Left) => self.direction = Direction::Left,
        (Direction::Left, Direction::Up) => self.direction = Direction::Up,
        (Direction::Left, Direction::Down) => self.direction = Direction::Down,
        _ => {}
}
    }
    pub fn tick(&mut self) {
        let cur_head = self.snake[0];
        let next_head = match &self.direction {
            Direction::Up => cur_head + 1,
            Direction::Right => cur_head + 1,
            Direction::Down => cur_head - 1,
            Direction::Left => cur_head - 1,
        };
        self.snake.pop();
        self.snake.push(next_head);
    }
    pub fn snake_body(&self) -> *const usize {
        self.snake.as_ptr()
    }
    pub fn snake_length(&self) -> usize {
        self.snake.len()
    }
}