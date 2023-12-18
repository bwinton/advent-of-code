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

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    pub fn move_pos(
        &self,
        curr: Point2,
        multiplier: i64,
        min: Option<Point2>,
        max: Option<Point2>,
    ) -> Option<Point2> {
        let (try_x, try_y) = match self {
            Direction::North => (curr.0 as i64, curr.1 as i64 - multiplier),
            Direction::East => (curr.0 as i64 + multiplier, curr.1 as i64),
            Direction::South => (curr.0 as i64, curr.1 as i64 + multiplier),
            Direction::West => (curr.0 as i64 - multiplier, curr.1 as i64),
        };

        if (min.is_none() || try_x >= min.unwrap().0 as i64 && try_y >= min.unwrap().1 as i64)
            && (max.is_none() || try_x < max.unwrap().0 as i64 && try_y < max.unwrap().1 as i64)
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
}
