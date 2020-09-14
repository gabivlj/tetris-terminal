pub const EMPTY_CELL: char = '▢';
pub const FILLED_CELL: char = '▨';
pub const E: char = '▢';
pub const F: char = '▨';

pub type Position = (isize, isize);
pub type CellStruct = [[char; 4]; 4];
pub type Cells = [CellStruct; 4];
pub type Rotation = isize;
pub type Piece = (Position, Rotation, Cells);

const BLOCK_ROTATIONS: Cells = [
    [[F, F, E, E], [F, F, E, E], [E, E, E, E], [E, E, E, E]],
    [[F, F, E, E], [F, F, E, E], [E, E, E, E], [E, E, E, E]],
    [[F, F, E, E], [F, F, E, E], [E, E, E, E], [E, E, E, E]],
    [[F, F, E, E], [F, F, E, E], [E, E, E, E], [E, E, E, E]],
];

const STICK_ROTATIONS: Cells = [
    // Vertical
    [[E, E, F, E], [E, E, F, E], [E, E, F, E], [E, E, F, E]],
    // Horizontal
    [[E, E, E, E], [E, E, E, E], [F, F, F, F], [E, E, E, E]],
    // Vertical (REVERSED)
    [[E, F, E, E], [E, F, E, E], [E, F, E, E], [E, F, E, E]],
    // Horizontal (REVERSED)
    [[E, E, E, E], [E, E, E, E], [F, F, F, F], [E, E, E, E]],
];

const REVERSE_L_ROTATIONS: Cells = [
    // Vertical
    [[E, E, E, E], [F, F, F, E], [E, E, F, E], [E, E, E, E]],
    // Horizontal
    [[E, E, E, E], [E, F, E, E], [E, F, E, E], [F, F, E, E]],
    // Vertical (REVERSED)
    [[E, E, E, E], [E, F, E, E], [E, F, F, F], [E, E, E, E]],
    // Horizontal (REVERSED)
    [[E, E, E, E], [E, F, F, E], [E, F, E, E], [E, F, E, E]],
];

pub const BLOCK_PIECE: Piece = ((0, 0), 0, BLOCK_ROTATIONS);
pub const STICK_PIECE: Piece = ((0, 0), 0, STICK_ROTATIONS);
pub const REVERSE_L_PIECE: Piece = ((0, 0), 0, REVERSE_L_ROTATIONS);
pub const EVERY_PIECE: [Piece; 3] = [BLOCK_PIECE, STICK_PIECE, REVERSE_L_PIECE];

use rand::Rng;

pub fn get_piece() -> Piece {
    let mut rng = rand::thread_rng();
    EVERY_PIECE[rng.gen_range(0, EVERY_PIECE.len())]
}

pub enum Move {
    LEFT,
    RIGHT,
    DOWN,
    ROTATION,
}
