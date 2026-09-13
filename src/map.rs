#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Wall,
}

pub const WIDTH: usize = 24;
pub const HEIGHT: usize = 24;

#[derive(Debug)]
pub struct Map {
    tiles: [[Tile; WIDTH]; HEIGHT],
}

impl Map {
    const fn from_ascii(rows: [&str; HEIGHT]) -> Self {
        let mut tiles = [[Tile::Empty; WIDTH]; HEIGHT];
        let mut y = 0;
        while y < HEIGHT {
            let bytes = rows[y].as_bytes();
            assert!(bytes.len() == WIDTH, "map row has the wrong width");
            let mut x = 0;
            while x < WIDTH {
                tiles[y][x] = match bytes[x] {
                    b'#' | b'1' => Tile::Wall,
                    _ => Tile::Empty,
                };
                x += 1;
            }
            y += 1;
        }
        Self { tiles }
    }

    pub fn tile(&self, x: i32, y: i32) -> Tile {
        let Ok(x) = usize::try_from(x) else {
            return Tile::Wall;
        };
        let Ok(y) = usize::try_from(y) else {
            return Tile::Wall;
        };
        self.tiles
            .get(y)
            .and_then(|row| row.get(x))
            .copied()
            .unwrap_or(Tile::Wall)
    }

    pub fn blocked(&self, x: f64, y: f64) -> bool {
        self.tile(x.floor() as i32, y.floor() as i32) == Tile::Wall
    }

    /// True when a circle of radius `r` around `(x, y)` stays in empty tiles.
    pub fn walkable(&self, x: f64, y: f64, r: f64) -> bool {
        !self.blocked(x - r, y - r)
            && !self.blocked(x + r, y - r)
            && !self.blocked(x - r, y + r)
            && !self.blocked(x + r, y + r)
    }
}

pub const WORLD: Map = Map::from_ascii([
    "########################",
    "#...........##.........#",
    "#...........##.........#",
    "#..######...##...####..#",
    "#..#................#..#",
    "#..#................#..#",
    "#......................#",
    "######...########...####",
    "#......................#",
    "#..#................#..#",
    "#..#......##........#..#",
    "#.........##...........#",
    "#.........##...........#",
    "#..#......##........#..#",
    "#..#................#..#",
    "#......................#",
    "######...########...####",
    "#......................#",
    "#..#................#..#",
    "#..#................#..#",
    "#..######...##...####..#",
    "#...........##.........#",
    "#...........##.........#",
    "########################",
]);

#[cfg(test)]
mod tests {
    use super::{HEIGHT, Tile, WIDTH, WORLD};

    #[test]
    fn world_is_enclosed() {
        for x in 0..WIDTH as i32 {
            assert_eq!(WORLD.tile(x, 0), Tile::Wall);
            assert_eq!(WORLD.tile(x, HEIGHT as i32 - 1), Tile::Wall);
        }
        for y in 0..HEIGHT as i32 {
            assert_eq!(WORLD.tile(0, y), Tile::Wall);
            assert_eq!(WORLD.tile(WIDTH as i32 - 1, y), Tile::Wall);
        }
    }

    #[test]
    fn spawn_tile_is_empty() {
        assert!(!WORLD.blocked(2.5, 2.5));
    }

    #[test]
    fn out_of_bounds_is_wall() {
        assert_eq!(WORLD.tile(-1, 5), Tile::Wall);
        assert_eq!(WORLD.tile(5, -1), Tile::Wall);
        assert_eq!(WORLD.tile(WIDTH as i32, 5), Tile::Wall);
        assert_eq!(WORLD.tile(5, HEIGHT as i32), Tile::Wall);
    }
}
