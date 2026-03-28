use std::cmp::PartialEq;
use crossterm::style::Color;
use crate::engine::Display;
use crate::snake::food::Food;
use crate::snake::vector::Vector2D;

#[derive(Clone)]
pub struct SnakeBody {
    pub position: Vector2D,
    color: Color,
}

impl PartialEq for SnakeBody {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl SnakeBody {
    pub fn new(position: Vector2D, color: Color) -> SnakeBody {
        SnakeBody {
            position,
            color,
        }
    }

    pub fn can_eat_food(&self, food: &Food) -> bool {
        food.same_position(&self.position)
    }

    pub fn change_color(&mut self, color: Color) {
        self.color = color;
    }

    pub fn draw(&self, display: &mut Display) {
        self.position.draw(display, self.color);
    }
}