use crate::tetris::PieceType;

use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;

use std::collections::VecDeque;

pub struct PieceQueue {
    queue: VecDeque<PieceType>,
    rng: ThreadRng,
}

impl PieceQueue {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            rng: rand::rng(),
        }
    }

    fn generate_queue(&mut self) {
        let mut pieces: [PieceType; 7] = [
            PieceType::I,
            PieceType::O,
            PieceType::T,
            PieceType::J,
            PieceType::L,
            PieceType::S,
            PieceType::Z,
        ];

        pieces.shuffle(&mut self.rng);

        for piece in pieces.iter() {
            self.queue.push_back(*piece);
        }
    }

    pub fn clear(&mut self) {
        self.queue.clear();
    }

    pub fn get_piece_from_queue(&mut self) -> PieceType {
        if self.queue.len() < 2 {
            self.generate_queue();
        }

        self.queue.pop_front().unwrap()
    }

    pub fn peek_next(&mut self) -> PieceType {
        self.queue[0].clone()
    }
}
