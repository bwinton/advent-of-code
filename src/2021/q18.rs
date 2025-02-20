use std::{
    fmt::{Display, Write},
    ops::AddAssign,
};

use itertools::Itertools;

//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q18.data");

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Character {
    Open,
    Number(u8),
    Close,
}

impl Character {
    fn is_number(&self) -> bool {
        matches!(self, Character::Number(_))
    }
}

impl Display for Character {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Character::Open => f.write_char('['),
            Character::Close => f.write_char(']'),
            Character::Number(x) => f.write_str(&x.to_string()),
        }
    }
}

impl AddAssign for Character {
    fn add_assign(&mut self, rhs: Self) {
        if let (Character::Number(x), Character::Number(y)) = (self, rhs) {
            *x += y;
        }
    }
}

fn reduce(number: &mut Vec<Character>) {
    let mut done = false;
    'outer: while !done {
        done = true;

        let mut depth = 0;
        for (i, c) in number.iter().enumerate() {
            match *c {
                Character::Open => {
                    depth += 1;
                }
                Character::Close => {
                    depth -= 1;
                }
                Character::Number(_) => {}
            }
            if depth > 4 {
                // If any pair is nested inside four pairs, the leftmost such pair explodes.
                let mut start = i;
                let mut end = start;
                while number[end] != Character::Close {
                    end += 1;
                }
                let old: Vec<Character> = number
                    .splice(start..=end, [Character::Number(0)].into_iter())
                    .collect();
                start -= 1;
                end = start + 2;
                let first = old[1];
                let second = old[2];
                if !first.is_number() || !second.is_number() {
                    panic!("Exploding non numbers!!!!!!! {:?}\n\n", old);
                }
                while start > 0 && !number[start].is_number() {
                    start -= 1;
                }
                number[start] += first;
                while end < number.len() - 1 && !number[end].is_number() {
                    end += 1;
                }
                number[end] += second;
                done = false;
                continue 'outer;
            }
        }
        for (i, c) in number.iter().enumerate() {
            if let Character::Number(x) = *c {
                // If any regular number is 10 or greater, the leftmost such regular number splits.
                if x >= 10 {
                    let start = x / 2;
                    let end = x.div_ceil(2);
                    let new = [
                        Character::Open,
                        Character::Number(start),
                        Character::Number(end),
                        Character::Close,
                    ];
                    number.splice(i..=i, new.into_iter());
                    done = false;
                    continue 'outer;
                }
            }
        }
    }
}

fn parse(line: &str) -> Vec<Character> {
    let number: Vec<Character> = line
        .chars()
        .filter_map(|c| match c {
            '[' => Some(Character::Open),
            ']' => Some(Character::Close),
            ',' => None,
            _ => Some(Character::Number(c.to_string().parse::<u8>().unwrap())),
        })
        .collect();
    number
}

fn magnitude(number: &mut Vec<Character>) -> usize {
    let curr = number.splice(0..1, vec![]).collect::<Vec<_>>()[0];
    match curr {
        Character::Open => {
            let first = magnitude(number);
            let second = magnitude(number);
            let close = number.splice(0..1, vec![]).collect::<Vec<_>>()[0];
            if close != Character::Close {
                panic!("Unmatched brackets!!!");
            }
            first * 3 + second * 2
        }
        Character::Number(x) => x as usize,
        Character::Close => {
            panic!("Unexpected Close!!!");
        }
    }
}

fn get_lines(data: &str) -> Vec<Vec<Character>> {
    let mut rv = vec![];
    for line in data.lines() {
        let number = parse(line);
        rv.push(number);
    }
    rv
}

fn add_lines(numbers: &[Vec<Character>]) -> Vec<Character> {
    let mut rv = vec![];

    for number in numbers.iter().cloned() {
        if rv.is_empty() {
            rv = number;
        } else {
            rv.insert(0, Character::Open);
            rv.extend_from_slice(&number);
            rv.push(Character::Close);
        }
        reduce(&mut rv);
    }
    rv
}

fn process_data_a(data: &str) -> usize {
    let lines = get_lines(data);
    let mut rv = add_lines(&lines);
    magnitude(&mut rv)
}

