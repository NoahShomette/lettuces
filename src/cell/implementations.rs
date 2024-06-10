use hexx::Hex;

use super::Cell;


impl From<Hex> for Cell {
    fn from(value: Hex) -> Self {
        Cell(value)
    }
}
