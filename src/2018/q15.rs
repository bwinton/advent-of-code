//-----------------------------------------------------
// Setup.

use std::{
    cmp::Ordering,
    collections::BTreeSet,
    fmt::{Display, Formatter, Result as FmtResult},
    hash::{Hash, Hasher},
    str::FromStr,
};

static INPUT: &str = include_str!("data/q15.data");

#[derive(Clone, Debug)]
enum Direction {
    North,
    West,
    East,
    South,
}

impl Direction {
    fn get_cell(&self, x: usize, y: usize) -> (usize, usize) {
        match self {
            Direction::North => (x, y - 1),
            Direction::West => (x - 1, y),
            Direction::East => (x + 1, y),
            Direction::South => (x, y + 1),
        }
    }
}

static DIRECTIONS: &[Direction; 4] = &[
    Direction::North,
    Direction::West,
    Direction::East,
    Direction::South,
];

#[derive(Clone, Debug, PartialEq, Eq)]
enum Kind {
    Empty,
    Wall,
    Goblin { hp: i32 },
    Elf { hp: i32 },
}

impl Display for Kind {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Kind::Empty => write!(f, " "),
            Kind::Wall => write!(f, "#"),
            Kind::Goblin { .. } => write!(f, "G"),
            Kind::Elf { .. } => write!(f, "E"),
        }
    }
}

impl Kind {
    fn from(c: char) -> Result<Kind, ()> {
        match c {
            '.' => Ok(Kind::Empty),
            '#' => Ok(Kind::Wall),
            'G' => Ok(Kind::Goblin { hp: 200 }),
            'E' => Ok(Kind::Elf { hp: 200 }),
            _ => Err(()),
        }
    }
}

#[derive(Clone)]
struct Board {
    cells: Vec<Vec<Kind>>,
    elf_count: usize,
    goblin_count: usize,
    elf_power: i32,
}

impl FromStr for Board {
    type Err = ();

