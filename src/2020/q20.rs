use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
};

//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q20.data");

static SEA_MONSTER: [(usize, usize); 15] = [
    (0, 18),
    (1, 0),
    (1, 5),
    (1, 6),
    (1, 11),
    (1, 12),
    (1, 17),
    (1, 18),
    (1, 19),
    (2, 1),
    (2, 4),
    (2, 7),
    (2, 10),
    (2, 13),
    (2, 16),
];

#[derive(Debug, Clone)]
struct Tile {
    name: String,
    data: Vec<String>,
}

// Tile 1787:
// ###...####
// #.##..##.#
// ...##.#.#.
// #.........
// .......#.#
// .#..#...##
// .##.#..#..
// #...#....#
// #..#.....#
// .#...##.#.

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name.as_str())?;
        f.write_str("\n")?;
        for line in &self.data {
            f.write_str(line.as_str())?;
            f.write_str("\n")?;
        }
        Ok(())
    }
}

impl Tile {
    fn left(&self) -> String {
        let mut rv = String::new();
        for row in &self.data {
            rv.push(row.chars().next().unwrap());
        }
        rv
    }
    fn right(&self) -> String {
        let mut rv = String::new();
        for row in &self.data {
            rv.push(row.chars().last().unwrap());
        }
        rv
    }
    fn top(&self) -> String {
        self.data[0].clone()
    }
    fn bottom(&self) -> String {
        self.data[self.data.len() - 1].clone()
    }

    fn flip_h(&self) -> Self {
        let mut data = vec![];
        for row in &self.data {
            let mut new = String::new();
            for char in row.chars().rev() {
                new.push(char);
            }
            data.push(new);
        }
        Tile {
            name: self.name.clone() + "-flip_h",
            data,
        }
    }

    fn flip_v(&self) -> Self {
        let mut data = self.data.clone();
        data.reverse();
        Tile {
            name: self.name.clone() + "-flip_v",
            data,
        }
    }

    fn rot_90(&self) -> Self {
        let mut data = vec![];
        for _ in 0..self.data.len() {
            data.push(String::new());
        }

        for row in self.data.iter() {
            for (i, char) in row.chars().enumerate() {
                data[i].push(char);
            }
        }
        for row in data.iter_mut() {
            *row = row.chars().rev().collect();
        }

        // println!("orig: {:?}\nrot90: {:?}", self.data, data);
        Tile {
            name: self.name.clone() + "-rot_90",
            data,
        }
    }

    fn rot_180(&self) -> Self {
        let mut data = vec![];
        for row in self.data.iter().rev() {
            let mut new = String::new();
            for char in row.chars().rev() {
                new.push(char);
            }
            data.push(new);
        }
        Tile {
            name: self.name.clone() + "-rot_180",
            data,
        }
    }

    fn rot_270(&self) -> Self {
        let mut data = vec![];
        let len = self.data.len();
        for _ in 0..len {
            data.push(String::new());
        }
        for row in self.data.iter() {
            for (i, char) in row.chars().enumerate() {
                data[len - i - 1].push(char);
            }
        }

        // println!("orig: {:?}\nrot90: {:?}", self.data, data);
        Tile {
            name: self.name.clone() + "-rot_270",
            data,
        }
    }
}

fn process_data_a(data: &str) -> usize {
    let mut tiles = HashMap::new();
    let mut tile_name = String::new();
    let mut tile_data = vec![];
    for line in data.lines() {
        // Do something
        if line.starts_with("Tile ") {
            tile_name = line[5..9].to_string();
        } else if line.is_empty() {
            let tile = Tile {
                name: tile_name.clone(),
                data: tile_data,
            };
            let tile_h = tile.flip_h();
            let tile_v = tile.flip_v();

            tiles.insert(tile.name.clone(), tile);
            tiles.insert(tile_h.name.clone(), tile_h);
            tiles.insert(tile_v.name.clone(), tile_v);

            tile_name = String::new();
            tile_data = vec![]
        } else {
            tile_data.push(line.to_string());
        }
    }
    // let width = (tile_count as f64).sqrt() as usize;

    let mut sides: HashMap<String, HashSet<String>> = HashMap::new();
    for tile in tiles.values() {
        let top = tile.top();
        let left = tile.left();
        let bottom = tile.bottom();
        let right = tile.right();
        sides
            .entry(top)
            .or_default()
            .insert(tile.name[..4].to_string());
        sides
            .entry(left)
            .or_default()
            .insert(tile.name[..4].to_string());
        sides
            .entry(bottom)
            .or_default()
            .insert(tile.name[..4].to_string());
        sides
            .entry(right)
            .or_default()
            .insert(tile.name[..4].to_string());
    }

    // println!("{:?}", sides);
    sides.retain(|_, value| value.len() == 1);
    // Build all the chains of length <width>.
    let mut tile_count: HashMap<String, usize> = HashMap::new();
    for (_, tiles) in sides {
        for tile in tiles {
            *tile_count.entry(tile).or_insert(0) += 1;
        }
    }
    tile_count.retain(|_, value| value == &4);
    let rv: usize = tile_count
        .keys()
        .map(|x| x.parse::<usize>().unwrap())
        .product();
    rv
}

