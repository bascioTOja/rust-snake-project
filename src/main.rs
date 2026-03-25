#![allow(unused)]
mod engine;
mod tetris;

use std::panic;

use crate::engine::GameEngine;
use crate::tetris::TetrisGame;

pub fn install_panic_hook() {
    panic::set_hook(Box::new(|info| {
        // leave alternate screen
        print!("\x1b[?1049l");

        eprintln!("CRASH: {info}");

        // optional: wait so it doesn't instantly disappear
        use std::io::{self, Read};
        let _ = io::stdin().read(&mut [0u8]).ok();
    }));
}

fn main() -> std::io::Result<()> {
    install_panic_hook();
    let mut engine = GameEngine::new();
    engine.debug.toggle_debug(true);
    engine.add_object(Box::new(TetrisGame::new()));
    // engine.add_object(Box::new(RainbowGoBrr::new()));
    engine.run()?;
    Ok(())
}
