//-----------------------------------------------------
// Setup.

use aoc::Day;

use std;

static INPUT: &'static str = include_str!("data/q13.data");

fn get_range(max: usize) -> Vec<usize> {
    ((0..max).chain((1..max - 1).rev())).collect()
}

define_iterator!(MultiIter(
    &ranges: Vec<(usize, usize, Vec<usize>)> = Vec::default(),
    &curr: usize = 0
  ) -> Option<Vec<(usize, usize, usize)>> {
  let mut rv = Vec::new();
  for value in ranges {
    rv.push((value.0, value.1, value.2[*curr % value.2.len()]));
  }
  *curr += 1;
  Some(rv)
});

fn get_ranges(data: &str) -> Vec<(usize, usize, Vec<usize>)> {
    let mut ranges = Vec::new();
    for line in data.lines() {
        let temp: Vec<usize> = line.split(": ").map(|x| x.parse().unwrap()).collect();
        ranges.push((temp[0], temp[1], get_range(temp[1])));
    }

    ranges
}

fn process_data_a(data: &str) -> usize {
    let ranges = get_ranges(data);
    let max = &ranges.iter().map(|x| x.0).max().unwrap() + 1;
    let mut rv = 0;
    let scanners = MultiIter {
        ranges: ranges.clone(),
        ..Default::default()
    };
    for tick in scanners.enumerate().take(max) {
        if let Some(scanner) = tick
            .1
            .iter()
            .find(|scanner| scanner.0 == tick.0 && scanner.2 == 0)
        {
            rv += scanner.0 * scanner.1;
        }
    }
    rv
}

fn process_data_b(data: &str) -> usize {
    let ranges = get_ranges(data);
    let max = &ranges.iter().map(|x| x.0).max().unwrap() + 1;
    let mut rv = 0;
    for delay in 0.. {
        let mut caught = false;
        let scanners = MultiIter {
            ranges: ranges.clone(),
            curr: delay,
        };
        for tick in scanners.enumerate().take(max) {
            if tick
                .1
                .iter()
                .any(|scanner| scanner.0 == tick.0 && scanner.2 == 0)
            {
                caught = true;
                break;
            }
        }
        if !caught {
            rv = delay;
            break;
        }
    }
    rv
}

//-----------------------------------------------------
// Questions.

pub struct Q;

impl Day for Q {
    fn number(&self) -> String {
        String::from("13")
    }

    fn a(&self) {
        print!("{}A: ", self.number());
        let result = process_data_a(INPUT);
        println!("Result = {}", result);
    }

    fn b(&self) {
        print!("{}B: ", self.number());
        let result = process_data_b(INPUT);
        println!("Result = {}", result);
    }
}

// MultIter { len: 5, dir: Direction::Up, .. Default::default() }

#[test]
fn a() {
    assert_eq!(
        process_data_a(
            "0: 3
1: 2
4: 4
6: 4",
        ),
        24
    );
}

#[test]
fn b() {
    assert_eq!(
        process_data_b(
            "0: 3
1: 2
4: 4
6: 4",
        ),
        10
    );
}
