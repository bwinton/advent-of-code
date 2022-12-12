use std::{
    collections::{HashMap, HashSet},
    str::Lines,
};

//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q04.data");

type Board = Vec<Vec<(u32, bool)>>;
type Location = (usize, usize, usize);

fn get_numbers(lines: &mut Lines) -> Vec<u32> {
    let numbers: Vec<u32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    lines.next();
    numbers
}

fn get_boards(lines: Lines) -> (Vec<Board>, HashMap<u32, Vec<Location>>) {
    let mut boards = vec![];
    let mut board = vec![];
    let mut numbers = HashMap::new();
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
        for (col, value) in row.iter().enumerate() {
            numbers
                .entry(value.0)
                .or_insert_with(Vec::new)
                .push((boards.len(), board.len(), col));
        }
        board.push(row);
    }
    if !board.is_empty() {
        boards.push(board);
    }
    (boards, numbers)
}

fn check_board(board: &[Vec<(u32, bool)>], row: usize, col: usize) -> bool {
    // Check the row…
    if board[row].iter().all(|&(_, checked)| checked) {
        return true;
    }

    // Check the column…
    let mut rv = true;
    for row in board.iter().take(board[0].len()) {
        rv &= row[col].1;
    }

    rv
}

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

    let numbers = get_numbers(&mut lines);
    let (mut boards, cache) = get_boards(lines);

    for number in numbers {
        // Toggle the numbers.
        if let Some(occurrences) = cache.get(&number) {
            for &(board, row, col) in occurrences {
                let item = &mut boards[board][row][col];
                item.1 = true;
                if check_board(&boards[board], row, col) {
                    return get_score(number, &boards[board]);
                }
            }
        }
    }
    0
}

fn process_data_b(data: &str) -> u32 {
    let mut lines = data.lines();

    let numbers = get_numbers(&mut lines);
    let (mut boards, cache) = get_boards(lines);
    let mut remaining: HashSet<usize> = (0..boards.len()).collect();

    for number in numbers {
        let &last_board = remaining.iter().next().unwrap();

        // Toggle the numbers.
        if let Some(occurrences) = cache.get(&number) {
            for &(board, row, col) in occurrences {
                if boards[board].is_empty() {
                    continue;
                }
                let item = &mut boards[board][row][col];
                item.1 = true;
                if check_board(&boards[board], row, col) {
                    remaining.remove(&board);
                }
            }
        }

        if remaining.is_empty() {
            return get_score(number, &boards[last_board]);
        }
    }
    0
}

//-----------------------------------------------------
// Questions.

q_impl!("4");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

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
    use pretty_assertions::assert_eq;

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
