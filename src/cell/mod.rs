pub(crate) mod helpers;
pub mod implementations;

use helpers::{add_cell_arrays, convert_hex_six_array};
use hexx::Hex;

/// A position in a grid. This can represent a hexagonal or square position. Functions for both are included on it.
#[derive(Default, Eq, Hash, PartialEq, Copy, Clone, Debug)]
pub struct Cell(Hex);

impl Cell {
    pub const fn new(x: i32, y: i32) -> Cell {
        Cell(Hex::new(x, y))
    }

    pub const fn new_unsigned(x: u32, y: u32) -> Cell {
        Cell(Hex::new(x as i32, y as i32))
    }

    pub const HEX_NEIGHBOR_OFFSETS: [Self; 6] = convert_hex_six_array(Hex::NEIGHBORS_COORDS);

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

    pub const fn inner(self) -> Hex {
        self.0
    }

    /// The raw x and y pos of this cell
    pub const fn pos(self) -> [i32; 2] {
        [self.0.x, self.0.y]
    }

    /// The calculated z coordinate. Used only for hexagonal grids.
    ///
    /// See [`Hex::z()`] for more info.
    pub const fn z(self) -> i32 {
        self.0.z()
    }
}
