use hexx::{Hex, OffsetHexMode};

use super::Cell;

impl From<Hex> for Cell {
    fn from(value: Hex) -> Self {
        Cell::new(value.x, value.y)
    }
}

impl Cell {
    /// Converts offset coordinates into axial coordinates with the given mode.
    pub fn from_offset_coordinates(coords: [i32; 2], mode: OffsetHexMode) -> Cell {
        Hex::from_offset_coordinates(coords, mode).into()
    }
}
