use super::Cell;

pub const fn add_cell_arrays(lhs: [Cell; 4], rhs: [Cell; 4]) -> [Cell; 8] {
    [
        lhs[0], lhs[1], lhs[2], lhs[3], rhs[0], rhs[1], rhs[2], rhs[3],
    ]
}
