//-----------------------------------------------------
// Setup.

use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{newline, u32},
    multi::{many1, separated_list1},
};

type Orderings = Vec<(u32, u32)>;
type Updates = Vec<Vec<u32>>;

static INPUT: &str = include_str!("data/q05.data");

fn page_ordering(i: &str) -> IResult<&str, (u32, u32)> {
    let (input, (first, _, second, _)) = (u32, tag("|"), u32, newline).parse(i)?;
    Ok((input, (first, second)))
}

fn page_orderings(i: &str) -> IResult<&str, Orderings> {
    let (input, data) = many1(page_ordering).parse(i)?;
    Ok((input, data))
}

fn update(i: &str) -> IResult<&str, Vec<u32>> {
    let (input, (data, _)) = (separated_list1(tag(","), u32), newline).parse(i)?;
    Ok((input, data))
}

fn updates(i: &str) -> IResult<&str, Updates> {
    let (input, data) = many1(update).parse(i)?;
    Ok((input, data))
}

fn parser(i: &str) -> IResult<&str, (Orderings, Updates)> {
    let (input, (order, _, updates)) = (page_orderings, newline, updates).parse(i)?;
    Ok((input, (order, updates)))
}

fn process_data_a(data: &str) -> u32 {
    let mut rv = 0;
    let (order, updates) = parser(data).unwrap().1;
    for update in updates {
        let mut valid = true;
        'outer: for i in 0..update.len() {
            for j in i + 1..update.len() {
                if !order.contains(&(update[i], update[j])) {
                    valid = false;
                    break 'outer;
                }
            }
        }
        if valid {
            rv += update[update.len() / 2];
        }
    }
    rv
}

fn process_data_b(data: &str) -> u32 {
    let mut rv = 0;
    let (order, mut updates) = parser(data).unwrap().1;
    for update in updates.iter_mut() {
        let mut valid = true;
        for i in 0..update.len() {
            for j in i + 1..update.len() {
                if !order.contains(&(update[i], update[j])) {
                    valid = false;
                    update.swap(i, j);
                }
            }
        }
        if !valid {
            rv += update[update.len() / 2];
        }
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
            "47|53
            97|13
            97|61
            97|47
            75|29
            61|13
            75|53
            29|13
            97|29
            53|29
            61|53
            97|53
            61|29
            47|13
            75|47
            97|75
            47|61
            75|61
            47|29
            75|13
            53|13

            75,47,61,53,29
            97,61,53,29,13
            75,29,13
            75,97,47,61,53
            61,13,29
            97,13,75,29,47
            "
        )),
        143
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "47|53
            97|13
            97|61
            97|47
            75|29
            61|13
            75|53
            29|13
            97|29
            53|29
            61|53
            97|53
            61|29
            47|13
            75|47
            97|75
            47|61
            75|61
            47|29
            75|13
            53|13

            75,47,61,53,29
            97,61,53,29,13
            75,29,13
            75,97,47,61,53
            61,13,29
            97,13,75,29,47
            "
        )),
        123
    );
}
