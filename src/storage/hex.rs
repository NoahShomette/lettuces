use glam::UVec2;

use hexx::HexOrientation;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "bevy_reflect")]
use bevy::prelude::Reflect;

use crate::cell::Cell;

use super::grid::Grid;

/// Storage structure for a hexagon map that is rectangular in nature with rows of the same length.
///
/// Uses Axial Coordinate System
///
/// See [RedBlobGames Hexagon Map Storage](https://www.redblobgames.com/grids/hexagons/#map-storage)
#[derive(Hash, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "bevy_reflect", derive(Reflect))]
pub struct HexRectangleStorage<T> {
    pub grid: Grid<T>,
    pub orientation: HexOrientation,
}

impl<T> HexRectangleStorage<T> {
    /// Construct a new [`HexRectangleStorage`] from the default for the given data
    pub fn new(x_size: usize, y_size: usize, orientation: HexOrientation) -> HexRectangleStorage<T>
    where
        T: Default,
    {
        HexRectangleStorage {
            grid: Grid::new(y_size, x_size),
            orientation,
        }
    }

    pub fn dimensions(&self) -> UVec2 {
        UVec2 {
            x: self.grid.cols() as u32,
            y: self.grid.rows() as u32,
        }
    }

    /// Construct a new [`HexRectangleStorage`] by cloning the given data
    pub fn new_uniform(
        x_size: usize,
        y_size: usize,
        data: T,
        orientation: HexOrientation,
    ) -> HexRectangleStorage<T>
    where
        T: Clone,
    {
        HexRectangleStorage {
            grid: Grid::init(y_size, x_size, data),
            orientation,
        }
    }

    /// Construct a new [`HexRectangleStorage`] from a vec of vecs of data.
    ///
    /// The data is layed out and accessed according to axial coordinates
    pub fn new_from_vec(data: Vec<Vec<T>>, orientation: HexOrientation) -> HexRectangleStorage<T>
    where
        T: Default + Clone + Copy,
    {
        let mut given_tile_count = 0u64;

        for tile_data in data.iter() {
            given_tile_count += tile_data.len() as u64;
        }

        assert_eq!((data[0].len() * data.len()) as u64, given_tile_count);

        let mut grid: Grid<T> = Grid::init(data[0].len(), data.len(), T::default());
        let mut current_x = 0usize;
        let mut current_y = 0usize;
        let row_length = data[0].len();
        grid.fill_with(|| {
            let tile = data[current_y][current_x];
            current_x += 1;
            if current_x == row_length {
                current_x = 0;
                current_y += 1;
            }
            tile
        });
        HexRectangleStorage { grid, orientation }
    }

    pub fn verify_access(&self, cell: Cell) -> Option<[usize; 2]> {
        match self.orientation {
            HexOrientation::Pointy => {
                let col = cell.x + f32::floor(cell.y as f32 / 2.0) as i32;
                if cell.y.is_negative()
                    || col.is_negative()
                    || col as usize > self.grid.cols()
                    || cell.y as usize > self.grid.rows()
                {
                    return None;
                }
                Some([col as usize, cell.y as usize])
            }
            HexOrientation::Flat => {
                let col = cell.y + f32::floor(cell.x as f32 / 2.0) as i32;
                if cell.x.is_negative()
                    || col.is_negative()
                    || col as usize > self.grid.cols()
                    || cell.x as usize > self.grid.rows()
                {
                    return None;
                }
                Some([cell.x as usize, col as usize])
            }
        }
    }

    /// Access data inside the grid. Verifies that the location is a valid cell according to hexagonal coordinate system
    pub fn get(&self, cell: Cell) -> Option<&T> {
        let access = self.verify_access(cell)?;
        self.grid.get(access[1], access[0])
    }

    /// Access data mutably inside the grid. Verifies that the location is a valid cell according to hexagonal coordinate system
    pub fn get_mut(&mut self, cell: Cell) -> Option<&mut T> {
        let access = self.verify_access(cell)?;
        self.grid.get_mut(access[1], access[0])
    }

    /// Sets the data at the given Cell. Verifies that the location is a valid cell according to hexagonal coordinate system
    pub fn set(&mut self, cell: Cell, data: T) {
        let Some(access) = self.verify_access(cell) else {
            return;
        };
        let Some(t) = self.grid.get_mut(access[1], access[0]) else {
            return;
        };
        *t = data;
    }
}

#[cfg(test)]
mod tests {
    use glam::UVec2;
    use hexx::HexOrientation;

    use crate::cell::Cell;

    use super::HexRectangleStorage;

    #[derive(Default, Clone, Copy, PartialEq, Eq, Debug)]
    struct TileData {
        position: Cell,
    }

