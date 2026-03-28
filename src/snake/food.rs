use crossterm::style::Color;
use crate::snake::vector::Vector2D;

pub struct Food {
    position: Vector2D,
    color: Color,
}

impl Food {
    pub fn new(position: Vector2D) -> Food {
        Food {
            position,
            color: Color::Rgb { r: 160, g: 74, b: 69 },
        }
    }

    pub fn same_position(&self, other: &Vector2D) -> bool {
        self.position == *other
    }
    
    pub fn set_position(&mut self, position: Vector2D) {
        self.position = position;
    }

    pub fn draw(&self, display: &mut crate::engine::Display) {
        self.position.draw(display, self.color);
    }
}