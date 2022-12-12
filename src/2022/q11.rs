//-----------------------------------------------------
// Setup.

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded, tuple},
    IResult,
};

static INPUT: &str = include_str!("data/q11.data");

#[derive(Debug)]
enum Operation {
    Times(u64),
    TimesOld,
    Plus(u64),
}
impl Operation {
    pub(crate) fn execute(&self, item: u64) -> u64 {
        match self {
            Operation::Times(value) => item * value,
            Operation::TimesOld => item * item,
            Operation::Plus(value) => item + value,
        }
    }
}
#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test: u64,
    true_branch: usize,
    false_branch: usize,
    inspected: usize,
}

fn header(i: &str) -> IResult<&str, ()> {
    let (input, _i) = delimited(
        tag("Monkey "),
        complete::u32,
        tuple((tag(":"), line_ending)),
    )(i)?;
    Ok((input, ()))
}

fn items(i: &str) -> IResult<&str, Vec<u64>> {
    let (input, values) = delimited(
        tag("  Starting items: "),
        separated_list1(tag(", "), complete::u64),
        line_ending,
    )(i)?;
    Ok((input, values))
}

fn operation(i: &str) -> IResult<&str, Operation> {
    let (input, (op, value)) = delimited(
        tag("  Operation: new = old "),
        alt((
            tuple((tag("* "), complete::u64)),
            tuple((tag("+ "), complete::u64)),
            map(tag("* old"), |_| ("* old", 0)),
        )),
        line_ending,
    )(i)?;
    let op = match op {
        "+ " => Operation::Plus(value),
        "* " => Operation::Times(value),
        "* old" => Operation::TimesOld,
        _ => panic!("Unknown operation: {} {}", op, value),
    };
    Ok((input, op))
}

fn test(i: &str) -> IResult<&str, u64> {
    let (input, value) = delimited(tag("  Test: divisible by "), complete::u64, line_ending)(i)?;
    Ok((input, value))
}

fn true_branch(i: &str) -> IResult<&str, usize> {
    let (input, value) = delimited(
        tag("    If true: throw to monkey "),
        complete::u64,
        line_ending,
    )(i)?;
    Ok((input, value as usize))
}

fn false_branch(i: &str) -> IResult<&str, usize> {
    let (input, value) = delimited(
        tag("    If false: throw to monkey "),
        complete::u64,
        line_ending,
    )(i)?;
    Ok((input, value as usize))
}

fn monkey(i: &str) -> IResult<&str, Monkey> {
    let (input, (items, operation, test, true_branch, false_branch)) = preceded(
        header,
        tuple((items, operation, test, true_branch, false_branch)),
    )(i)?;
    Ok((
        input,
        Monkey {
            items,
            operation,
            test,
            true_branch,
            false_branch,
            inspected: 0,
        },
    ))
}

fn parser(i: &str) -> IResult<&str, Vec<Monkey>> {
    let (input, list) = separated_list1(line_ending, monkey)(i)?;
    Ok((input, list))
}

fn process_data(data: &str, iterations: i32, extra_worry: bool) -> usize {
    let mut monkeys = parser(data).unwrap().1;
    let mut divisor = 1;
    for monkey in &monkeys {
        divisor *= monkey.test
    }
    for _ in 0..iterations {
        for i in 0..monkeys.len() {
            let mut throws = vec![];
            throws.resize(monkeys.len(), vec![]);

            let monkey = &mut monkeys[i];
            for &item in &monkey.items {
                let mut item = monkey.operation.execute(item);
                if !extra_worry {
                    item /= 3;
                }
                let item = item % divisor;
                let next = if item % monkey.test == 0 {
                    monkey.true_branch
                } else {
                    monkey.false_branch
                };
                throws[next].push(item);
                monkey.inspected += 1;
            }
            monkeys[i].items.clear();
            for i in 0..throws.len() {
                monkeys[i].items.extend_from_slice(&throws[i]);
            }
        }
    }
    monkeys
        .iter()
        .map(|m| m.inspected)
        .sorted()
        .rev()
        .take(2)
        .product()
}

fn process_data_a(data: &str) -> usize {
    process_data(data, 20, false)
}

fn process_data_b(data: &str) -> usize {
    process_data(data, 10_000, true)
}

//-----------------------------------------------------
// Questions.

q_impl!("11");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "Monkey 0:
    Starting items: 79, 98
    Operation: new = old * 19
    Test: divisible by 23
      If true: throw to monkey 2
      If false: throw to monkey 3
  
  Monkey 1:
    Starting items: 54, 65, 75, 74
    Operation: new = old + 6
    Test: divisible by 19
      If true: throw to monkey 2
      If false: throw to monkey 0
  
  Monkey 2:
    Starting items: 79, 60, 97
    Operation: new = old * old
    Test: divisible by 13
      If true: throw to monkey 1
      If false: throw to monkey 3
  
  Monkey 3:
    Starting items: 74
    Operation: new = old + 3
    Test: divisible by 17
      If true: throw to monkey 0
      If false: throw to monkey 1
  "
        )),
        10605
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "Monkey 0:
    Starting items: 79, 98
    Operation: new = old * 19
    Test: divisible by 23
      If true: throw to monkey 2
      If false: throw to monkey 3
  
  Monkey 1:
    Starting items: 54, 65, 75, 74
    Operation: new = old + 6
    Test: divisible by 19
      If true: throw to monkey 2
      If false: throw to monkey 0
  
  Monkey 2:
    Starting items: 79, 60, 97
    Operation: new = old * old
    Test: divisible by 13
      If true: throw to monkey 1
      If false: throw to monkey 3
  
  Monkey 3:
    Starting items: 74
    Operation: new = old + 3
    Test: divisible by 17
      If true: throw to monkey 0
      If false: throw to monkey 1
  "
        )),
        2_713_310_158
    );
}
