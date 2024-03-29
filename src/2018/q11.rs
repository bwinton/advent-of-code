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

fn get_cells(serial: i32) -> Vec<Vec<i32>> {
    let mut cells: Vec<Vec<i32>> = (1..301_usize)
        .into_par_iter()
        .map(|x| {
            (1..301_usize)
                .into_par_iter()
                .map(|y| get_power(x, y, serial))
                .collect()
        })
        .collect();

    for x in 1..300 {
        for y in 1..300 {
            cells[x][y] += cells[x - 1][y] + cells[x][y - 1] - cells[x - 1][y - 1];
        }
    }

    cells
}

fn get_total_power(cells: &[Vec<i32>], x: usize, y: usize, size: usize) -> i32 {
    cells[x][y] + cells[x + size][y + size] - cells[x + size][y] - cells[x][y + size]
}

fn process_data_a(data: i32) -> String {
    let cells = get_cells(data);

    let rv = (0..297_usize)
        .into_par_iter()
        .map(|x| {
            (0..297_usize)
                .into_par_iter()
                .map(|y| (get_total_power(&cells, x, y, 3), (x + 2, y + 2)))
                .max()
                .unwrap()
        })
        .max()
        .unwrap();

    format!("{},{}", (rv.1).0, (rv.1).1)
}

fn process_data_b(data: i32) -> String {
    let cells = get_cells(data);

    let rv = (0..300_usize)
        .into_par_iter()
        .map(|x| {
            (0..300_usize)
                .into_par_iter()
                .map(|y| {
                    let max_size = 300 - max(x, y);
                    (0..max_size)
                        .into_par_iter()
                        .map(|size| (get_total_power(&cells, x, y, size), (x + 2, y + 2), size))
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
    use pretty_assertions::assert_eq;

    assert_eq!(get_power(3, 5, 8), 4);
    assert_eq!(get_power(122, 79, 57), -5);
    assert_eq!(get_power(217, 196, 39), 0);
    assert_eq!(get_power(101, 153, 71), 4);
    assert_eq!(process_data_a(18), "33,45".to_string());
    assert_eq!(process_data_a(42), "21,61".to_string());
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_b(18), "90,269,16".to_string());
    assert_eq!(process_data_b(42), "232,251,12".to_string());
}