    fn from_str(s: &str) -> Result<Board, ()> {
        let mut cells = vec![];
        let mut elf_count = 0;
        let mut goblin_count = 0;
        for line in s.lines() {
            let mut row = vec![];
            for c in line.chars() {
                if let Ok(kind) = Kind::from(c) {
                    match kind {
                        Kind::Elf { .. } => elf_count += 1,
                        Kind::Goblin { .. } => goblin_count += 1,
                        _ => {}
                    }
                    row.push(kind);
                } else {
                    return Err(());
                }
            }
            cells.push(row);
        }
        Ok(Board {
            cells,
            elf_count,
            goblin_count,
            elf_power: 3,
        })
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        for line in &self.cells {
            let mut units = vec![];
            for kind in line {
                write!(f, "{}", kind)?;
                match kind {
                    Kind::Goblin { hp } => units.push(('G', hp)),
                    Kind::Elf { hp } => units.push(('E', hp)),
                    _ => {}
                }
            }
            if !units.is_empty() {
                write!(f, " ")?;
            }
            for unit in units {
                write!(f, "  {}({})", unit.0, unit.1)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
struct Cell {
    x: usize,
    y: usize,
    length: usize,
    from: Direction,
}

impl Ord for Cell {
    fn cmp(&self, other: &Cell) -> Ordering {
        let mut rv = self.length.cmp(&other.length);
        if rv == Ordering::Equal {
            rv = self.y.cmp(&other.y);
        }
        if rv == Ordering::Equal {
            rv = self.x.cmp(&other.x);
        }
        rv
    }
}

impl PartialOrd for Cell {
    fn partial_cmp(&self, other: &Cell) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Cell) -> bool {
        self.length == other.length && self.x == other.x && self.y == other.y
    }
}

impl Eq for Cell {}

impl Hash for Cell {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.length.hash(state);
        self.x.hash(state);
        self.y.hash(state);
    }
}

pub fn take_min<V: Ord + Clone>(map: &mut BTreeSet<V>) -> Option<V> {
    let opt_min_value: Option<V> = map.iter().next().cloned();
    if let Some(value) = &opt_min_value {
        map.remove(value);
    }
    opt_min_value
}

impl Board {
    fn enemy_adjacent(&self, x: usize, y: usize, kind: &Kind) -> Option<(i32, usize, usize)> {
        let mut rv = vec![];
        for direction in DIRECTIONS {
            let (x, y) = direction.get_cell(x, y);
            if let Kind::Goblin { .. } = kind {
                if let Kind::Elf { hp } = self.cells[y][x] {
                    rv.push((hp, y, x));
                }
            }
            if let Kind::Elf { .. } = kind {
                if let Kind::Goblin { hp } = self.cells[y][x] {
                    rv.push((hp, y, x));
                }
            }
        }
        rv.sort_unstable();
        rv.first().map(|x| (x.0, x.2, x.1))
    }

    fn find_closest(&self, x: usize, y: usize, kind: &Kind) -> Option<Direction> {
        let mut choices = BTreeSet::new();
        let mut seen = BTreeSet::new();
        let mut upcoming = BTreeSet::new();

        if self.enemy_adjacent(x, y, kind).is_some() {
            return None;
        }
        for direction in DIRECTIONS {
            let (x, y) = direction.get_cell(x, y);
            if self.cells[y][x] == Kind::Empty {
                if let Some(_enemy) = self.enemy_adjacent(x, y, kind) {
                    return Some(direction.clone());
                }
                choices.insert(Cell {
                    x,
                    y,
                    length: 1,
                    from: direction.clone(),
                });
            }
        }

        while let Some(curr) = take_min(&mut choices) {
            for direction in DIRECTIONS {
                let (x, y) = direction.get_cell(curr.x, curr.y);
                if self.cells[y][x] != Kind::Empty {
                    // We can't move there, so bail out.
                    continue;
                }
                if let Some(_enemy) = self.enemy_adjacent(x, y, kind) {
                    return Some(curr.from);
                }
                let next = Cell {
                    x,
                    y,
                    length: curr.length + 1,
                    from: curr.from.clone(),
                };
                if self.cells[y][x] == Kind::Empty
                    && !seen.contains(&(next.x, next.y))
                    && !upcoming.contains(&(next.x, next.y))
                {
                    upcoming.insert((next.x, next.y));
                    choices.insert(next);
                }
            }
            upcoming.remove(&(curr.x, curr.y));
            seen.insert((curr.x, curr.y));
        }
        None
    }

    fn move_unit(&mut self, x: usize, y: usize, direction: Direction) -> (usize, usize) {
        let unit = self.cells[y][x].clone();
        self.cells[y][x] = Kind::Empty;
        match direction {
            Direction::North => {
                self.cells[y - 1][x] = unit;
                (x, y - 1)
            }
            Direction::West => {
                self.cells[y][x - 1] = unit;
                (x - 1, y)
            }
            Direction::East => {
                self.cells[y][x + 1] = unit;
                (x + 1, y)
            }
            Direction::South => {
                self.cells[y + 1][x] = unit;
                (x, y + 1)
            }
        }
    }

    fn step(&mut self) -> bool {
        let mut cells = self.cells.clone();
        let mut full_turn = true;
        for y in 0..self.cells.len() {
            for x in 0..self.cells[y].len() {
                let cell = cells[y][x].clone();
                match cell {
                    Kind::Goblin { .. } => {
                        if self.elf_count == 0 {
                            full_turn = false;
                        }
                        let mut x = x;
                        let mut y = y;
                        let direction = self.find_closest(x, y, &cell);
                        if let Some(direction) = direction {
                            let rv = self.move_unit(x, y, direction);
                            x = rv.0;
                            y = rv.1;
                        }
                        // If there's an elf around us, attack the lowest hp one.
                        if let Some((hp, x, y)) = self.enemy_adjacent(x, y, &cell) {
                            let hp = hp - 3;
                            if hp <= 0 {
                                self.cells[y][x] = Kind::Empty;
                                cells[y][x] = Kind::Empty;
                                self.elf_count -= 1;
                            } else {
                                self.cells[y][x] = Kind::Elf { hp };
                            }
                        }
                    }
                    Kind::Elf { .. } => {
                        if self.goblin_count == 0 {
                            full_turn = false;
                        }
                        let mut x = x;
                        let mut y = y;
                        let direction = self.find_closest(x, y, &cell);
                        if let Some(direction) = direction {
                            let rv = self.move_unit(x, y, direction);
                            x = rv.0;
                            y = rv.1;
                        }
                        // If there's a goblin around us, attack the lowest hp one.
                        if let Some((hp, x, y)) = self.enemy_adjacent(x, y, &cell) {
                            let hp = hp - self.elf_power;
                            if hp <= 0 {
                                self.cells[y][x] = Kind::Empty;
                                cells[y][x] = Kind::Empty;
                                self.goblin_count -= 1;
                            } else {
                                self.cells[y][x] = Kind::Goblin { hp };
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        full_turn
    }

    fn calculate_score(&self, turn: i32) -> i32 {
        let mut score = 0;
        for row in &self.cells {
            for cell in row {
                match cell {
                    Kind::Goblin { hp } => score += hp,
                    Kind::Elf { hp } => score += hp,
                    _ => {}
                }
            }
        }
        score * turn
    }
}

fn process_data_a(data: &str) -> i32 {
    let mut board: Board = data.parse().unwrap();
    let mut i = 0;
    while board.elf_count > 0 && board.goblin_count > 0 {
        if board.step() {
            i += 1;
        }
    }
    board.calculate_score(i)
}

fn process_data_b(data: &str) -> i32 {
    let board: Board = data.parse().unwrap();
    let elf_count = board.elf_count;
    let mut elf_power = 4;
    loop {
        let mut curr = board.clone();
        curr.elf_power = elf_power;
        let mut i = 0;
        while curr.elf_count == elf_count && curr.goblin_count > 0 {
            if curr.step() {
                i += 1;
            }
        }
        if curr.goblin_count == 0 {
            return curr.calculate_score(i);
        }
        elf_power += 1;
    }
}

//-----------------------------------------------------
// Questions.

q_impl!("15");

#[test]
fn a() {
    assert_eq!(
        process_data_a(
            "#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######"
        ),
        27730
    );
    assert_eq!(
        process_data_a(
            "#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######"
        ),
        36334
    );
    assert_eq!(
        process_data_a(
            "#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######"
        ),
        39514
    );
    assert_eq!(
        process_data_a(
            "#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######"
        ),
        27755
    );
    assert_eq!(
        process_data_a(
            "#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######"
        ),
        28944
    );
    assert_eq!(
        process_data_a(
            "#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########"
        ),
        18740
    );
}

#[test]
fn b() {
    assert_eq!(
        process_data_b(
            "#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######"
        ),
        4988
    );
    assert_eq!(
        process_data_b(
            "#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######"
        ),
        31284
    );
    assert_eq!(
        process_data_b(
            "#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######"
        ),
        3478
    );
    assert_eq!(
        process_data_b(
            "#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######"
        ),
        6474
    );
    assert_eq!(
        process_data_b(
            "#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########"
        ),
        1140
    );
}