fn find_monsters(board: &[String]) -> Option<Vec<Vec<char>>> {
    let mut count = 0;
    let mut board = board
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for y in 0..board.len() - 2 {
        for x in 0..board[y].len() - 19 {
            let mut found = true;
            for coords in &SEA_MONSTER {
                if board[y + coords.0][x + coords.1] != '#' {
                    found = false;
                    break;
                }
            }
            if found {
                count += 1;
                for coords in &SEA_MONSTER {
                    board[y + coords.0][x + coords.1] = 'O';
                }
            }
        }
    }
    if count == 0 {
        None
    } else {
        Some(board)
    }
}

fn process_data_b(data: &str) -> usize {
    let mut all_tiles = vec![];
    let mut tile_map: HashMap<String, Tile> = HashMap::new();
    let mut tile_name = String::new();
    let mut tile_data = vec![];
    let mut width = 0;
    for line in data.lines() {
        // Do something
        if line.starts_with("Tile ") {
            tile_name = line[5..9].to_string();
        } else if line.is_empty() {
            width += 1;
            let tile = Tile {
                name: tile_name.clone(),
                data: tile_data,
            };
            let tile_h = tile.flip_h();
            let tile_r90 = tile.rot_90();
            let tile_r90_h = tile_r90.flip_h();
            let tile_r180 = tile.rot_180();
            let tile_r180_h = tile_r180.flip_h();
            let tile_r270 = tile.rot_270();
            let tile_r270_h = tile_r270.flip_h();

            tile_map.insert(tile.clone().name, tile.clone());
            tile_map.insert(tile_h.clone().name, tile_h.clone());
            tile_map.insert(tile_r90.clone().name, tile_r90.clone());
            tile_map.insert(tile_r90_h.clone().name, tile_r90_h.clone());
            tile_map.insert(tile_r180.clone().name, tile_r180.clone());
            tile_map.insert(tile_r180_h.clone().name, tile_r180_h.clone());
            tile_map.insert(tile_r270.clone().name, tile_r270.clone());
            tile_map.insert(tile_r270_h.clone().name, tile_r270_h.clone());
            all_tiles.append(&mut vec![
                tile,
                tile_h,
                tile_r90,
                tile_r90_h,
                tile_r180,
                tile_r180_h,
                tile_r270,
                tile_r270_h,
            ]);

            tile_name = String::new();
            tile_data = vec![]
        } else {
            tile_data.push(line.to_string());
        }
    }
    let width = (width as f64).sqrt() as usize;

    let mut board: Vec<Vec<Option<String>>> = vec![];
    for _ in 0..width {
        let mut row = vec![];
        for _ in 0..width {
            row.push(None);
        }
        board.push(row);
    }

    // println!("All_Tiles:");
    // for tile in &all_tiles {
    //     println!("  {:?}", tile);
    // }
    // println!();

    // println!("Starting search…");
    let mut stack = VecDeque::new();
    stack.push_front((board, all_tiles, (0, 0)));
    let mut found: Option<Vec<Vec<String>>> = None;
    while !stack.is_empty() {
        let (board, all_tiles, (x, y)) = stack.pop_front().unwrap();
        // println!("\n{}:{}", stack.len(), &all_tiles.len());
        // for row in &board {
        //     println!("{:?}", row);
        // }
        if all_tiles.is_empty() {
            // println!("Found it!!!");
            found = Some(
                board
                    .iter()
                    .map(|row| row.iter().map(|cell| cell.clone().unwrap()).collect())
                    .collect(),
            );
            break;
        }
        for tile in &all_tiles {
            // check if the border fits to the top / left tile
            let mut fits = true;
            if x > 0 {
                let prev = &board[y][x - 1].clone().unwrap().clone();
                let prev = tile_map.get(prev).unwrap();
                if tile.left() != prev.right() {
                    fits = false;
                }
            }
            if y > 0 {
                let prev = &board[y - 1][x].clone().unwrap().clone();
                let prev = tile_map.get(prev).unwrap();
                if tile.top() != prev.bottom() {
                    fits = false;
                }
            }
            if fits {
                // println!("  {} fits!", tile.name);
                let mut board = board.clone();
                board[y][x] = Some(tile.name.clone());
                let mut x = x + 1;
                let mut y = y;
                if x == width {
                    y += 1;
                    x = 0;
                }
                let mut all_tiles = all_tiles.clone();
                // print!("{} ->", all_tiles.len());
                all_tiles.retain(|x| x.name[..4] != tile.name[..4]);
                // println!("{}", all_tiles.len());
                stack.push_front((board, all_tiles, (x, y)));
            }
        }
    }

    let mut board = vec![];
    for line in found.unwrap() {
        let mut row = vec![];
        for cell in line {
            let tile = tile_map.get(&cell).unwrap();
            let tile_width = tile.data.len();
            if row.is_empty() {
                for _ in 1..tile_width - 1 {
                    row.push(String::new());
                }
            }
            // println!("Tile: {:?}", tile);
            for (i, line) in tile.data[1..tile_width - 1].iter().enumerate() {
                // println!("{:?}", line[1..tile_width - 2].to_string());
                row[i] += &line[1..tile_width - 1];
            }
        }
        board.extend(row);
    }
    let board = Tile {
        name: "Full Board".to_string(),
        data: board,
    };
    // println!("Testing orig");
    if let Some(board) = find_monsters(&board.data) {
        return board.into_iter().flatten().filter(|&c| c == '#').count();
    }
    // println!("Testing orig_h");
    if let Some(board) = find_monsters(&board.flip_h().data) {
        return board.into_iter().flatten().filter(|&c| c == '#').count();
    }
    let board_r90 = board.rot_90();
    // println!("Testing r90");
    if let Some(board) = find_monsters(&board_r90.data) {
        return board.into_iter().flatten().filter(|&c| c == '#').count();
    }
    // println!("Testing r90_h");
    if let Some(board) = find_monsters(&board_r90.flip_h().data) {
        return board.into_iter().flatten().filter(|&c| c == '#').count();
    }
    let board_r180 = board.rot_180();
    // println!("Testing r180");
    if let Some(board) = find_monsters(&board_r180.data) {
        return board.into_iter().flatten().filter(|&c| c == '#').count();
    }
    // println!("Testing r180_h");
    if let Some(board) = find_monsters(&board_r180.flip_h().data) {
        return board.into_iter().flatten().filter(|&c| c == '#').count();
    }
    let board_r270 = board.rot_270();
    // println!("Testing r270");
    if let Some(board) = find_monsters(&board_r270.data) {
        return board.into_iter().flatten().filter(|&c| c == '#').count();
    }
    // println!("Testing r270_hh");
    if let Some(board) = find_monsters(&board_r270.flip_h().data) {
        return board.into_iter().flatten().filter(|&c| c == '#').count();
    }

    0
}

