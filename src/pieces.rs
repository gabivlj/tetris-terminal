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

pub const BLOCK_PIECE: Piece = ((0, 0), 0, BLOCK_ROTATIONS, 2, 2);

pub enum Move {
    LEFT,
    RIGHT,
    DOWN,
}
