use std::fmt::Display;

pub type Point2 = (i64, i64);
pub type Point3 = (i64, i64, i64);

/// Calculate the indices around x and y to the specified depth, handling (0,0), and return them in a vec.
/// Note: includes (x,y)
pub fn ring(point: Point2, depth: usize, min: Point2, max: Point2) -> Vec<Point2> {
    let (x, y) = point;
    let mut rv = vec![];
    for i in (x - depth as i64).max(min.0)..=(x + depth as i64).min(max.0) {
        for j in (y - depth as i64).max(min.1)..=(y + depth as i64).min(max.1) {
            rv.push((i, j));
        }
    }
    rv
}

pub fn point_to_index(point: Point2, width: i64) -> usize {
    (point.1 * width + point.0) as usize
}

pub fn in_bounds(test: Point2, origin: Point2, bounds: Point2) -> bool {
    origin.0 <= test.0 && test.0 < bounds.0 && origin.1 <= test.1 && test.1 < bounds.1
}

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn get_point(&self) -> Point2 {
        match self {
            Direction::North => (0, -1),
            Direction::East => (1, 0),
            Direction::South => (0, 1),
            Direction::West => (-1, 0),
        }
    }
    pub fn move_pos(
        &self,
        curr: Point2,
        multiplier: i64,
        min: Option<Point2>,
        max: Option<Point2>,
    ) -> Option<Point2> {
        let (try_x, try_y) = match self {
            Direction::North => (curr.0, curr.1 - multiplier),
            Direction::East => (curr.0 + multiplier, curr.1),
            Direction::South => (curr.0, curr.1 + multiplier),
            Direction::West => (curr.0 - multiplier, curr.1),
        };

        if (min.is_none() || try_x >= min.unwrap().0 && try_y >= min.unwrap().1)
            && (max.is_none() || try_x < max.unwrap().0 && try_y < max.unwrap().1)
        {
            Some((try_x, try_y))
        } else {
            None
        }
    }

    pub fn all() -> Vec<Direction> {
        vec![
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ]
    }

    pub fn opposite(&self, other: &Direction) -> bool {
        match self {
            Direction::North => other == &Direction::South,
            Direction::East => other == &Direction::West,
            Direction::South => other == &Direction::North,
            Direction::West => other == &Direction::East,
        }
    }

    pub fn turn_right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    pub fn turn_left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::North => f.write_str("North"),
            Direction::East => f.write_str("East"),
            Direction::South => f.write_str("South"),
            Direction::West => f.write_str("West"),
        }
    }
}
