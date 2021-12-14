use std::collections::HashMap;

use itertools::Itertools;

//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q14.data");

// For each pair of characters,
//   Get the two new pairs of characters, and set their count to the count of the original pair.
fn step(
    doubles: &mut HashMap<(char, char), usize>,
    map: &HashMap<(char, char), char>,
) -> HashMap<(char, char), usize> {
    let mut rv = HashMap::new();
    for (key, count) in doubles {
        let &new = map.get(key).unwrap();
        let (start, end) = *key;
        {
            *rv.entry((start, new)).or_default() += *count;
        }
        {
            *rv.entry((new, end)).or_default() += *count;
        }
    }
    rv
}

// The count of characters is the sum of the counts of the pairs where it's the first letter
// (plus one for the last letter…)
fn get_counts(doubles: &HashMap<(char, char), usize>, last: char) -> HashMap<char, usize> {
    let mut rv = HashMap::new();

    for ((start, _), count) in doubles {
        *rv.entry(*start).or_default() += *count;
    }
    *rv.entry(last).or_default() += 1;

    rv
}

fn run(data: &str, iterations: usize) -> usize {
    let mut lines = data.lines();
    let input = lines.next().unwrap().to_owned();
    lines.next();
    let mut map: HashMap<(char, char), char> = HashMap::new();
    for line in lines {
        let (start, end) = line.split_once(" -> ").unwrap();
        let key = start.chars().tuple_windows().next().unwrap();
        map.insert(key, end.chars().next().unwrap());
    }

    // Collect a count of each time we see pairs of characters.
    let mut doubles: HashMap<(char, char), usize> = HashMap::new();
    for (first, second) in input.chars().tuple_windows() {
        {
            *doubles.entry((first, second)).or_default() += 1;
        }
    }

    for _ in 0..iterations {
        doubles = step(&mut doubles, &map);
    }

    let counts = get_counts(&doubles, input.chars().last().unwrap());

    let mut high = 0;
    let mut low = usize::MAX;
    for &v in counts.values() {
        if v > high {
            high = v;
        }
        if v < low {
            low = v;
        }
    }
    high - low
}
fn process_data_a(data: &str) -> usize {
    run(data, 10)
}

fn process_data_b(data: &str) -> usize {
    run(data, 40)
}

//-----------------------------------------------------
// Questions.

q_impl!("14");

#[test]
fn a() {
    assert_eq!(
        process_data_a(indoc!(
            "NNCB

    CH -> B
    HH -> N
    CB -> H
    NH -> C
    HB -> C
    HC -> B
    HN -> C
    NN -> C
    BH -> H
    NC -> B
    NB -> B
    BN -> B
    BB -> N
    BC -> B
    CC -> N
    CN -> C"
        )),
        1588
    );
}

#[test]
fn b() {
    assert_eq!(
        process_data_b(indoc!(
            "NNCB

    CH -> B
    HH -> N
    CB -> H
    NH -> C
    HB -> C
    HC -> B
    HN -> C
    NN -> C
    BH -> H
    NC -> B
    NB -> B
    BN -> B
    BB -> N
    BC -> B
    CC -> N
    CN -> C"
        )),
        2_188_189_693_529
    );
}
