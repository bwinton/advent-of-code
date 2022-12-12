//-----------------------------------------------------
// Setup.

use itertools::Itertools;

static INPUT: &str = include_str!("data/q17.data");

fn process_data_a_impl(data: &str, amount: u32) -> usize {
    let containers: Vec<u32> = data.lines().map(|x| x.parse().unwrap()).collect();
    let mut count = 0;
    for len in 1..=containers.len() {
        for permutation in containers.iter().combinations(len) {
            // print!("{:?} = ", &permutation);
            let sum: u32 = permutation.into_iter().sum();
            // println!("{}", sum);
            if sum == amount {
                count += 1;
            }
        }
    }
    count
}

fn process_data_b_impl(data: &str, amount: u32) -> usize {
    let containers: Vec<u32> = data.lines().map(|x| x.parse().unwrap()).collect();
    let mut smallest: Vec<Vec<_>> = Vec::new();
    for len in 1..=containers.len() {
        for permutation in containers.iter().combinations(len) {
            let sum: u32 = permutation.iter().cloned().sum();
            if sum == amount {
                if !smallest.is_empty() && permutation.len() < smallest[0].len() {
                    smallest.clear();
                }
                if smallest.is_empty() || permutation.len() == smallest[0].len() {
                    smallest.push(permutation.clone());
                }
            }
        }
    }
    smallest.len()
}

fn process_data_a(data: &str) -> usize {
    process_data_a_impl(data, 150)
}

fn process_data_b(data: &str) -> usize {
    process_data_b_impl(data, 150)
}

//-----------------------------------------------------
// Questions.

q_impl!("17");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a_impl(
            "20
15
10
5
5",
            25,
        ),
        4
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b_impl(
            "20
15
10
5
5",
            25,
        ),
        3
    );
}
