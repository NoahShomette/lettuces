use std::{
    fmt::{Display, Formatter},
    ops::Add,
};

use glam::IVec2;

use super::Cell;

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&*format!("x:{}, y:{}", self.x, self.y))
    }
}

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

impl Into<(usize, usize)> for Cell {
    fn into(self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
    }
}
