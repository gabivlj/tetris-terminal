extern crate termion;

pub mod pieces;
pub mod tetris;
pub mod utils;

use tetris::game::Tetris;

fn main() {
    let mut tetris = Tetris::new();
    tetris.start();
}
