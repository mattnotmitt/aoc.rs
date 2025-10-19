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

    pub fn safe_access<'a, T>(&self, grid: &'a Vec<Vec<T>>) -> Option<&'a T> {
        if self.x < 0 || self.y < 0 {
            return None;
        }
        return grid
            .get(self.x as usize)
            .and_then(|r| r.get(self.y as usize));
    }
}

#[derive(Clone, Copy)]
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
