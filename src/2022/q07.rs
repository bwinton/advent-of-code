//-----------------------------------------------------
// Setup.

use std::collections::HashMap;

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending, one_of},
    combinator::map,
    multi::{many1, separated_list1},
    sequence::{preceded, separated_pair},
    IResult,
};

static INPUT: &str = include_str!("data/q07.data");

#[derive(Debug)]
enum Line {
    Cd(String),
    Ls,
    Dir(String),
    File(u128, String),
}

fn ls(i: &str) -> IResult<&str, Line> {
    let (input, command) = map(tag("$ ls"), |_| Line::Ls)(i)?;
    Ok((input, command))
}

fn cd(i: &str) -> IResult<&str, Line> {
    let (input, command) = map(
        preceded(tag("$ cd "), many1(one_of("abcdefghijklmnopqrstuvwxyz./"))),
        |name| Line::Cd(name.iter().cloned().collect()),
    )(i)?;
    Ok((input, command))
}

fn dir(i: &str) -> IResult<&str, Line> {
    let (input, command) = map(preceded(tag("dir "), alpha1), |name: &str| {
        Line::Dir(name.to_owned())
    })(i)?;
    Ok((input, command))
}

fn size(i: &str) -> IResult<&str, Line> {
    let (input, command) = map(
        separated_pair(
            complete::u128,
            tag(" "),
            many1(one_of("abcdefghijklmnopqrstuvwxyz.")),
        ),
        |(size, name)| Line::File(size, name.iter().cloned().collect()),
    )(i)?;
    Ok((input, command))
}

fn output(i: &str) -> IResult<&str, Line> {
    let (input, command) = alt((ls, cd, dir, size))(i)?;
    Ok((input, command))
}

fn parser(i: &str) -> IResult<&str, Vec<Line>> {
    let (input, list) = separated_list1(line_ending, output)(i)?;
    Ok((input, list))
}

fn get_data(data: &str) -> HashMap<String, u128> {
    let mut dirstack = vec![];
    let mut directories: HashMap<String, u128> = HashMap::new();
    directories.insert("".to_owned(), 0);
    let lines = parser(data).unwrap().1;
    for line in lines {
        // println!("{:?}, {:?}", line, dirstack);
        match line {
            Line::Cd(new_dir) => match new_dir.as_str() {
                "/" => {
                    dirstack.clear();
                    dirstack.push("".to_owned())
                }
                ".." => {
                    dirstack.pop();
                }
                d => dirstack.push(d.to_owned()),
            },
            Line::Ls => {}
            Line::Dir(name) => {
                let dirname = dirstack.join("/") + "/" + &name;
                directories.entry(dirname).or_default();
            }
            Line::File(size, _name) => {
                let mut stack = dirstack.clone();
                while !stack.is_empty() {
                    let dirname = stack.join("/");
                    directories.entry(dirname).and_modify(|x| *x += size);
                    stack.pop();
                }
            }
        }
    }
    directories
}

fn process_data_a(data: &str) -> u128 {
    let directories = get_data(data);

    directories
        .iter()
        .filter(|&(_, &size)| size <= 100_000)
        .map(|(_, &size)| size)
        .sum()
}

fn process_data_b(data: &str) -> u128 {
    let directories = get_data(data);

    let total_space = 70_000_000;
    let free_space = total_space - directories[""];
    let needed_space = 30_000_000 - free_space;
    *directories
        .values()
        .sorted()
        .find(|&&x| x > needed_space)
        .unwrap()
}

//-----------------------------------------------------
// Questions.

q_impl!("7");

#[test]
fn a() {
    assert_eq!(
        process_data_a(indoc!(
            "$ cd /
    $ ls
    dir a
    14848514 b.txt
    8504156 c.dat
    dir d
    $ cd a
    $ ls
    dir e
    29116 f
    2557 g
    62596 h.lst
    $ cd e
    $ ls
    584 i
    $ cd ..
    $ cd ..
    $ cd d
    $ ls
    4060174 j
    8033020 d.log
    5626152 d.ext
    7214296 k
    "
        )),
        95_437
    );
}

#[test]
fn b() {
    assert_eq!(
        process_data_b(indoc!(
            "$ cd /
    $ ls
    dir a
    14848514 b.txt
    8504156 c.dat
    dir d
    $ cd a
    $ ls
    dir e
    29116 f
    2557 g
    62596 h.lst
    $ cd e
    $ ls
    584 i
    $ cd ..
    $ cd ..
    $ cd d
    $ ls
    4060174 j
    8033020 d.log
    5626152 d.ext
    7214296 k
    "
        )),
        24_933_642
    );
}
