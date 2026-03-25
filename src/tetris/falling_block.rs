use crate::tetris::PieceType;

pub struct FallingBlock {
    pub piece_type: PieceType,
    pub rotation: usize,
    pub x: usize,
    pub y: usize,
    pub lock: bool,
    pub lock_timer: f32,
    pub lock_tries: i8,
}

impl FallingBlock {
    pub fn new(piece_type: PieceType) -> Self {
        Self {
            piece_type,
            rotation: 0,
            x: 3,
            y: 3,
            lock: false,
            lock_timer: 0.0,
            lock_tries: 15,
        }
    }

    pub fn rotation_mode(&self, rotation: usize) -> usize {
        match self.rotation {
            0 => match rotation {
                1 => 0,
                3 => 7,
                _ => 0,
            },
            1 => match rotation {
                0 => 1,
                2 => 2,
                _ => 0,
            },
            2 => match rotation {
                1 => 3,
                3 => 4,
                _ => 0,
            },
            3 => match rotation {
                0 => 6,
                2 => 5,
                _ => 0,
            },
            _ => 0,
        }
    }
}
