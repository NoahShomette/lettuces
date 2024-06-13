pub(crate) mod helpers;
#[cfg(feature = "hex")]
pub mod hex;
pub mod implementations;
#[cfg(feature = "square")]
pub mod square;

#[cfg(feature = "bevy")]
use bevy::prelude::Component;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "bevy_reflect")]
use bevy::prelude::Reflect;

/// A position in a grid. This can represent a hexagonal or square position. Functions for both are included on it.
///
/// If working with hexagonal maps its recommended to convert your Cell into [`Hex`](hex::Hex) when needed
#[derive(Default, Eq, Hash, PartialEq, Copy, Clone, Debug)]
#[cfg_attr(feature = "bevy", derive(Component))]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(Reflect))]
pub struct Cell {
    pub x: i32,
    pub y: i32,
}

impl Cell {
    /// (0, 0)
    pub const ORIGIN: Self = Self::ZERO;
    /// (0, 0)
    pub const ZERO: Self = Self::new(0, 0);

    /// (1, 1)
    pub const ONE: Self = Self::new(1, 1);
    /// (-1, -1)
    pub const NEG_ONE: Self = Self::new(-1, -1);

    /// +X (Q) (1, 0)
    pub const X: Self = Self::new(1, 0);
    /// -X (-Q) (-1, 0)
    pub const NEG_X: Self = Self::new(-1, 0);
    /// +Y (R) (0, 1)
    pub const Y: Self = Self::new(0, 1);
    /// -Y (-R) (0, -1)
    pub const NEG_Y: Self = Self::new(0, -1);

    pub const fn new(x: i32, y: i32) -> Cell {
        Cell { x, y }
    }

    pub const fn splat(v: i32) -> Self {
        Self::new(v, v)
    }

    pub const fn new_unsigned(x: u32, y: u32) -> Cell {
        Cell::new(x as i32, y as i32)
    }

    pub const fn from_array([x, y]: [i32; 2]) -> Self {
        Self::new(x, y)
    }

    pub const fn from_slice(slice: &[i32]) -> Self {
        Self::new(slice[0], slice[1])
    }

    pub fn write_to_slice(self, slice: &mut [i32]) {
        slice[0] = self.x;
        slice[1] = self.y;
    }

    /// `x` coordinate
    pub const fn x(self) -> i32 {
        self.x
    }

    /// `y` coordinate
    pub const fn y(self) -> i32 {
        self.y
    }

    /// The raw x and y pos of this cell
    pub const fn pos(self) -> [i32; 2] {
        [self.x, self.y]
    }
}
