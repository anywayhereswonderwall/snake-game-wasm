use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub struct SnakeGame {
    width: usize,
    height: usize,
}


#[wasm_bindgen]
impl SnakeGame {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
}