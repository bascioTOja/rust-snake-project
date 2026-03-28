use crossterm::style::Color;
use crate::engine::Display;
use crate::snake::snake::Snake;
use crate::tetris::{get_piece_color, PieceType};

pub struct SnakeBoard {
    pub origin: (usize, usize),
    pub size: (u8, u8),
    tile_colors: [Color; 2],
}

impl SnakeBoard {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            origin: (x, y),
            tile_colors: [Color::Rgb {r: 46, g: 51, b: 54}, Color::Rgb {r: 63, g: 69, b: 73}],
            size: (20, 20),
        }
    }

    pub fn get_free_spots(&self, snake: &Snake) -> Vec<(u8, u8)> {
        let mut free_spots = Vec::new();
        for y in self.origin.1 as u8..self.size.1 {
            for x in self.origin.0 as u8..self.size.0 {
                if !snake.check_in_snake(x, y) {
                    free_spots.push((x, y));
                }
            }
        }
        free_spots
    }

    pub fn draw(&self, display: &mut Display) {
        display.set_area_char(self.origin.0, self.origin.1, self.size.0 as usize * 2, self.size.1 as usize, ' ');

        for y in 0..self.size.1 as usize {
            for x in 0..self.size.0 as usize {
                let color = self.tile_colors[(x + y) % 2];
                display.set_area_bg(self.origin.0 + x * 2, self.origin.1 + y, 2, 1, color);

            }
        }
    }
}