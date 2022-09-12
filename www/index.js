import { SnakeGame } from "hello";

// Game constants
const GAME_WIDTH = 12;
const GAME_HEIGHT = 10;
const CELL_SIZE = 50;

const snakeGame = SnakeGame.new(GAME_WIDTH, GAME_HEIGHT);

const canvas = document.getElementById("snake-canvas");
canvas.height = GAME_HEIGHT * CELL_SIZE;
canvas.width = GAME_WIDTH * CELL_SIZE;
const ctx = canvas.getContext("2d");

// Function thats draws SVG canvas path
// for the game's "world" (the grid)
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
drawGame();
