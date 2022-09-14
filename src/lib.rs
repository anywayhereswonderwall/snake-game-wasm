use std::collections::HashSet;
use rand::Rng;
use wasm_bindgen::prelude::*;

// RETURNS RANDON NUMBER IN GIVEN RANGE
pub fn random_range(min: usize, max: usize) -> usize {
    let mut rng = rand::thread_rng();
    let value = rng.gen_range(min..max);
    value
}
// CHECKS FOR DUPLICATES IN A VECTOR
pub fn contains_duplicates(vec: &Vec<usize>) -> bool {
    let mut values = HashSet::new();
    for e in vec {
        values.insert(e);
    };
    !(values.len() == vec.len())
}


#[wasm_bindgen]
pub enum Direction {
    Up,
    Right,
    Down,
    Left
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
    // CONSTRUCTOR
    pub fn new(width: usize, height: usize) -> Self {
        Self { 
            width, 
            height, 
            snake: vec![17],
            direction: Direction::Left,
            food: random_range(0, width * height),
            lost: false
        }
    }

    // HANDLES USER'S DIRECTIONS INPUT
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
        self.snake_self_collision();
        if self.lost {
            return;
        }


        if self.did_eat() {
            self.generate_food();
            self.snake_move(true);
        } else {
            self.snake_move(false);
        }
    }
    
    // SNAKE'S AUTONOMOUS/INDEPENDENT MOVEMENT
    // ALSO DETECTS COLLISIONS WITH BOARD BORDERS
    pub fn snake_move(&mut self, did_eat: bool) {
        let cur_head = self.snake[0];
        let next_head = match &self.direction {
            Direction::Up => cur_head - self.width,
            Direction::Right => cur_head + 1,
            Direction::Down => cur_head + self.width,
            Direction::Left => cur_head - 1,
        };
        // CHECKING FOR BOARD BORDER COLLISION
        // 
        // BOTTOM AND TOP BORDER 
        if &self.snake[0] > &(&self.width * &self.height)  {
            self.lost = true;
            return;
        }
        if &self.snake[0] < &0 {
            self.lost = true;
            return;
        }
        // LEFT AND RIGHT BORDER
        let cur_row = cur_head / self.width;
        let cur_column = cur_head % self.width;
        let next_row = next_head / self.width;
        let next_column = next_head % self.width;
        if (cur_row != next_row) & (cur_column != next_column) {
            self.lost = true;
            return;
        }
        self.snake.insert(0, next_head);
        // IF SNAKE DID NOT EAT, POP TAIL; THEREFORE CREATE AN ILLUSION OF MOVEMENT
        // OTHERWISE GROW SNAKE BY NOT REMOVING TAIL
        if !did_eat {
            self.snake.pop();
        }
    }



    // CHECKS FOR SNAKE'S COLLISION WITH ITSELF
    pub fn snake_self_collision(&mut self) {
        if contains_duplicates(&self.snake) {
            self.lost = true;
        }
    }

    pub fn did_eat(&self) -> bool {
        self.snake[0] == self.food
    }

    // RANDOMLY PLACES NEW FOOD ON THE BOARD 
    pub fn generate_food(&mut self) {
        self.food = random_range(0, self.width * self.height);
    }


    // 
    // 
    // GETTER FUNCTIONS FOR JS
    pub fn snake_body(&self) -> *const usize {
        self.snake.as_ptr()
    }
    pub fn snake_length(&self) -> usize {
        self.snake.len()
    }
    pub fn food(&self) -> usize {
        self.food
    }
    pub fn lost(&self) -> bool {
        self.lost
    }
}