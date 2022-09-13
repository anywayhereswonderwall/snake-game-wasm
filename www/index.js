import { SnakeGame, Direction } from "hello";
import * as wasm from "../pkg/hello_bg.wasm";

// Game constants
const GAME_WIDTH = 12;
const GAME_HEIGHT = 10;
const CELL_SIZE = 50;
const FOOD_COLOR = "#ff1100";
const SNAKE_COLOR = "#000000";
const snakeGame = SnakeGame.new(GAME_WIDTH, GAME_HEIGHT);
const canvas = document.getElementById("snake-canvas");
canvas.height = GAME_HEIGHT * CELL_SIZE;
canvas.width = GAME_WIDTH * CELL_SIZE;
const ctx = canvas.getContext("2d");

// Rendering function for the board
const drawGame = () => {
  ctx.beginPath();
  for (let x = 0; x <= GAME_WIDTH; x++) {
    ctx.moveTo(x * CELL_SIZE, 0);
    ctx.lineTo(x * CELL_SIZE, GAME_HEIGHT * CELL_SIZE);
  }
  for (let y = 0; y <= GAME_HEIGHT; y++) {
    ctx.moveTo(0, y * CELL_SIZE);
    ctx.lineTo(CELL_SIZE * GAME_WIDTH, y * CELL_SIZE);
  }
  ctx.stroke();
};

// Filler function for any given cell
const fillCell = (idx) => {
  const column = idx % GAME_WIDTH;
  const row = Math.floor(idx / GAME_WIDTH);
  ctx.beginPath();
  ctx.fillRect(column * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
  ctx.stroke();
};

const drawSnake = () => {
  const snakeBody = new Uint32Array(
    wasm.memory.buffer,
    snakeGame.snake_body(),
    snakeGame.snake_length()
  );
  ctx.fillStyle = SNAKE_COLOR;
  snakeBody.forEach((e) => fillCell(e));
};

const drawFood = () => {
  ctx.fillStyle = FOOD_COLOR;
  const food = snakeGame.food();
  fillCell(food);
};

document.addEventListener("keydown", (e) => {
  switch (e.code) {
    case "ArrowUp":
      snakeGame.change_direction(Direction.Up);
      break;
    case "ArrowRight":
      snakeGame.change_direction(Direction.Right);
      break;
    case "ArrowDown":
      snakeGame.change_direction(Direction.Down);
      break;
    case "ArrowLeft":
      snakeGame.change_direction(Direction.Left);
      break;
  }
});

function play() {
  setTimeout(() => {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    drawGame();
    drawSnake();
    drawFood();
    snakeGame.tick();
    requestAnimationFrame(play);
  }, 300);
}

play();
