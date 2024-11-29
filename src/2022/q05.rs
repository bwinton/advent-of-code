//-----------------------------------------------------
// Setup.

use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, digit1, line_ending},
    multi::{many1, separated_list1},
    sequence::{delimited, separated_pair, tuple},
};

type Board = Vec<Vec<char>>;
type Move = (usize, usize, usize);

static INPUT: &str = include_str!("data/q05.data");

fn cell(i: &str) -> IResult<&str, Option<char>> {
    let (input, board) = alt((
        delimited(tag("["), alpha1, tag("]")),
        tag("   "),
        delimited(tag(" "), digit1, tag(" ")),
    ))(i)?;
    let board = match board {
        "   " => None,
        board if board.chars().next().unwrap().is_ascii_digit() => None,
        board if board.chars().next().unwrap().is_ascii_uppercase() => {
            Some(board.chars().next().unwrap())
        }
        _ => {
            println!("Invalid board line: {}", board);
            None
        }
    };
    Ok((input, board))
}

fn line(i: &str) -> IResult<&str, Vec<Option<char>>> {
    let (input, board) = separated_list1(tag(" "), cell)(i)?;
    Ok((input, board))
}

fn rule(i: &str) -> IResult<&str, Move> {
    // move 1 from 7 to 4
    let (input, (_, count, _, from, _, to)) = tuple((
        tag("move "),
        complete::u16,
        tag(" from "),
        complete::u16,
        tag(" to "),
        complete::u16,
    ))(i)?;
    Ok((input, (count as usize, from as usize, to as usize)))
}

fn parser(i: &str) -> IResult<&str, (Board, Vec<Move>)> {
    let (input, (line_data, rules)) = separated_pair(
        separated_list1(line_ending, line),
        many1(line_ending),
        separated_list1(line_ending, rule),
    )(i)?;
    let mut board = vec![];
    for _ in 0..line_data[0].len() {
        board.push(vec![]);
    }
    for d in line_data {
        for r in d.iter().enumerate() {
            if let Some(c) = r.1 {
                board[r.0].push(*c);
            }
        }
    }
    for col in board.iter_mut() {
        col.reverse()
    }
    Ok((input, (board, rules)))
}

fn process_data_a(data: &str) -> String {
    let mut rv = "".to_owned();
    let (_i, (mut board, rules)) = parser(data).unwrap();
    for (count, from, to) in rules {
        for _ in 0..count {
            let value = board[from - 1].pop().unwrap();
            board[to - 1].push(value);
        }
    }
    for mut col in board {
        rv.push(col.pop().unwrap());
    }
    rv
}

fn process_data_b(data: &str) -> String {
    let mut rv = "".to_owned();
    let (_i, (mut board, rules)) = parser(data).unwrap();
    for (count, from, to) in rules {
        let mut values = vec![];
        for _ in 0..count {
            values.push(board[from - 1].pop().unwrap());
        }
        values.reverse();
        for value in values {
            board[to - 1].push(value);
        }
    }
    for mut col in board {
        rv.push(col.pop().unwrap());
    }
    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("5");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "    [D]    
    [N] [C]    
    [Z] [M] [P]
     1   2   3 
    
    move 1 from 2 to 1
    move 3 from 1 to 3
    move 2 from 2 to 1
    move 1 from 1 to 2
    "
        )),
        "CMZ"
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "    [D]    
    [N] [C]    
    [Z] [M] [P]
     1   2   3 
    
    move 1 from 2 to 1
    move 3 from 1 to 3
    move 2 from 2 to 1
    move 1 from 1 to 2
    "
        )),
        "MCD"
    );
}
