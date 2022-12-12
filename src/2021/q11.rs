//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q11.data");

fn increment_neighbours(board: &mut [Vec<(u8, bool)>], x: i8, y: i8) {
    let i_low: i8 = if x == 0 { 0 } else { -1 };
    let i_high: i8 = if x == (board.len() - 1) as i8 { 0 } else { 1 };
    let j_low: i8 = if y == 0 { 0 } else { -1 };
    let j_high: i8 = if y == (board[0].len() - 1) as i8 {
        0
    } else {
        1
    };
    for i in i_low..=i_high {
        for j in j_low..=j_high {
            if i == 0 && j == 0 {
                board[x as usize][y as usize].1 = true;
            }
            if board[(x + i) as usize][(y + j) as usize].0 < 10 {
                board[(x + i) as usize][(y + j) as usize].0 += 1;
            }
        }
    }
}

fn step(board: &[Vec<u8>]) -> (usize, Vec<Vec<u8>>) {
    let mut flashes = 0;
    let mut next = vec![];
    for line in board {
        let mut row = vec![];
        for cell in line {
            row.push((*cell, false));
        }
        next.push(row);
    }

    for line in next.iter_mut() {
        for mut cell in line.iter_mut() {
            cell.0 += 1;
        }
    }

    let mut found = true;
    while found {
        found = false;
        let mut next2 = next.clone();
        for (i, line) in next.iter().enumerate() {
            for (j, cell) in line.iter().enumerate() {
                if cell.0 > 9 && !next2[i][j].1 {
                    increment_neighbours(&mut next2, i as i8, j as i8);
                    // println!("Found [{}][{}], {:?} == {:?}", i, j, cell, next2[i][j]);
                    found = true;
                }
            }
        }
        // println!("\nIn loop… {}", found);
        // for line in &next {
        //     println!("  {}", line.iter().map(|x| x.0.to_string()).join(" "));
        // }
        next = next2;
        // println!();
        // for line in &next {
        //     println!("  {}", line.iter().map(|x| x.0.to_string()).join(" "));
        // }
    }

    for line in next.iter_mut() {
        for cell in line.iter_mut() {
            if cell.1 {
                flashes += 1;
                cell.0 = 0;
            }
        }
    }

    let mut rv = vec![];
    for line in next {
        let mut row = vec![];
        for cell in line {
            row.push(cell.0);
        }
        rv.push(row);
    }

    (flashes, rv)
}

fn process_data_a(data: &str) -> usize {
    let mut rv = 0;
    let mut board = vec![];
    for line in data.lines() {
        let line: Vec<u8> = line
            .chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect();
        board.push(line);
    }

    for _ in 0..100 {
        let (flashes, next) = step(&board);
        // println!("\nAfter {} steps, flashes: {}\n", i + 1, flashes);
        // for line in &next {
        //     println!("  {}", line.iter().map(|x| x.to_string()).join(" "));
        // }
        rv += flashes;
        board = next;
    }
    rv
}

fn process_data_b(data: &str) -> usize {
    let mut board = vec![];
    for line in data.lines() {
        let line: Vec<u8> = line
            .chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect();
        board.push(line);
    }

    let mut i = 0;
    let mut found = false;
    while !found {
        let (flashes, next) = step(&board);
        if flashes == board.len() * board[0].len() {
            found = true;
        }
        i += 1;
        // if i > 190 {
        //     println!("\nAfter {} steps, flashes: {}\n", i, flashes);
        //     for line in &next {
        //         println!("  {}", line.iter().map(|x| x.to_string()).join(" "));
        //     }
        // }
        // if i > 200 {
        //     found = true;
        // }
        board = next;
    }
    i
}

//-----------------------------------------------------
// Questions.

q_impl!("11");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "5483143223
    2745854711
    5264556173
    6141336146
    6357385478
    4167524645
    2176841721
    6882881134
    4846848554
    5283751526"
        )),
        1656
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "5483143223
    2745854711
    5264556173
    6141336146
    6357385478
    4167524645
    2176841721
    6882881134
    4846848554
    5283751526"
        )),
        195
    );
}
