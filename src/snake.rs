pub mod snake_board;
mod snake;
mod food;
mod snake_body;
mod vector;

use crossterm::event::KeyCode;
use crossterm::style::Color;
use rand::prelude::ThreadRng;
use rand::RngExt;
use crate::engine::{Display, GameObject, InputState};
use crate::snake::food::Food;
use crate::snake::snake::Snake;
use crate::snake::snake_board::SnakeBoard;
use crate::snake::vector::Vector2D;
use crate::tetris::{FallingBlock, PieceType, TetrisGame};

pub struct SnakeGame {
    board: SnakeBoard,
    food: Food,
    snake: Snake,

    rng: ThreadRng,
    move_timer: f32,
    time_to_move: f32,
}


fn new_snake() -> Snake {
    Snake::new(3, Vector2D::new(10, 5), 1)
}

impl SnakeGame {
    pub fn new() -> Self {
        Self {
            board: SnakeBoard::new(2, 2),
            food: Food::new(Vector2D::new(0, 0)),
            snake: new_snake(),
            rng: rand::rng(),
            move_timer: 0.0,
            time_to_move: 0.22,
        }
    }

    pub fn reset_game(&mut self) {
        self.snake = new_snake();
        self.place_random_food();
    }

    pub fn get_point(&mut self) {
        self.snake.grow();
        self.place_random_food();
    }

    pub fn move_snake(&mut self) {
        self.snake.move_snake();
        if (self.snake.check_food_collision(&self.food)) {
            self.get_point();
        }
    }

    pub fn place_random_food(&mut self) {
        let free_spots = self.board.get_free_spots(&self.snake);
        if free_spots.is_empty() {
            return;
        }

        let random_index = self.rng.random_range(0..free_spots.len());
        let (x, y) = free_spots[random_index];
        self.food.set_position(Vector2D::new(x, y));
    }

    pub fn handle_key(&mut self, input_state: &InputState) {
        if input_state.keys_pressed.is_empty() {
            return;
        }
        if input_state.is_key_pressed(KeyCode::Char('w')) {
            self.snake.append_move(0);
        }
        if input_state.is_key_pressed(KeyCode::Char('d')) {
            self.snake.append_move(1);
        }
        if input_state.is_key_pressed(KeyCode::Char('s')) {
            self.snake.append_move(2);
        }
        if input_state.is_key_pressed(KeyCode::Char('a')) {
            self.snake.append_move(3);
        }
    }

    pub fn check_collisions(&mut self) -> bool {
        self.snake.check_wall_collision(&self.board) || self.snake.check_self_collision()
    }

    pub fn time_to_move(&mut self, dt: f32) -> bool {
        self.move_timer += dt;
        if self.move_timer > self.time_to_move {
            self.move_timer = 0.0;
            return true;
        }
        false
    }

    pub fn draw(&mut self, display: &mut Display) {
        // DRAW GAME FRAME
        display.set_area_char(0, 0, 44, 23, '█');
        display.set_area_char(0, 0, 44, 1, '▄');
        display.set_area_char(0, 23, 44, 1, '▀');
        display.set_area_fg(0, 0, 44, 24, Color::Rgb{ r: 99, g: 159, b: 195 });

        self.board.draw(display);
        self.snake.draw(display);
        self.food.draw(display);
    }
}

impl GameObject for SnakeGame {
    fn start(&mut self, display: &mut Display) {
        self.place_random_food();
        self.draw(display);
    }

    fn update(&mut self, dt: f32, input_events: &InputState, display: &mut Display) {
        self.handle_key(input_events);

        if self.time_to_move(dt) {
            self.move_snake();
            if self.check_collisions() {
                self.reset_game();
            }
        }

        self.draw(display);
    }
}