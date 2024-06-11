use glam::UVec2;

use crate::cell::Cell;

use super::grid::Grid;

/// Storage structure for a hexagon map that is rectangular in nature with rows of the same length.
///
/// See [RedBlobGames Hexagon Map Storage](https://www.redblobgames.com/grids/hexagons/#map-storage)
pub struct HexRectangleStorage<T> {
    pub grid: Grid<T>,
    pub dimensions: UVec2,
}

impl<T> HexRectangleStorage<T> {
    /// Construct a new [`HexRectangleStorage`] from the default for the given data
    pub fn new(x_size: usize, y_size: usize) -> HexRectangleStorage<T>
    where
        T: Default,
    {
        HexRectangleStorage {
            grid: Grid::new(y_size, x_size),
            dimensions: UVec2 {
                x: x_size as u32,
                y: y_size as u32,
            },
        }
    }

    /// Construct a new [`HexRectangleStorage`] by cloning the given data
    pub fn new_uniform(x_size: usize, y_size: usize, data: T) -> HexRectangleStorage<T>
    where
        T: Clone,
    {
        HexRectangleStorage {
            grid: Grid::init(y_size, x_size, data),
            dimensions: UVec2 {
                x: x_size as u32,
                y: y_size as u32,
            },
        }
    }

    /// Construct a new [`HexRectangleStorage`] from a vec of vecs of data
    pub fn new_from_vec(data: Vec<Vec<T>>) -> HexRectangleStorage<T>
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
        HexRectangleStorage {
            grid,
            dimensions: UVec2 {
                x: data[0].len() as u32,
                y: data.len() as u32,
            },
        }
    }

    pub fn verify_access(cell: Cell) -> Option<[usize; 2]> {
        let col = cell.x as i32 + f32::floor(cell.y as f32 / 2.0) as i32;
        if cell.y.is_negative() || col.is_negative() {
            return None;
        }
        Some([col as usize, cell.y as usize])
    }

    /// Access data inside the grid. Verifies that the location is a valid cell according to hexagonal coordinate system
    pub fn get(&self, cell: Cell) -> Option<&T> {
        let access = Self::verify_access(cell)?;
        self.grid.get(access[1], access[0])
    }

    /// Access data mutably inside the grid. Verifies that the location is a valid cell according to hexagonal coordinate system
    pub fn get_mut(&mut self, cell: Cell) -> Option<&mut T> {
        let access = Self::verify_access(cell)?;
        self.grid.get_mut(access[1], access[0])
    }

    /// Sets the data at the given Cell. Verifies that the location is a valid cell according to hexagonal coordinate system
    pub fn set(&mut self, cell: Cell, data: T) {
        let Some(access) = Self::verify_access(cell) else {
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
    use crate::cell::Cell;

    use super::HexRectangleStorage;

    #[derive(Default)]
    struct MapData;

    #[test]
    fn test_access() {
        let map = HexRectangleStorage::<MapData>::new(7, 7);

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
        let data = map.get(Cell::new(6, 2));
        assert!(data.is_none());
    }
}