//-----------------------------------------------------
// Questions.

q_impl!("20");

#[test]
fn a() {
    let name = "Tile".to_string();
    let data = vec!["123".to_string(), "456".to_string(), "789".to_string()];
    let tile = Tile { name, data };
    // let tile_h = tile.flip_h();
    // let tile_v = tile.flip_v();
    // let tile_r90 = tile.rot_90();
    // let tile_r90h = tile_r90.flip_h();
    // let tile_r90v = tile_r90.flip_v();
    // let tile_r180 = tile.rot_180();
    // let tile_r180h = tile_r180.flip_h();
    // let tile_r180v = tile_r180.flip_v();
    // let tile_r270 = tile.rot_270();
    // let tile_r270h = tile_r270.flip_h();
    // let tile_r270v = tile_r270.flip_v();

    assert_eq!(tile.top(), "123".to_string());
    assert_eq!(tile.left(), "147".to_string());
    assert_eq!(tile.right(), "369".to_string());
    assert_eq!(tile.bottom(), "789".to_string());

    // println!("{}\n{}", tile.to_string(), tile_h.to_string());
    // println!("{}\n{}", tile.to_string(), tile_v.to_string());
    // println!("{}\n{}", tile.to_string(), tile_r90.to_string());
    // println!("{}\n{}", tile.to_string(), tile_r90h.to_string());
    // println!("{}\n{}", tile.to_string(), tile_r90v.to_string());
    // println!("{}\n{}", tile.to_string(), tile_r180.to_string());
    // println!("{}\n{}", tile.to_string(), tile_r180h.to_string());
    // println!("{}\n{}", tile.to_string(), tile_r180v.to_string());
    // println!("{}\n{}", tile.to_string(), tile_r270.to_string());
    // println!("{}\n{}", tile.to_string(), tile_r270h.to_string());
    // println!("{}\n{}", tile.to_string(), tile_r270v.to_string());

    assert_eq!(
        process_data_a(indoc!(
            "Tile 2311:
            ..##.#..#.
            ##..#.....
            #...##..#.
            ####.#...#
            ##.##.###.
            ##...#.###
            .#.#.#..##
            ..#....#..
            ###...#.#.
            ..###..###

            Tile 1951:
            #.##...##.
            #.####...#
            .....#..##
            #...######
            .##.#....#
            .###.#####
            ###.##.##.
            .###....#.
            ..#.#..#.#
            #...##.#..

            Tile 1171:
            ####...##.
            #..##.#..#
            ##.#..#.#.
            .###.####.
            ..###.####
            .##....##.
            .#...####.
            #.##.####.
            ####..#...
            .....##...

            Tile 1427:
            ###.##.#..
            .#..#.##..
            .#.##.#..#
            #.#.#.##.#
            ....#...##
            ...##..##.
            ...#.#####
            .#.####.#.
            ..#..###.#
            ..##.#..#.

            Tile 1489:
            ##.#.#....
            ..##...#..
            .##..##...
            ..#...#...
            #####...#.
            #..#.#.#.#
            ...#.#.#..
            ##.#...##.
            ..##.##.##
            ###.##.#..

            Tile 2473:
            #....####.
            #..#.##...
            #.##..#...
            ######.#.#
            .#...#.#.#
            .#########
            .###.#..#.
            ########.#
            ##...##.#.
            ..###.#.#.

            Tile 2971:
            ..#.#....#
            #...###...
            #.#.###...
            ##.##..#..
            .#####..##
            .#..####.#
            #..#.#..#.
            ..####.###
            ..#.#.###.
            ...#.#.#.#

            Tile 2729:
            ...#.#.#.#
            ####.#....
            ..#.#.....
            ....#..#.#
            .##..##.#.
            .#.####...
            ####.#.#..
            ##.####...
            ##..#.##..
            #.##...##.

            Tile 3079:
            #.#.#####.
            .#..######
            ..#.......
            ######....
            ####.#..#.
            .#...#.##.
            #.#####.##
            ..#.###...
            ..#.......
            ..#.###...

            "
        )),
        20899048083289
    );
}

