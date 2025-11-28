use std::ops;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Point2D {
    pub x: isize,
    pub y: isize,
}

impl ops::Add<Point2D> for Point2D {
    type Output = Point2D;
    fn add(self, rhs: Point2D) -> Point2D {
        Point2D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub<Point2D> for Point2D {
    type Output = Point2D;
    fn sub(self, rhs: Point2D) -> Point2D {
        Point2D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::Add<Direction2D> for Point2D {
    type Output = Point2D;
    fn add(self, rhs: Direction2D) -> Point2D {
        let dir_point = rhs.resolve();
        Point2D {
            x: self.x + dir_point.x,
            y: self.y + dir_point.y,
        }
    }
}

impl Point2D {
    pub fn new(x: isize, y: isize) -> Point2D {
        Point2D { x, y }
    }

    pub fn move_wrap(&self, size: &(isize, isize), step: &Point2D) -> Point2D {
        let mut new_pos = *self + *step;

        new_pos.x = ((new_pos.x % size.0) + size.0) % size.0;
        new_pos.y = ((new_pos.y % size.1) + size.1) % size.1;
        assert!(
            new_pos.x >= 0,
            "new_pos.x ({:?}) must be non-negative",
            new_pos
        );
        assert!(
            new_pos.y >= 0,
            "new_pos.y ({:?}) must be non-negative",
            new_pos
        );
        assert!(
            new_pos.x < size.0,
            "new_pos.x ({:?}) must be less than {}",
            new_pos,
            size.0
        );
        assert!(
            new_pos.y < size.1,
            "new_pos.y ({:?}) must be less than {}",
            new_pos,
            size.1
        );
        return new_pos;
    }

    pub fn safe_access<'a, T>(&self, grid: &'a Vec<Vec<T>>) -> Option<&'a T> {
        if self.x < 0 || self.y < 0 {
            return None;
        }
        return grid
            .get(self.x as usize)
            .and_then(|r| r.get(self.y as usize));
    }

    pub fn update<T>(&self, grid: &mut Vec<Vec<T>>, val: T) {
        if self.x < 0 || self.y < 0 {
            return;
        }
        grid[self.x as usize][self.y as usize] = val;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Direction2D {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Direction2D {
    pub fn iter() -> std::slice::Iter<'static, Direction2D> {
        static DIRECTIONS: [Direction2D; 8] = [
            Direction2D::N,
            Direction2D::NE,
            Direction2D::E,
            Direction2D::SE,
            Direction2D::S,
            Direction2D::SW,
            Direction2D::W,
            Direction2D::NW,
        ];
        return DIRECTIONS.iter();
    }

    pub fn iter_cardinal() -> std::slice::Iter<'static, Direction2D> {
        static DIRECTIONS: [Direction2D; 4] = [
            Direction2D::N,
            Direction2D::E,
            Direction2D::S,
            Direction2D::W,
        ];
        return DIRECTIONS.iter();
    }

    fn resolve(&self) -> Point2D {
        return match self {
            Direction2D::N => Point2D::new(-1, 0),
            Direction2D::NE => Point2D::new(-1, 1),
            Direction2D::E => Point2D::new(0, 1),
            Direction2D::SE => Point2D::new(1, 1),
            Direction2D::S => Point2D::new(1, 0),
            Direction2D::SW => Point2D::new(1, -1),
            Direction2D::W => Point2D::new(0, -1),
            Direction2D::NW => Point2D::new(-1, -1),
        };
    }

    pub fn flip(&self) -> Direction2D {
        return match self {
            Direction2D::N => Direction2D::S,
            Direction2D::NE => Direction2D::SW,
            Direction2D::E => Direction2D::W,
            Direction2D::SE => Direction2D::NW,
            Direction2D::S => Direction2D::N,
            Direction2D::SW => Direction2D::NE,
            Direction2D::W => Direction2D::E,
            Direction2D::NW => Direction2D::SE,
        };
    }

    pub fn rotate_90(&self) -> Direction2D {
        return match self {
            Direction2D::N => Direction2D::E,
            Direction2D::NE => Direction2D::SE,
            Direction2D::E => Direction2D::S,
            Direction2D::SE => Direction2D::SW,
            Direction2D::S => Direction2D::W,
            Direction2D::SW => Direction2D::NW,
            Direction2D::W => Direction2D::N,
            Direction2D::NW => Direction2D::NE,
        };
    }
}

pub fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}
