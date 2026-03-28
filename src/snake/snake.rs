use crossterm::style::Color;
use crate::engine::Display;
use crate::snake::food::Food;
use crate::snake::snake_board::SnakeBoard;
use crate::snake::snake_body::SnakeBody;
use crate::snake::vector::Vector2D;

pub struct Snake {
    head_color: Color,
    body_color: Color,
    direction: u8,
    moves: Vec<u8>,
    body: Vec<SnakeBody>,
}

impl Snake {
    pub fn new(length: u8, start_position: Vector2D, direction: u8) -> Snake {
        let mut head_color = Color::Rgb{ r: 196, g: 141, b: 77 };
        let mut body_color = Color::Rgb{ r: 133, g: 196, b: 77 };

        let mut body = Vec::new();
        for i in 0..length {
            let position = Vector2D::new(start_position.x - i, start_position.y);
            body.push(SnakeBody::new(position, body_color));
        }

        Snake {
            head_color,
            body_color,
            direction,
            moves: Vec::new(),
            body,
        }
    }

    pub fn check_self_collision(&mut self) -> bool {
        let head = &self.body[0];

        for segment in self.body.iter().skip(1) {
            if segment == head {
                return true;
            }
        }

        false
    }

    pub fn check_food_collision(&mut self, food: &Food) -> bool {
        let head = &self.body[0];
        head.can_eat_food(food)
    }

    pub fn check_wall_collision(&mut self, board: &SnakeBoard) -> bool {
        let head = &self.body[0];
        head.position.x < board.origin.0 as u8 - 1 || head.position.x >= board.origin.0 as u8 + board.size.0 - 1 || head.position.y < board.origin.1 as u8 || head.position.y >= board.origin.1 as u8 + board.size.1
    }

    pub fn check_in_snake(&self, x: u8, y: u8) -> bool {
        for segment in self.body.iter() {
            if segment.position.x == x && segment.position.y == y {
                return true;
            }
        }
        false
    }

    pub fn append_move(&mut self, direction: u8) {
        let last_move = if self.moves.len() > 0 {&self.moves[self.moves.len() - 1]} else {&self.direction};
        if last_move == &direction {
            return;
        }

        if (self.direction + 2) % 4 == direction {
            return;
        }

        self.moves.push(direction);
    }

    pub fn update_direction(&mut self) {
        if self.moves.len() == 0 {
            return;
        }
        self.direction = self.moves.remove(0);
    }

    pub fn append_head(&mut self) {
        let head = &mut self.body[0];
        head.change_color(self.body_color);

    }

    pub fn move_snake(&mut self) {
        self.update_direction();
        let head = &mut self.body[0];
        head.change_color(self.body_color);
        let new_head_position = head.position.add_direction(self.direction);
        let new_head = SnakeBody::new(new_head_position, self.head_color);
        self.body.insert(0, new_head);
        self.body.pop();
    }

    pub fn grow(&mut self) {
        let last_body = &self.body[self.body.len() - 1];
        let new_body = SnakeBody::new(last_body.position.clone(), self.body_color);
        self.body.push(new_body);
    }

    pub fn draw(&self, display: &mut Display) {
        for (i, segment) in self.body.iter().enumerate() {
            segment.draw(display)
        }
    }
}