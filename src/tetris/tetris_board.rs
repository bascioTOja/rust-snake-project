use crate::tetris::{FallingBlock, PieceType, SHAPES, get_piece_color};

use crate::engine::Display;
use crossterm::style::Color;

pub struct TetrisBoard {
    origin: (usize, usize),
    board: [[PieceType; 10]; 25],
}

impl TetrisBoard {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            origin: (x, y),
            board: [[PieceType::None; 10]; 25],
        }
    }

    pub fn is_occupied(&self, x_pos: usize, y_pos: usize) -> bool {
        if x_pos >= 10 || y_pos >= 25 {
            return true;
        }

        self.board[y_pos][x_pos] != PieceType::None
    }

    pub fn try_piece(&self, piece: PieceType, rotation: usize, x_pos: usize, y_pos: usize) -> bool {
        for y in 0..4 {
            for x in 0..4 {
                if SHAPES[piece as usize][rotation][y][x] == 1 {
                    if self.is_occupied(x_pos.wrapping_add(x), y_pos.wrapping_add(y)) {
                        return false;
                    }
                }
            }
        }
        return true;
    }

    pub fn place_piece(&mut self, piece: &FallingBlock) {
        for y in 0..4 {
            for x in 0..4 {
                if SHAPES[piece.piece_type as usize][piece.rotation][y][x] == 1 {
                    self.board[piece.y.wrapping_add(y)][piece.x.wrapping_add(x)] = piece.piece_type
                }
            }
        }
    }

    pub fn clear_board(&mut self) {
        for y in 0..25 {
            for x in 0..10 {
                self.board[y][x] = PieceType::None;
            }
        }
    }

    //draw without adding on the board
    pub fn draw_falling_piece(&mut self, display: &mut Display, falling_block: &FallingBlock) {
        for y in 0..4 {
            for x in 0..4 {
                if falling_block.y + y >= 5 {
                    if SHAPES[falling_block.piece_type as usize][falling_block.rotation][y][x] == 1
                    {
                        let color = get_piece_color(falling_block.piece_type);

                        display.insert_raw_str_fg_bg(
                            "🭼🭿", //"𜷂𜷖""🭼🭿"
                            self.origin.0 + falling_block.x.wrapping_add(x) * 2,
                            self.origin.1 + falling_block.y.wrapping_sub(5).wrapping_add(y),
                            Color::Black,
                            color,
                        );
                    }
                }
            }
        }
    }

    pub fn draw_shadow_piece(&mut self, display: &mut Display, falling_block: &FallingBlock) {
        let mut bottom_pos = falling_block.y;

        for i in 1..25 {
            let can_fall = self.try_piece(
                falling_block.piece_type,
                falling_block.rotation,
                falling_block.x,
                falling_block.y + i,
            );

            if can_fall {
                bottom_pos = falling_block.y + i;
            } else {
                break;
            }
        }

        for y in 0..4 {
            for x in 0..4 {
                if bottom_pos + y >= 5 {
                    if SHAPES[falling_block.piece_type as usize][falling_block.rotation][y][x] == 1
                    {
                        let color = get_piece_color(falling_block.piece_type);

                        display.insert_raw_str_fg_bg(
                            "▓▓", //"𜷂𜷖""🭼🭿""░░"
                            self.origin.0 + falling_block.x.wrapping_add(x) * 2,
                            self.origin.1 + bottom_pos.wrapping_sub(5).wrapping_add(y),
                            Color::Black,
                            color,
                        );
                    }
                }
            }
        }
    }

    pub fn draw_board(&mut self, display: &mut Display) {
        display.set_area_bg(self.origin.0, self.origin.1, 20, 20, Color::Black);
        display.set_area_char(self.origin.0, self.origin.1, 20, 20, ' ');

        for y in 0..20 {
            for x in 0..10 {
                let color = get_piece_color(self.board[y + 5][x]);

                display.set_area_bg(self.origin.0 + x * 2, self.origin.1 + y, 2, 1, color);

                // let fg_color = TetrisBoard::get_fg_color(self.board[y + 5][x]);
                if self.board[y + 5][x] as u8 != PieceType::None as u8 {
                    display.insert_raw_str_fg(
                        "🭼🭿", //"𜷂𜷖""🭼🭿"
                        self.origin.0 + x * 2,
                        self.origin.1 + y,
                        Color::Black,
                    );
                }
            }
        }
    }

    pub fn get_lines_to_clear(&mut self) -> Vec<usize> {
        let mut lines = Vec::new();

        for y in 0..25 {
            let mut is_full = true;
            for x in 0..10 {
                if self.board[y][x] as u8 == 0 {
                    is_full = false;
                }
            }

            if is_full {
                lines.push(y);
            }
        }

        lines
    }

    pub fn draw_line_overlay(
        &mut self,
        display: &mut Display,
        origin_x: usize,
        origin_y: usize,
        line_index: usize,
    ) {
        display.set_area_char(origin_x, origin_y + line_index - 5, 20, 1, '█');
        display.set_area_fg(origin_x, origin_y + line_index - 5, 20, 1, Color::White);
    }

    pub fn draw_game_over(&mut self, display: &mut Display) {
        display.set_area_char(self.origin.0, self.origin.1, 20, 20, ' ');
        display.set_area_bg(self.origin.0, self.origin.1, 20, 20, Color::Grey);

        display.insert_raw_str(
            &format!("{: ^20}", "GAME OVER :c"),
            self.origin.0,
            self.origin.1 + 9,
        );
        display.insert_raw_str(
            &format!("{: ^20}", "Press R to restart"),
            self.origin.0,
            self.origin.1 + 11,
        );
    }

    pub fn check_gameover(&self) -> bool {
        for y in 0..5 {
            for x in 0..10 {
                if self.board[y][x] != PieceType::None {
                    return true;
                }
            }
        }

        false
    }

    pub fn clear_line(&mut self, line_index: usize) {
        for x in 0..10 {
            self.board[line_index][x] = PieceType::None;
        }

        for y in (1..=line_index).rev() {
            self.board[y] = self.board[y - 1];
        }
    }
}
