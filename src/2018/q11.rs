//-----------------------------------------------------
// Setup.

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::cmp::max;

static INPUT: i32 = 5535;

fn get_power(x: usize, y: usize, serial: i32) -> i32 {
    let rack_id = (x + 10) as i32;
    let mut power: i32 = rack_id * y as i32 + serial;
    power *= rack_id;
    power /= 100;
    power %= 10;
    power - 5
}

fn get_total_power(cells: &[Vec<i32>], x: usize, y: usize, size: usize) -> i32 {
    let mut rv = 0;
    for row in cells.iter().skip(x).take(size) {
        for cell in row.iter().skip(y).take(size) {
            rv += cell;
        }
    }
    rv
}

fn process_data_a(data: i32) -> String {
    let cells: Vec<Vec<i32>> = (1..301 as usize)
        .into_par_iter()
        .map(|x| {
            (1..301 as usize)
                .into_par_iter()
                .map(|y| get_power(x, y, data))
                .collect()
        })
        .collect();

    let rv = (1..298 as usize)
        .into_par_iter()
        .map(|x| {
            (1..298 as usize)
                .into_par_iter()
                .map(|y| (get_total_power(&cells, x, y, 3), (x + 1, y + 1)))
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    format!("{},{}", (rv.1).0, (rv.1).1)
}

fn process_data_b(data: i32) -> String {
    let cells: Vec<Vec<i32>> = (1..301 as usize)
        .into_par_iter()
        .map(|x| {
            (1..301 as usize)
                .into_par_iter()
                .map(|y| get_power(x, y, data))
                .collect()
        })
        .collect();

    let rv = (1..300 as usize)
        .into_par_iter()
        .map(|x| {
            (1..300 as usize)
                .into_par_iter()
                .map(|y| {
                    let max_size = 301 - max(x, y);
                    (0..max_size)
                        .into_par_iter()
                        .map(|size| (get_total_power(&cells, x, y, size), (x + 1, y + 1), size))
                        .max()
                        .unwrap()
                })
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    format!("{},{},{}", (rv.1).0, (rv.1).1, rv.2)
}

//-----------------------------------------------------
// Questions.

q_impl!("11");

#[test]
fn a() {
    assert_eq!(get_power(3, 5, 8), 4);
    assert_eq!(get_power(122, 79, 57), -5);
    assert_eq!(get_power(217, 196, 39), 0);
    assert_eq!(get_power(101, 153, 71), 4);
    assert_eq!(process_data_a(18), "33,45".to_string());
    assert_eq!(process_data_a(42), "21,61".to_string());
}

#[test]
fn b() {
    assert_eq!(process_data_b(18), "90,269,16".to_string());
    assert_eq!(process_data_b(42), "232,251,12".to_string());
}