    #[test]
    fn test_basic_access_flat() {
        let map = HexRectangleStorage::<TileData>::new(7, 7, HexOrientation::Flat);

        let data = map.get(Cell::new(6, -3));
        assert!(data.is_some());
        let data = map.get(Cell::new(6, -2));
        assert!(data.is_some());
        let data = map.get(Cell::new(0, 6));
        assert!(data.is_some());

        let data = map.get(Cell::new(6, -4));
        assert!(data.is_none());
        let data = map.get(Cell::new(1, -1));
        assert!(data.is_none());
        let data = map.get(Cell::new(0, 7));
        assert!(data.is_none());
        let data = map.get(Cell::new(8, 2));
        assert!(data.is_none());
    }
    #[test]
    fn test_basic_access_pointy() {
        let map = HexRectangleStorage::<TileData>::new(7, 7, HexOrientation::Pointy);

        let data = map.get(Cell::new(-3, 6));
        assert!(data.is_some());
        let data = map.get(Cell::new(-2, 6));
        assert!(data.is_some());
        let data = map.get(Cell::new(0, 6));
        assert!(data.is_some());

        let data = map.get(Cell::new(-4, 6));
        assert!(data.is_none());
        let data = map.get(Cell::new(-1, -1));
        assert!(data.is_none());
        let data = map.get(Cell::new(0, 7));
        assert!(data.is_none());
        let data = map.get(Cell::new(8, 2));
        assert!(data.is_none());
    }

    #[test]
    fn test_correct_access_flat() {
        let map = make_map(UVec2::new(15, 15), HexOrientation::Flat);

        let map = HexRectangleStorage::<TileData>::new_from_vec(map, HexOrientation::Flat);

        let data = map.get(Cell::new(6, -3));
        assert_eq!(
            data,
            Some(&TileData {
                position: Cell::new(6, -3)
            })
        );
        let data = map.get(Cell::new(6, -2));
        assert_eq!(
            data,
            Some(&TileData {
                position: Cell::new(6, -2)
            })
        );
        let data = map.get(Cell::new(0, 6));
        assert_eq!(
            data,
            Some(&TileData {
                position: Cell::new(0, 6)
            })
        );

        let data = map.get(Cell::new(5, -2));
        assert_eq!(
            data,
            Some(&TileData {
                position: Cell::new(5, -2)
            })
        );
        let data = map.get(Cell::new(4, -1));
        assert_eq!(
            data,
            Some(&TileData {
                position: Cell::new(4, -1,)
            })
        );
        let data = map.get(Cell::new(6, 0));
        assert_eq!(
            data,
            Some(&TileData {
                position: Cell::new(6, 0)
            })
        );
        let data = map.get(Cell::new(5, 3));
        assert_eq!(
            data,
            Some(&TileData {
                position: Cell::new(5, 3)
            })
        );
    }

    #[test]
    fn test_correct_access_pointy() {
        let map = make_map(UVec2::new(15, 15), HexOrientation::Pointy);

        let map = HexRectangleStorage::<TileData>::new_from_vec(map, HexOrientation::Pointy);

        let data = map.get(Cell::new(-3, 6));
        assert_eq!(
            data,
            Some(&TileData {
                position: Cell::new(-3, 6)
            })
        );
        let data = map.get(Cell::new(-2, 6));
        assert_eq!(
            data,
            Some(&TileData {
                position: Cell::new(-2, 6)
            })
        );
        let data = map.get(Cell::new(0, 6));
        assert_eq!(
            data,
            Some(&TileData {
                position: Cell::new(0, 6)
            })
        );

        let data = map.get(Cell::new(-2, 5));
        assert_eq!(
            data,
            Some(&TileData {
                position: Cell::new(-2, 5)
            })
        );
        let data = map.get(Cell::new(-1, 4));
        assert_eq!(
            data,
            Some(&TileData {
                position: Cell::new(-1, 4)
            })
        );
        let data = map.get(Cell::new(6, 0));
        assert_eq!(
            data,
            Some(&TileData {
                position: Cell::new(6, 0)
            })
        );
        let data = map.get(Cell::new(5, 3));
        assert_eq!(
            data,
            Some(&TileData {
                position: Cell::new(5, 3)
            })
        );
    }

    fn make_map(size: UVec2, orientation: HexOrientation) -> Vec<Vec<TileData>> {
        let mode = match orientation {
            HexOrientation::Pointy => hexx::OffsetHexMode::OddRows,
            HexOrientation::Flat => hexx::OffsetHexMode::OddColumns,
        };

        let mut map = vec![];
        for y in 0..size.y {
            let mut row = vec![];
            for x in 0..size.x {
                row.push(TileData {
                    position: Cell::from(Cell::from_offset_coordinates([x as i32, y as i32], mode)),
                });
            }
            map.push(row);
        }
        map
    }
}

pub fn convert_2d_array_index_to_rectangle_position(position: UVec2) -> Cell {
    let y_offset = f32::floor(position.y as f32 / 2.0) as i32;
    Cell::new(position.x as i32 - y_offset, position.y as i32)
}
