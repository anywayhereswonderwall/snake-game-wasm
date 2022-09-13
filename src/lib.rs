use rand::Rng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum Direction {
    Up,
    Right,
    Down,
    Left
}
// pub type Position = (usize, usize);

pub fn random_range(min: usize, max: usize) -> usize {
    let mut rng = rand::thread_rng();
    let value = rng.gen_range(min..max);
    value
}

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
            food: random_range(0, width * height),
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
        if self.lost {
            return;
        }
        let cur_head = self.snake[0];
        let next_head = match &self.direction {
            Direction::Up => cur_head - self.width,
            Direction::Right => cur_head + 1,
            Direction::Down => cur_head + self.width,
            Direction::Left => cur_head - 1,
        };
        self.snake.pop();
        self.snake.insert(0, next_head);
        if self.snake.contains(&self.food) {
            self.generate_food();
        }
    }
    pub fn generate_food(&mut self) {
        self.food = random_range(0, self.width * self.height);
    } 
    pub fn snake_body(&self) -> *const usize {
        self.snake.as_ptr()
    }
    pub fn snake_length(&self) -> usize {
        self.snake.len()
    }
    pub fn food(&self) -> usize {
        self.food
    }
}