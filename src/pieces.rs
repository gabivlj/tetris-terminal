pub const EMPTY_CELL: char = '▢';
pub const FILLED_CELL: char = '▨';
pub const E: char = '▢';
pub const F: char = '▨';

pub type Position = (usize, usize);
pub type CellStruct = [[char; 4]; 4];
pub type Cells = [CellStruct; 4];
pub type Rotation = usize;
pub type Piece = (Position, Rotation, Cells);

const BLOCK_ROTATIONS: Cells = [
    [[F, F, E, E], [F, F, E, E], [E, E, E, E], [E, E, E, E]],
    [[F, F, E, E], [F, F, E, E], [E, E, E, E], [E, E, E, E]],
    [[F, F, E, E], [F, F, E, E], [E, E, E, E], [E, E, E, E]],
    [[F, F, E, E], [F, F, E, E], [E, E, E, E], [E, E, E, E]],
];

pub const BLOCK_PIECE: Piece = ((0, 0), 0, BLOCK_ROTATIONS);
