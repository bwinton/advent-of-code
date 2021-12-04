//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q04.data");

fn get_score(number: u32, board: &[Vec<(u32, bool)>]) -> u32 {
    let mut unmarked = 0;
    for row in board {
        for item in row {
            if !item.1 {
                unmarked += item.0;
            }
        }
    }
    // println!("{}*{} = {}", unmarked, number, number * unmarked);
    number * unmarked
}

fn process_data_a(data: &str) -> u32 {
    let mut lines = data.lines();
    let numbers: Vec<u32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    lines.next();
    let mut boards = vec![];
    let mut board = vec![];
    for line in lines {
        if line.is_empty() {
            // Done the board.
            boards.push(board);
            board = vec![];
            continue;
        }
        let row: Vec<(u32, bool)> = line
            .split_ascii_whitespace()
            .map(|x| (x.parse().unwrap(), false))
            .collect();
        board.push(row);
    }
    if !board.is_empty() {
        boards.push(board);
    }

    for number in numbers {
        // Toggle the numbers.
        for board in boards.iter_mut() {
            for row in board.iter_mut() {
                for item in row.iter_mut() {
                    if item.0 == number {
                        item.1 = true;
                    }
                }
            }
        }

        // Check the boards
        for board in &boards {
            for row in board {
                if row.iter().all(|&(_, checked)| checked) {
                    return get_score(number, board);
                }
            }
            'column: for i in 0..board[0].len() {
                for row in board.iter() {
                    if !row[i].1 {
                        continue 'column;
                    }
                }
                return get_score(number, board);
            }
        }
    }
    0
}

fn process_data_b(data: &str) -> u32 {
    let mut lines = data.lines();
    let numbers: Vec<u32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    lines.next();
    let mut boards = vec![];
    let mut board = vec![];
    for line in lines {
        if line.is_empty() {
            // Done the board.
            boards.push(board);
            board = vec![];
            continue;
        }
        let row: Vec<(u32, bool)> = line
            .split_ascii_whitespace()
            .map(|x| (x.parse().unwrap(), false))
            .collect();
        board.push(row);
    }
    boards.push(board);

    for number in numbers {
        // Toggle the numbers.
        for board in boards.iter_mut() {
            for row in board.iter_mut() {
                for item in row.iter_mut() {
                    if item.0 == number {
                        item.1 = true;
                    }
                }
            }
        }

        let last_board = boards[0].clone();
        boards.retain(|board| {
            // Check the boards
            for row in board {
                if row.iter().all(|&(_, checked)| checked) {
                    return false;
                }
            }
            'column: for i in 0..board[0].len() {
                for row in board {
                    if !row[i].1 {
                        continue 'column;
                    }
                }
                return false;
            }
            true
        });
        if boards.is_empty() {
            return get_score(number, &last_board);
        }
    }
    0
}

//-----------------------------------------------------
// Questions.

q_impl!("4");

#[test]
fn a() {
    assert_eq!(
        process_data_a(indoc!(
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19
    
     3 15  0  2 22
     9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6
    
    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
     2  0 12  3  7
"
        )),
        4512
    );
}

#[test]
fn b() {
    assert_eq!(
        process_data_b(indoc!(
            "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19
    
     3 15  0  2 22
     9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6
    
    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
     2  0 12  3  7
    "
        )),
        1924
    );
}
