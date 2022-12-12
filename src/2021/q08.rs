//-----------------------------------------------------
// Setup.

use itertools::Itertools;
use std::collections::{HashMap, HashSet};

static INPUT: &str = include_str!("data/q08.data");

fn process_data_a(data: &str) -> usize {
    let mut rv = 0;
    for line in data.lines() {
        // Do something
        let (_, digits) = line.split_once(" | ").unwrap();
        rv += digits
            .split_ascii_whitespace()
            .map(|x| x.len())
            .filter(|x| [2, 3, 4, 7].contains(x))
            .count();
    }
    rv
}

fn sorted(str: &str) -> String {
    str.chars().sorted().collect()
}

fn find_len_sorted(data: &[&str], len: usize) -> String {
    sorted(data.iter().find(|&&x| x.len() == len).unwrap())
}

fn calculate_mapping(data: &[&str]) -> HashMap<String, u64> {
    let mut rv = HashMap::new();
    let one = find_len_sorted(data, 2);
    let four = find_len_sorted(data, 4);
    let seven = find_len_sorted(data, 3);
    let eight = find_len_sorted(data, 7);
    rv.insert(one.clone(), 1);
    rv.insert(four.clone(), 4);
    rv.insert(seven.clone(), 7);
    rv.insert(eight, 8);

    let one_set: HashSet<char> = one.chars().collect();
    let four_seven_set: HashSet<char> = four.chars().chain(seven.chars()).collect();

    let fives = data.iter().filter(|&&x| x.len() == 5);
    for value in fives {
        // len(5) => { 2, 3, 5 }
        let value_set: HashSet<char> = value.chars().collect();
        match (
            (&value_set - &four_seven_set).len(),
            (&one_set - &value_set).len(),
        ) {
            (1, 0) => {
                rv.insert(sorted(value), 3);
            }
            (1, 1) => {
                rv.insert(sorted(value), 5);
            }
            (2, 1) => {
                rv.insert(sorted(value), 2);
            }
            _ => {
                println!(
                    "Can't happen!!! {}, {}, {}",
                    value,
                    (&value_set - &four_seven_set).len(),
                    (&one_set - &value_set).len()
                )
            }
        }
    }
    let five = rv.iter().find(|&(_, &v)| v == 5).unwrap().0;
    let five_set: HashSet<char> = five.chars().collect();
    let three = rv.iter().find(|&(_, &v)| v == 3).unwrap().0;
    let three_set: HashSet<char> = three.chars().collect();

    let sixes = data.iter().filter(|&&x| x.len() == 6);
    for value in sixes {
        // len(6) => { 0, 6, 9 }
        let value_set: HashSet<char> = value.chars().collect();
        match (
            (&value_set - &five_set).len(),
            (&value_set - &three_set).len(),
        ) {
            (1, 1) => {
                rv.insert(sorted(value), 9);
            }
            (1, 2) => {
                rv.insert(sorted(value), 6);
            }
            (2, 2) => {
                rv.insert(sorted(value), 0);
            }
            _ => {
                println!("Can't happen!!! {}", value)
            }
        }
    }
    rv
}

fn get_value(digits: &[&str], mapping: &HashMap<String, u64>) -> u64 {
    let mut rv = 0;
    for &digit in digits {
        let segments: String = digit.chars().sorted().collect();
        rv += mapping.get(&segments).unwrap();
        rv *= 10;
    }
    rv / 10
}

fn process_data_b(data: &str) -> u64 {
    let mut rv = 0;
    for line in data.lines() {
        // Do something
        let (mapping, digits) = line.split_once(" | ").unwrap();
        let mapping: Vec<_> = mapping.split_ascii_whitespace().collect();
        let mapping = calculate_mapping(&mapping);

        let digits: Vec<_> = digits.split_ascii_whitespace().collect();
        rv += get_value(&digits, &mapping);
    }
    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("8");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"
        )),
        0
    );
    assert_eq!(
        process_data_a(indoc!(
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
    edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
    fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
    fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
    aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
    fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
    dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
    bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
    egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
    gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
    "
        )),
        26
    );
}

#[test]
fn b() {
    // let data: Vec<_> = "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab".split_ascii_whitespace().collect();
    // let mapping = HashMap::from([]);
    // assert_eq!(get_mapping(&data), mapping);
    assert_eq!(
        process_data_b(indoc!(
            "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf"
        )),
        5353
    );
    // assert_eq!(process_data_b(indoc!("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
    // edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
    // fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
    // fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
    // aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
    // fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
    // dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
    // bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
    // egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
    // gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
    // ")), 61229);
}
