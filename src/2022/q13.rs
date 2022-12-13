//-----------------------------------------------------
// Setup.

use std::{
    cmp::Ordering,
    fmt::{Display, Write},
};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, terminated, tuple},
    IResult, Parser,
};

static INPUT: &str = include_str!("data/q13.data");

#[derive(Clone, Debug, Eq, PartialEq)]
struct Node {
    data: Option<u8>,
    children: Vec<Node>,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(data) = &self.data {
            f.write_str(&format!("{}", data))?;
        } else {
            f.write_char('[')?;
            for child in &self.children {
                f.write_fmt(format_args!("{}", child))?;
                f.write_char(',')?;
            }
            f.write_char(']')?;
        }
        Ok(())
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn compare_lists(first: &[Node], second: &[Node]) -> Ordering {
    for (a, b) in first.iter().zip(second.iter()) {
        match a.cmp(b) {
            Ordering::Equal => {}
            x => {
                return x;
            }
        }
    }
    first.len().cmp(&second.len())
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.data.is_some(), other.data.is_some()) {
            (true, true) => {
                // We're both numbers, compare us!
                self.data.cmp(&other.data)
            }
            (true, false) => compare_lists(&[self.clone()], &other.children),
            (false, true) => compare_lists(&self.children, &[other.clone()]),
            (false, false) => {
                // We're both lists! compare us!
                compare_lists(&self.children, &other.children)
            }
        }
    }
}

fn array(i: &str) -> IResult<&str, Vec<Node>> {
    let (input, node) = delimited(tag("["), separated_list0(tag(","), node), tag("]"))(i)?;
    Ok((input, node))
}

fn node(i: &str) -> IResult<&str, Node> {
    let (input, node) = alt((
        array.map(|a| Node {
            data: None,
            children: a,
        }),
        complete::u8.map(|v| Node {
            data: Some(v),
            children: vec![],
        }),
    ))(i)?;
    Ok((input, node))
}

fn lines(i: &str) -> IResult<&str, (Node, Node)> {
    let (input, (a, b)) = tuple((terminated(node, line_ending), terminated(node, line_ending)))(i)?;
    Ok((input, (a, b)))
}

fn parser(i: &str) -> IResult<&str, Vec<(Node, Node)>> {
    let (input, list) = separated_list1(line_ending, lines)(i)?;
    Ok((input, list))
}

fn process_data_a(data: &str) -> usize {
    let mut rv = 0;
    let nodes = parser(data).unwrap().1;
    for (index, (a, b)) in nodes.iter().enumerate() {
        let test = a <= b;
        if test {
            rv += index + 1;
        }
    }
    rv
}

fn process_data_b(data: &str) -> usize {
    let mut sentinels = parser("[[2]]\n[[6]]\n\n").unwrap().1;
    let (first, second) = sentinels[0].clone();
    let mut nodes = parser(data).unwrap().1;
    nodes.append(&mut sentinels);
    let nodes: Vec<Node> = nodes
        .iter()
        .flat_map(|(a, b)| vec![a, b])
        .sorted()
        .cloned()
        .collect();
    let first = nodes.iter().position(|x| x == &first);
    let second = nodes.iter().position(|x| x == &second);
    (first.unwrap() + 1) * (second.unwrap() + 1)
}

//-----------------------------------------------------
// Questions.

q_impl!("13");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "[1,1,3,1,1]
    [1,1,5,1,1]
    
    [[1],[2,3,4]]
    [[1],4]
    
    [9]
    [[8,7,6]]
    
    [[4,4],4,4]
    [[4,4],4,4,4]
    
    [7,7,7,7]
    [7,7,7]
    
    []
    [3]
    
    [[[]]]
    [[]]
    
    [1,[2,[3,[4,[5,6,7]]]],8,9]
    [1,[2,[3,[4,[5,6,0]]]],8,9]
    "
        )),
        13
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "[1,1,3,1,1]
    [1,1,5,1,1]
    
    [[1],[2,3,4]]
    [[1],4]
    
    [9]
    [[8,7,6]]
    
    [[4,4],4,4]
    [[4,4],4,4,4]
    
    [7,7,7,7]
    [7,7,7]
    
    []
    [3]
    
    [[[]]]
    [[]]
    
    [1,[2,[3,[4,[5,6,7]]]],8,9]
    [1,[2,[3,[4,[5,6,0]]]],8,9]
    "
        )),
        140
    );
}
