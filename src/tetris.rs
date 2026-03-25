mod falling_block;
mod piece_queue;
mod shape_data;
mod tetris_board;

//MODULE IMPORTS
pub use falling_block::FallingBlock;
pub use piece_queue::PieceQueue;
pub use shape_data::{I_KICK_TABLE, JLTSZ_KICK_TABLE, PieceType, SHAPES, get_piece_color};
pub use tetris_board::TetrisBoard;

use crossterm::event::KeyCode;
use crossterm::style::Color;

use crate::engine::{Display, GameObject, InputState};

pub struct TetrisGame {
    board: TetrisBoard,
    falling_block: FallingBlock,
    piece_queue: PieceQueue,

    held_piece: PieceType,
    swapped_piece: bool,

    points: usize,
    line_clears: usize,
    level: usize,

    line_clear_timer: f32,
    drop_timer: f32,
}

impl TetrisGame {
    pub fn new() -> Self {
        Self {
            board: TetrisBoard::new(2, 2),
            falling_block: FallingBlock::new(PieceType::None),
            piece_queue: PieceQueue::new(),

            held_piece: PieceType::None,
            swapped_piece: false,

            points: 0,
            level: 1,
            line_clears: 0,

            line_clear_timer: 0.0,
            drop_timer: 0.0,
        }
    }

    pub fn try_move(&mut self, x_offset: isize, y_offset: isize) -> bool {
        let x_offset: usize = x_offset as usize;
        let y_offset: usize = y_offset as usize;

        let can_move = self.board.try_piece(
            self.falling_block.piece_type,
            self.falling_block.rotation,
            self.falling_block.x.wrapping_add(x_offset),
            self.falling_block.y.wrapping_add(y_offset),
        );

        if can_move {
            self.falling_block.x = self.falling_block.x.wrapping_add(x_offset);
            self.falling_block.y = self.falling_block.y.wrapping_add(y_offset);
        }

        can_move
    }

    pub fn try_rotate(&mut self, rotation: usize) -> bool {
        if self.falling_block.piece_type == PieceType::O {
            let can_rotate = self.board.try_piece(
                self.falling_block.piece_type,
                rotation,
                self.falling_block.x,
                self.falling_block.y,
            );

            if can_rotate {
                self.falling_block.rotation = rotation;
            }

            return can_rotate;
        }

        //ITERATE KICK TABLE FOR PIECE

        let mut kick_table = JLTSZ_KICK_TABLE[self.falling_block.rotation_mode(rotation)];

        if self.falling_block.piece_type == PieceType::I {
            kick_table = I_KICK_TABLE[self.falling_block.rotation_mode(rotation)];
        }

        for kick in kick_table.iter() {
            let can_rotate = self.board.try_piece(
                self.falling_block.piece_type,
                rotation,
                self.falling_block.x.wrapping_add(kick.0 as usize),
                self.falling_block.y.wrapping_add(kick.1 as usize),
            );

            if can_rotate {
                self.falling_block.rotation = rotation;
                self.falling_block.x = self.falling_block.x.wrapping_add(kick.0 as usize);
                self.falling_block.y = self.falling_block.y.wrapping_add(kick.1 as usize);
                return true;
            }
        }

        false
    }

    pub fn draw_next_piece(
        piece: PieceType,
        display: &mut Display,
        origin_x: usize,
        origin_y: usize,
    ) {
        for y in 0..4 {
            for x in 0..4 {
                if SHAPES[piece as usize][0][y][x] == 1 {
                    let color = get_piece_color(piece);

                    display.insert_raw_str_fg_bg(
                        "🭼🭿", //"𜷂𜷖""🭼🭿"
                        origin_x + x * 2,
                        origin_y + y,
                        Color::Black,
                        color,
                    );
                } else {
                    display.insert_raw_str_fg_bg(
                        "  ", //"𜷂𜷖""🭼🭿"
                        origin_x + x * 2,
                        origin_y + y,
                        Color::Black,
                        Color::Black,
                    );
                }
            }
        }
    }

    pub fn try_swap_piece(&mut self) {
        if !self.swapped_piece {
            let temp = self.held_piece;
            self.held_piece = self.falling_block.piece_type;
            self.swapped_piece = true;
            self.drop_timer = 0.0;

            if temp == PieceType::None {
                self.falling_block = FallingBlock::new(self.piece_queue.get_piece_from_queue());
            } else {
                self.falling_block = FallingBlock::new(temp);
            }
        }
    }

    pub fn update_stats(&mut self, line_clears: usize) {
        match line_clears {
            1 => {
                self.points += 40 * (self.level + 1);
            }
            2 => {
                self.points += 100 * (self.level + 1);
            }
            3 => {
                self.points += 300 * (self.level + 1);
            }
            4 => {
                self.points += 1200 * (self.level + 1);
            }
            _ => {}
        };

        self.line_clears += line_clears;
        self.level = 1 + self.line_clears / 10;
    }

    pub fn get_drop_interval(&self) -> f32 {
        (0.8 - self.level as f32 * 0.007).powf(self.level as f32)
    }

