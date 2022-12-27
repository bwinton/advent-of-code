use std::iter;

//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q20.data");

fn parse_algorithm(data: &str) -> [bool; 512] {
    let mut rv = [false; 512];
    let data: Vec<char> = data.chars().collect();
    for (i, &curr) in data.iter().enumerate() {
        if curr == '#' {
            rv[i] = true
        };
    }
    rv
}

fn get_index(pixel: &(i32, i32), image: &[Vec<bool>], background: bool) -> usize {
    let mut rv = 0;
    for row in -1..=1 {
        for column in -1..=1 {
            let x = pixel.0 + column;
            let y = pixel.1 + row;
            if x < 0 || x >= image.len() as i32 || y < 0 || y >= image[x as usize].len() as i32 {
                rv += if background { 1 } else { 0 };
            } else {
                rv += if image[y as usize][x as usize] { 1 } else { 0 };
            }
            rv <<= 1;
        }
    }
    rv >> 1
}

fn step(board: &[Vec<bool>], algorithm: &[bool; 512], background: bool) -> Vec<Vec<bool>> {
    let mut rv: Vec<Vec<bool>> = vec![];

    for (y, row) in board.iter().enumerate() {
        let mut new = vec![];
        for (x, _) in row.iter().enumerate() {
            let index = get_index(&(x as i32, y as i32), board, background);
            new.push(algorithm[index]);
        }
        rv.push(new);
    }
    rv
}

fn parse_board(data: &str) -> ([bool; 512], Vec<Vec<bool>>) {
    let mut lines = data.lines();
    let algorithm = lines.next().unwrap();
    let algorithm = parse_algorithm(algorithm);
    lines.next();
    let mut board = vec![];
    for line in lines {
        let mut row = iter::repeat(false).take(100).collect::<Vec<_>>();
        row.extend(line.chars().map(|c| c == '#').collect::<Vec<_>>());
        row.extend(iter::repeat(false).take(100));
        board.push(row);
    }

    let len = board[0].len();
    let full_row = iter::repeat(false).take(len).collect::<Vec<_>>();
    let mut top = vec![];
    for _ in 0..100 {
        top.push(full_row.clone());
    }
    let mut new = top.clone();
    new.extend(board);
    new.extend(top);
    (algorithm, new)
}

fn process_data_a(data: &str) -> usize {
    let (algorithm, mut board) = parse_board(data);

    let image_flip = algorithm[0] && !algorithm[511];
    let mut background = false;

    for _ in 0..2 {
        board = step(&board, &algorithm, background);
        if image_flip {
            background = !background;
        }
    }
    board.iter().flatten().filter(|&&x| x).count()

    // 5382 is too low
    // 5391 is too low
    // ****** 5395 is the answer *****
    // 6037 is too high
    // 6283 is too high
}

fn process_data_b(data: &str) -> usize {
    let (algorithm, mut board) = parse_board(data);

    let image_flip = algorithm[0] && !algorithm[511];
    let mut background = false;

    for _ in 0..50 {
        board = step(&board, &algorithm, background);
        if image_flip {
            background = !background;
        }
    }
    board.iter().flatten().filter(|&&x| x).count()
}

//-----------------------------------------------------
// Questions.

q_impl!("20");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_a(indoc!("..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#
    
    #..#.
    #....
    ##..#
    ..#..
    ..###
    ")), 35);
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_b(indoc!("..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#
    
    #..#.
    #....
    ##..#
    ..#..
    ..###
    ")), 3351);
}
