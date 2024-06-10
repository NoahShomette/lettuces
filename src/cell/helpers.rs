use hexx::Hex;

use super::Cell;

pub const fn add_cell_arrays(lhs: [Cell; 4], rhs: [Cell; 4]) -> [Cell; 8] {
    [
        lhs[0], lhs[1], lhs[2], lhs[3], rhs[0], rhs[1], rhs[2], rhs[3],
    ]
}

pub const fn hex_into_cell(value: Hex) -> Cell {
    Cell(value)
}

pub const fn convert_hex_six_array(value: [Hex; 6]) -> [Cell; 6] {
    [
        hex_into_cell(value[0]),
        hex_into_cell(value[1]),
        hex_into_cell(value[2]),
        hex_into_cell(value[3]),
        hex_into_cell(value[4]),
        hex_into_cell(value[5]),
    ]
}
