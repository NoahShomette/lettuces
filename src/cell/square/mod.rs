use super::{helpers::add_cell_arrays, Cell};

impl Cell {
    pub const SQUARE_PRIMARY_OFFSETS: [Self; 4] = [
        Self::new(1, 0),
        Self::new(0, 1),
        Self::new(-1, 0),
        Self::new(0, -1),
    ];

    pub const SQUARE_DIAGONAL_OFFSETS: [Self; 4] = [
        Self::new(1, 1),
        Self::new(-1, 1),
        Self::new(-1, -1),
        Self::new(1, -1),
    ];

    pub const SQUARE_OFFSETS: [Self; 8] =
        add_cell_arrays(Self::SQUARE_DIAGONAL_OFFSETS, Self::SQUARE_PRIMARY_OFFSETS);
}
