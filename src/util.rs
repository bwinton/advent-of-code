pub type Point2 = (usize, usize);
pub type Point3 = (usize, usize, usize);

/// Calculate the indices around x and y to the specified depth, handling (0,0), and return them in a vec.
/// Note: includes (x,y)
pub fn ring(point: Point2, depth: usize) -> Vec<Point2> {
    let (x, y) = point;
    let mut rv = vec![];
    for i in x.saturating_sub(depth)..=x + depth {
        for j in y.saturating_sub(depth)..=y + depth {
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
    pub fn move_pos(&self, curr: Point2, max: Point2) -> Option<Point2> {
        let (try_x, try_y) = match self {
            Direction::North => (curr.0 as i32, curr.1 as i32 - 1),
            Direction::East => (curr.0 as i32 + 1, curr.1 as i32),
            Direction::South => (curr.0 as i32, curr.1 as i32 + 1),
            Direction::West => (curr.0 as i32 - 1, curr.1 as i32),
        };
        if try_x >= 0 && try_y >= 0 && try_x < max.0 as i32 && try_y < max.1 as i32 {
            Some((try_x as usize, try_y as usize))
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
