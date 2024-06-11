use hexx::Hex;

use super::Cell;

impl From<Hex> for Cell {
    fn from(value: Hex) -> Self {
        Cell::new(value.x, value.y)
    }
}

impl Cell {}
