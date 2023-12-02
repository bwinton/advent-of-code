use std::collections::VecDeque;

//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q18.data");

enum Op {
    Plus,
    Times,
}

fn process(line: &mut VecDeque<char>) -> usize {
    let mut rv = 0;
    let mut op = None;
    let mut number = String::new();
    while !line.is_empty() {
        let token = line.pop_front().unwrap();
        match token {
            ' ' => {
                if !number.is_empty() {
                    let value = number.parse::<usize>().unwrap();
                    number = String::new();

                    match op {
                        Some(Op::Plus) => {
                            rv += value;
                        }
                        Some(Op::Times) => {
                            rv *= value;
                        }
                        None => {
                            rv = value;
                        }
                    };
                }
            }
            '+' => {
                op = Some(Op::Plus);
            }
            '*' => {
                op = Some(Op::Times);
            }
            '(' => {
                let value = process(line);
                match op {
                    Some(Op::Plus) => {
                        rv += value;
                    }
                    Some(Op::Times) => {
                        rv *= value;
                    }
                    None => {
                        rv = value;
                    }
                };
            }
            ')' => {
                if !number.is_empty() {
                    let value = number.parse::<usize>().unwrap();
                    match op {
                        Some(Op::Plus) => {
                            rv += value;
                        }
                        Some(Op::Times) => {
                            rv *= value;
                        }
                        None => {
                            rv = value;
                        }
                    };
                }
                return rv;
            }
            ch if ch.is_ascii_digit() => {
                number.push(ch);
            }
            _ => {
                panic!("Unknown character!");
            }
        }
    }
    if !number.is_empty() {
        let value = number.parse::<usize>().unwrap();

        match op {
            Some(Op::Plus) => {
                rv += value;
            }
            Some(Op::Times) => {
                rv *= value;
            }
            None => {
                rv = value;
            }
        };
    }

    rv
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Token {
    Plus,
    Times,
    Value(usize),
}

fn eval(stack: Vec<Token>) -> usize {
    let mut tokens = stack.iter();
    let mut next = vec![];
    while let Some(token) = tokens.next() {
        let len = next.len();
        if token == &Token::Plus {
            let a = next[len - 1];
            let &b = tokens.next().unwrap();

            match (a, b) {
                (Token::Value(prev), Token::Value(following)) => {
                    next[len - 1] = Token::Value(prev + following);
                }
                _ => {
                    panic!("Trying to splice non-values.")
                }
            }
        } else {
            next.push(*token);
        }
    }

    let mut end = vec![];
    tokens = next.iter();
    while let Some(token) = tokens.next() {
        let len = end.len();
        if token == &Token::Times {
            let a = end[len - 1];
            let &b = tokens.next().unwrap();

            match (a, b) {
                (Token::Value(prev), Token::Value(following)) => {
                    end[len - 1] = Token::Value(prev * following);
                }
                _ => {
                    panic!("Trying to splice non-values.")
                }
            }
        } else {
            end.push(*token);
        }
    }

    if end.len() == 1 {
        match end[0] {
            Token::Value(value) => value,
            _ => {
                panic!("Unknown token {:?}", &end[0]);
            }
        }
    } else {
        panic!("More than one token!");
    }
}

fn process_b(line: &mut VecDeque<char>) -> usize {
    let mut number = String::new();
    let mut stack = vec![];
    while !line.is_empty() {
        let token = line.pop_front().unwrap();
        match token {
            ' ' => {
                if !number.is_empty() {
                    let value = number.parse::<usize>().unwrap();
                    number = String::new();
                    stack.push(Token::Value(value));
                }
            }
            '+' => {
                stack.push(Token::Plus);
            }
            '*' => {
                stack.push(Token::Times);
            }
            '(' => {
                let value = process_b(line);
                stack.push(Token::Value(value));
            }
            ')' => {
                if !number.is_empty() {
                    let value = number.parse::<usize>().unwrap();
                    stack.push(Token::Value(value));
                }
                return eval(stack);
            }
            ch if ch.is_ascii_digit() => {
                number.push(ch);
            }
            _ => {
                panic!("Unknown character!");
            }
        }
    }
    if !number.is_empty() {
        let value = number.parse::<usize>().unwrap();
        stack.push(Token::Value(value));
    }

    eval(stack)
}

fn process_data_a(data: &str) -> usize {
    let mut rv = 0;
    for line in data.lines() {
        rv += process(&mut line.chars().collect());
    }
    rv
}

fn process_data_b(data: &str) -> usize {
    let mut rv = 0;
    for line in data.lines() {
        rv += process_b(&mut line.chars().collect());
    }
    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("18");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_a("1 + 2 * 3 + 4 * 5 + 6"), 71);
    assert_eq!(process_data_a("1 + (2 * 3) + (4 * (5 + 6))"), 51);
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_b("1 + (2 * 3) + (4 * (5 + 6))"), 51);
    assert_eq!(process_data_b("1 + 2 * 3 + 4 * 5 + 6"), 231);
}