    pub fn drop_tile(&mut self) {
        let mut bottom_pos = self.falling_block.y;

        for i in 1..25 {
            let can_fall = self.board.try_piece(
                self.falling_block.piece_type,
                self.falling_block.rotation,
                self.falling_block.x,
                self.falling_block.y + i,
            );

            if can_fall {
                bottom_pos = self.falling_block.y + i;
            } else {
                break;
            }
        }

        self.falling_block.y = bottom_pos;

        self.swapped_piece = false;
        self.board.place_piece(&self.falling_block);

        self.falling_block = FallingBlock::new(self.piece_queue.get_piece_from_queue());
    }
}

impl GameObject for TetrisGame {
    fn start(&mut self, _display: &mut Display) {
        self.falling_block.piece_type = self.piece_queue.get_piece_from_queue();
    }

    fn update(&mut self, dt: f32, input_events: &InputState, display: &mut Display) {
        // GAME OVER HANDLER
        if self.board.check_gameover() {
            self.board.draw_game_over(display);

            if input_events.is_key_pressed(KeyCode::Char('r')) {
                self.level = 1;
                self.points = 0;
                self.line_clears = 0;
                self.board.clear_board();
                self.held_piece = PieceType::None;
                self.swapped_piece = false;
                self.piece_queue.clear();
                self.falling_block = FallingBlock::new(self.piece_queue.get_piece_from_queue())
            }

            return;
        }

        // LINE CLEARING LOGIC
        // PAUSES OTHER GAME EVENTS WHILE ACTIVE

        let lines = self.board.get_lines_to_clear();

        if lines.len() != 0 {
            self.line_clear_timer += dt;

            for line in &lines {
                self.board.draw_line_overlay(display, 2, 1, *line);
            }

            if self.line_clear_timer > 0.1 {
                self.update_stats(lines.len());

                for line in &lines {
                    self.board.clear_line(*line);
                }
                self.line_clear_timer = 0.0;
            }
            return;
        }

        self.drop_timer += dt;

        //INPUT HANDLING
        if input_events.is_key_pressed(KeyCode::Char('a')) {
            if self.try_move(-1, 0) {
                if self.falling_block.lock && self.falling_block.lock_tries > 0 {
                    self.falling_block.lock_tries -= 1;
                    self.falling_block.lock_timer = 0.0;
                }
            };
        }

        if input_events.is_key_pressed(KeyCode::Char('d')) {
            if self.try_move(1, 0) {
                if self.falling_block.lock && self.falling_block.lock_tries > 0 {
                    self.falling_block.lock_tries -= 1;
                    self.falling_block.lock_timer = 0.0;
                }
            };
        }

        if input_events.is_key_pressed(KeyCode::Char('w')) {
            if self.try_rotate((self.falling_block.rotation + 1) % 4) {
                if self.falling_block.lock && self.falling_block.lock_tries > 0 {
                    self.falling_block.lock_tries -= 1;
                    self.falling_block.lock_timer = 0.0;
                }
            };
        }

        if input_events.is_key_pressed(KeyCode::Char('s')) {
            if self.try_move(0, 1) {
                self.drop_timer = 0.0;
            }
        }

        if input_events.is_key_pressed(KeyCode::Char(' ')) {
            self.drop_tile();
        }

        if input_events.is_key_pressed(KeyCode::Char('q')) {
            self.try_swap_piece();
        }

        //GRAVITY + TILE LOCKING
        if self.drop_timer > self.get_drop_interval() {
            if !self.try_move(0, 1) {
                self.falling_block.lock_timer += dt;
                self.falling_block.lock = true;

                if self.falling_block.lock_timer > 0.25 {
                    self.drop_timer = 0.0;
                    self.board.place_piece(&self.falling_block);
                    self.swapped_piece = false;
                    self.falling_block = FallingBlock::new(self.piece_queue.get_piece_from_queue());
                }
            } else {
                self.drop_timer = 0.0;
            }
        }

        //DRAW GAME FRAME
        display.set_area_char(0, 0, 44, 22, '█');
        display.set_area_char(0, 0, 44, 1, '▄');
        display.set_area_char(0, 22, 44, 1, '▀');
        display.set_area_fg(0, 0, 44, 23, Color::DarkGrey);

        self.board.draw_board(display);
        self.board.draw_shadow_piece(display, &self.falling_block);
        self.board.draw_falling_piece(display, &self.falling_block);

        display.insert_raw_str_fg_bg(
            &format!("{: ^20}", "TETRIS by Ishaia"),
            2,
            1,
            Color::White,
            Color::DarkGrey,
        );

        display.insert_raw_str_fg_bg(
            &format!("{: ^8}", "NEXT"),
            24,
            1,
            Color::White,
            Color::DarkGrey,
        );
        TetrisGame::draw_next_piece(self.piece_queue.peek_next(), display, 24, 2);

        display.insert_raw_str_fg_bg(
            &format!("{: ^8}", "HELD"),
            34,
            1,
            Color::White,
            Color::DarkGrey,
        );
        TetrisGame::draw_next_piece(self.held_piece, display, 34, 2);

        display.insert_raw_str_fg_bg(
            &format!("Level: {}", self.level),
            24,
            10,
            Color::White,
            Color::DarkGrey,
        );
        display.insert_raw_str_fg_bg(
            &format!("Points: {}", self.points),
            24,
            12,
            Color::White,
            Color::DarkGrey,
        );
    }
}
