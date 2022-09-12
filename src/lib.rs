use std::collections::VecDeque;

use wasm_bindgen::prelude::*;



#[wasm_bindgen]
pub struct SnakeGame {
    width: usize,
    height: usize,
    snake: VecDeque<usize>,
    lost: bool
}


#[wasm_bindgen]
impl SnakeGame {
    pub fn new(width: usize, height: usize) -> Self {
        Self { 
            width, 
            height, 
            snake: VecDeque::from([20]),
            lost: false
        }
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
}