fn process_data_b(data: &str) -> usize {
    let mut max = 0;
    let lines = get_lines(data);
    for pair in lines.iter().cloned().permutations(2) {
        let mut rv = add_lines(&pair);
        let _temp = rv.clone();
        let test = magnitude(&mut rv);
        if test > max {
            max = test;
        }
    }
    max
}

//-----------------------------------------------------
// Questions.

q_impl!("18");

#[test]
fn a() {
    fn test_reduce(line: &str) -> Vec<Character> {
        let mut test = parse(line);
        reduce(&mut test);
        test
    }
    assert_eq!(
        test_reduce("[[[[[9,8],1],2],3],4]"),
        parse("[[[[0,9],2],3],4]")
    );
    assert_eq!(
        test_reduce("[7,[6,[5,[4,[3,2]]]]]"),
        parse("[7,[6,[5,[7,0]]]]")
    );
    assert_eq!(
        test_reduce("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]"),
        parse("[[3,[2,[8,0]]],[9,[5,[7,0]]]]")
    );
    assert_eq!(
        test_reduce("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]"),
        parse("[[3,[2,[8,0]]],[9,[5,[7,0]]]]")
    );

    assert_eq!(
        test_reduce("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]"),
        parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
    );

    assert_eq!(
        test_reduce(
            "[[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]],[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]]"
        ),
        parse("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]")
    );
    assert_eq!(process_data_a("[9,1]"), 29);
    assert_eq!(process_data_a("[1,9]"), 21);
    assert_eq!(process_data_a("[[9,1],[1,9]]"), 129);
    assert_eq!(process_data_a("[[1,2],[[3,4],5]]"), 143);
    assert_eq!(process_data_a("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"), 1384);
    assert_eq!(process_data_a("[[[[1,1],[2,2]],[3,3]],[4,4]]"), 445);
    assert_eq!(process_data_a("[[[[3,0],[5,3]],[4,4]],[5,5]]"), 791);
    assert_eq!(process_data_a("[[[[5,0],[7,4]],[5,5]],[6,6]]"), 1137);
    assert_eq!(
        process_data_a("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"),
        3488
    );

    assert_eq!(
        add_lines(&get_lines(indoc!(
            "[1,1]
    [2,2]
    [3,3]
    [4,4]
    "
        ))),
        parse("[[[[1,1],[2,2]],[3,3]],[4,4]]")
    );

    assert_eq!(
        add_lines(&get_lines(indoc!(
            "[1,1]
    [2,2]
    [3,3]
    [4,4]
    [5,5]
    "
        ))),
        parse("[[[[3,0],[5,3]],[4,4]],[5,5]]")
    );

    assert_eq!(
        add_lines(&get_lines(indoc!(
            "[1,1]
    [2,2]
    [3,3]
    [4,4]
    [5,5]
    [6,6]
    "
        ))),
        parse("[[[[5,0],[7,4]],[5,5]],[6,6]]")
    );

    assert_eq!(
        add_lines(&get_lines(indoc!(
            "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
    [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
    [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
    [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
    [7,[5,[[3,8],[1,4]]]]
    [[2,[2,2]],[8,[8,1]]]
    [2,9]
    [1,[[[9,3],9],[[9,0],[0,7]]]]
    [[[5,[7,4]],7],1]
    [[[[4,2],2],6],[8,7]]
    "
        ))),
        parse("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
    );

    assert_eq!(
        process_data_a(indoc!(
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
    [[[5,[2,8]],4],[5,[[9,9],0]]]
    [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
    [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
    [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
    [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
    [[[[5,4],[7,7]],8],[[8,3],8]]
    [[9,3],[[9,9],[6,[4,9]]]]
    [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
    [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
    "
        )),
        4140
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
    [[[5,[2,8]],4],[5,[[9,9],0]]]
    [6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
    [[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
    [[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
    [[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
    [[[[5,4],[7,7]],8],[[8,3],8]]
    [[9,3],[[9,9],[6,[4,9]]]]
    [[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
    [[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
    "
        )),
        3993
    );
}
