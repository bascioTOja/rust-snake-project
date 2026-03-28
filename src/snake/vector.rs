use crossterm::style::Color;
use crate::engine::Display;

#[derive(Clone)]
pub struct Vector2D {
    pub x: u8,
    pub y: u8,
}

impl PartialEq for Vector2D {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Vector2D {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    pub fn add(&self, other: &Vector2D) -> Vector2D {
        Vector2D {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    pub fn add_direction(&self, direction: u8) -> Vector2D {
        match direction {
            0 => Vector2D::new(self.x , self.y - 1), // Up
            1 => Vector2D::new(self.x + 1, self.y), // Right
            2 => Vector2D::new(self.x, self.y + 1), // Down
            3 => Vector2D::new(self.x - 1, self.y), // Left
            _ => Vector2D::new(self.x, self.y),
        }
    }

    pub fn draw(&self, display: &mut Display, color: Color) {
        display.set_area_bg(self.x as usize * 2, self.y as usize, 2, 1, color);
    }
}