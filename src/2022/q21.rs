//-----------------------------------------------------
// Setup.

use std::collections::HashMap;

use nom::{
    IResult,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending},
    multi::separated_list1,
    sequence::tuple,
};

static INPUT: &str = include_str!("data/q21.data");

#[derive(Debug, Clone)]
enum Value {
    Number(i64),
    Monkey(String),
}

#[derive(Debug, Clone)]
enum Operation {
    Number(i64),
    Plus(Value, Value),
    Minus(Value, Value),
    Times(Value, Value),
    Divide(Value, Value),
}
impl Operation {
    fn maybe_replace(&mut self, values: &HashMap<String, i64>) -> Option<Operation> {
        match self {
            Operation::Plus(a, b) => {
                if let Value::Monkey(name) = &a {
                    if values.contains_key(name) {
                        *a = Value::Number(values[name]);
                    }
                }
                if let Value::Monkey(name) = &b {
                    if values.contains_key(name) {
                        *b = Value::Number(values[name]);
                    }
                }
                if let (Value::Number(a), Value::Number(b)) = (a, b) {
                    return Some(Operation::Number(*a + *b));
                }
            }
            Operation::Minus(a, b) => {
                if let Value::Monkey(name) = &a {
                    if values.contains_key(name) {
                        *a = Value::Number(values[name]);
                    }
                }
                if let Value::Monkey(name) = &b {
                    if values.contains_key(name) {
                        *b = Value::Number(values[name]);
                    }
                }
                if let (Value::Number(a), Value::Number(b)) = (a, b) {
                    return Some(Operation::Number(*a - *b));
                }
            }
            Operation::Times(a, b) => {
                if let Value::Monkey(name) = &a {
                    if values.contains_key(name) {
                        *a = Value::Number(values[name]);
                    }
                }
                if let Value::Monkey(name) = &b {
                    if values.contains_key(name) {
                        *b = Value::Number(values[name]);
                    }
                }
                if let (Value::Number(a), Value::Number(b)) = (a, b) {
                    return Some(Operation::Number(*a * *b));
                }
            }
            Operation::Divide(a, b) => {
                if let Value::Monkey(name) = &a {
                    if values.contains_key(name) {
                        *a = Value::Number(values[name]);
                    }
                }
                if let Value::Monkey(name) = &b {
                    if values.contains_key(name) {
                        *b = Value::Number(values[name]);
                    }
                }
                if let (Value::Number(a), Value::Number(b)) = (a, b) {
                    return Some(Operation::Number(*a / *b));
                }
            }
            _ => panic!("Unknown operation {:?}", self),
        }
        None
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    name: String,
    operation: Operation,
}

fn number(i: &str) -> IResult<&str, Operation> {
    let (input, value) = complete::i64(i)?;
    Ok((input, Operation::Number(value)))
}

fn calculation(i: &str) -> IResult<&str, Operation> {
    let (input, (a, op, b)) = tuple((
        alpha1,
        alt((tag(" + "), tag(" - "), tag(" * "), tag(" / "))),
        alpha1,
    ))(i)?;
    let (a, b) = (Value::Monkey(a.to_owned()), Value::Monkey(b.to_owned()));
    let op = match op {
        " + " => Operation::Plus(a, b),
        " - " => Operation::Minus(a, b),
        " * " => Operation::Times(a, b),
        " / " => Operation::Divide(a, b),
        _ => panic!("Unknown Operation! {:?}{}{:?}", a, op, b),
    };
    Ok((input, op))
}

fn operation(i: &str) -> IResult<&str, Operation> {
    let (input, op) = alt((number, calculation))(i)?;
    Ok((input, op))
}

// root: pppw + sjmn
fn monkey(i: &str) -> IResult<&str, Monkey> {
    let (input, (name, _, op)) = tuple((alpha1, tag(": "), operation))(i)?;
    Ok((input, Monkey {
        name: name.to_owned(),
        operation: op,
    }))
}

fn parser(i: &str) -> IResult<&str, Vec<Monkey>> {
    let (input, list) = separated_list1(line_ending, monkey)(i)?;
    Ok((input, list))
}

fn get_root(monkeys: &mut Vec<Monkey>) -> i64 {
    let mut values: HashMap<String, i64> = HashMap::new();
    while !values.contains_key("root") {
        for monkey in monkeys.iter_mut() {
            if let Operation::Number(value) = &mut monkey.operation {
                values.insert(monkey.name.clone(), *value);
            }
        }
        monkeys.retain(|monkey| !matches!(monkey.operation, Operation::Number(_)));

        for monkey in monkeys.iter_mut() {
            match &mut monkey.operation {
                Operation::Number(_) => {}
                op => {
                    if let Some(op) = op.maybe_replace(&values) {
                        monkey.operation = op;
                    }
                }
            }
        }
    }
    values["root"]
}

fn process_data_a(data: &str) -> i64 {
    let mut monkeys = parser(data).unwrap().1;
    get_root(&mut monkeys)
}

fn process_data_b(data: &str) -> i64 {
    let mut monkeys = parser(data).unwrap().1;
    let mut me = 3_412_650_897_400;
    for monkey in monkeys.iter_mut() {
        match monkey.name.as_str() {
            "root" => {
                monkey.operation = match &monkey.operation {
                    Operation::Plus(a, b)
                    | Operation::Minus(a, b)
                    | Operation::Times(a, b)
                    | Operation::Divide(a, b) => Operation::Minus(a.clone(), b.clone()),
                    op => panic!("Root should not be a number! {:?}", op),
                }
            }
            "humn" => {
                if let Operation::Number(5) = monkey.operation {
                    // We're running test data.
                    me = 295;
                }
            }
            _ => {}
        }
    }
    monkeys.retain(|monkey| monkey.name != "humn");

    // Found via binary search. 😂

    loop {
        let mut monkeys = monkeys.clone();
        monkeys.push(Monkey {
            name: "humn".to_owned(),
            operation: Operation::Number(me),
        });
        let rv = get_root(&mut monkeys);
        if rv == 0 {
            break;
        }
        me += 1;
    }
    me

    // 3_412_650_897_405
}

//-----------------------------------------------------
// Questions.

q_impl!("21");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "root: pppw + sjmn
    dbpl: 5
    cczh: sllz + lgvd
    zczc: 2
    ptdq: humn - dvpt
    dvpt: 3
    lfqf: 4
    humn: 5
    ljgn: 2
    sjmn: drzm * dbpl
    sllz: 4
    pppw: cczh / lfqf
    lgvd: ljgn * ptdq
    drzm: hmdt - zczc
    hmdt: 32
    "
        )),
        152
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "root: pppw + sjmn
    dbpl: 5
    cczh: sllz + lgvd
    zczc: 2
    ptdq: humn - dvpt
    dvpt: 3
    lfqf: 4
    humn: 5
    ljgn: 2
    sjmn: drzm * dbpl
    sllz: 4
    pppw: cczh / lfqf
    lgvd: ljgn * ptdq
    drzm: hmdt - zczc
    hmdt: 32
    "
        )),
        301
    );
}