#[test]
fn b() {
    assert_eq!(
        process_data_b(indoc!(
            "Tile 2311:
            ..##.#..#.
            ##..#.....
            #...##..#.
            ####.#...#
            ##.##.###.
            ##...#.###
            .#.#.#..##
            ..#....#..
            ###...#.#.
            ..###..###

            Tile 1951:
            #.##...##.
            #.####...#
            .....#..##
            #...######
            .##.#....#
            .###.#####
            ###.##.##.
            .###....#.
            ..#.#..#.#
            #...##.#..

            Tile 1171:
            ####...##.
            #..##.#..#
            ##.#..#.#.
            .###.####.
            ..###.####
            .##....##.
            .#...####.
            #.##.####.
            ####..#...
            .....##...

            Tile 1427:
            ###.##.#..
            .#..#.##..
            .#.##.#..#
            #.#.#.##.#
            ....#...##
            ...##..##.
            ...#.#####
            .#.####.#.
            ..#..###.#
            ..##.#..#.

            Tile 1489:
            ##.#.#....
            ..##...#..
            .##..##...
            ..#...#...
            #####...#.
            #..#.#.#.#
            ...#.#.#..
            ##.#...##.
            ..##.##.##
            ###.##.#..

            Tile 2473:
            #....####.
            #..#.##...
            #.##..#...
            ######.#.#
            .#...#.#.#
            .#########
            .###.#..#.
            ########.#
            ##...##.#.
            ..###.#.#.

            Tile 2971:
            ..#.#....#
            #...###...
            #.#.###...
            ##.##..#..
            .#####..##
            .#..####.#
            #..#.#..#.
            ..####.###
            ..#.#.###.
            ...#.#.#.#

            Tile 2729:
            ...#.#.#.#
            ####.#....
            ..#.#.....
            ....#..#.#
            .##..##.#.
            .#.####...
            ####.#.#..
            ##.####...
            ##..#.##..
            #.##...##.

            Tile 3079:
            #.#.#####.
            .#..######
            ..#.......
            ######....
            ####.#..#.
            .#...#.##.
            #.#####.##
            ..#.###...
            ..#.......
            ..#.###...

            "
        )),
        273
    );
}
