//-----------------------------------------------------
// Setup.

use std::collections::{HashMap, HashSet};

static INPUT: &str = include_str!("data/q10.data");

#[derive(Clone, Debug)]
struct State {
    current: usize,
    used: Vec<usize>,
    remaining: HashSet<usize>,
}

fn process_data_a(data: &str) -> usize {
    let mut values: Vec<usize> = data.lines().map(|x| x.parse().unwrap()).collect();
    values.sort_unstable();
    let mut prev = 0;
    let mut ones = 0;
    let mut threes = 1;
    for value in values {
        if value == prev + 1 {
            ones += 1;
        } else if value == prev + 3 {
            threes += 1;
        }
        prev = value;
    }
    ones * threes
}

fn process_data_b(data: &str) -> usize {
    let mut adapters: Vec<i32> = data.lines().map(|x| x.parse().unwrap()).collect();
    adapters.sort_unstable();
    let mut combinations = HashMap::new();
    combinations.insert(&0, 1);
    for adapter in &adapters {
        let value: usize = combinations.get(&(adapter - 1)).unwrap_or(&0)
            + combinations.get(&(adapter - 2)).unwrap_or(&0)
            + combinations.get(&(adapter - 3)).unwrap_or(&0);
        combinations.insert(adapter, value);
    }
    *combinations.get(adapters.last().unwrap() as &i32).unwrap()
}

//-----------------------------------------------------
// Questions.

q_impl!("10");

#[test]
fn a() {
    assert_eq!(
        process_data_a(
            "16
10
15
5
1
11
7
19
6
12
4"
        ),
        35
    );
}

#[test]
fn b() {
    assert_eq!(
        process_data_b(
            "16
10
15
5
1
11
7
19
6
12
4"
        ),
        8
    );
    assert_eq!(
        process_data_b(
            "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3"
        ),
        19208
    );
}
