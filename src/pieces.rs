pub const EMPTY_CELL: char = '▢';
pub const FILLED_CELL: char = '▨';
pub const E: char = '▢';
pub const F: char = '▨';

pub type Position = (isize, isize);
pub type CellStruct = [[char; 4]; 4];
pub type Cells = [CellStruct; 4];
pub type Rotation = usize;
pub type Piece = (Position, Rotation, Cells, usize, usize);

const BLOCK_ROTATIONS: Cells = [
    [[F, F, E, E], [F, F, E, E], [E, E, E, E], [E, E, E, E]],
    [[F, F, E, E], [F, F, E, E], [E, E, E, E], [E, E, E, E]],
    [[F, F, E, E], [F, F, E, E], [E, E, E, E], [E, E, E, E]],
    [[F, F, E, E], [F, F, E, E], [E, E, E, E], [E, E, E, E]],
];

const STICK_ROTATIONS: Cells = [
    // Vertical
    [[E, F, E, E], [E, F, E, E], [E, F, E, E], [E, F, E, E]],
    // Horizontal
    [[E, E, E, E], [E, E, E, E], [F, F, F, F], [E, E, E, E]],
    // Vertical (REVERSED)
    [[E, F, E, E], [E, F, E, E], [E, F, E, E], [E, F, E, E]],
    // Horizontal (REVERSED)
    [[E, E, E, E], [E, E, E, E], [F, F, F, F], [E, E, E, E]],
];

pub const BLOCK_PIECE: Piece = ((0, 0), 0, BLOCK_ROTATIONS, 2, 2);
pub const STICK_PIECE: Piece = ((0, 0), 0, STICK_ROTATIONS, 1, 4);

pub enum Move {
    LEFT,
    RIGHT,
    DOWN,
    ROTATION,
}
