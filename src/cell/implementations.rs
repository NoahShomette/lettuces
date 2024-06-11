use std::ops::Add;

use glam::IVec2;

use super::Cell;

impl From<IVec2> for Cell {
    fn from(value: IVec2) -> Self {
        Cell::new(value.x, value.y)
    }
}

impl Add<Cell> for Cell {
    type Output = Self;

    fn add(self, rhs: Cell) -> Self::Output {
        Cell::new(self.x.add(rhs.x), self.y.add(rhs.y))
    }
}
