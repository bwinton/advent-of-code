//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q08.data");

fn visible(trees: &[Vec<u32>], row: usize, col: usize) -> bool {
    if row == 0 || col == 0 || row == trees.len() - 1 || col == trees[row].len() - 1 {
        return true;
    }

    let test = trees[row][col];

    let mut up = true;
    for item in trees.iter().take(row) {
        if item[col] >= test {
            up = false;
            break;
        }
    }

    let mut down = true;
    for item in trees.iter().skip(row + 1) {
        if item[col] >= test {
            down = false;
            break;
        }
    }

    let mut left = true;
    for x in 0..col {
        if trees[row][x] >= test {
            left = false;
            break;
        }
    }

    let mut right = true;
    for x in col + 1..trees[row].len() {
        if trees[row][x] >= test {
            right = false;
            break;
        }
    }
    up | down | left | right
}

fn scenic(trees: &[Vec<u32>], row: usize, col: usize) -> u32 {
    if row == 0 || col == 0 || row == trees.len() - 1 || col == trees[row].len() - 1 {
        return 0;
    }

    let test = trees[row][col];

    let mut up = 0;
    for y in 1..=row {
        up += 1;
        if trees[row - y][col] >= test {
            break;
        }
    }

    let mut down = 0;
    for item in trees.iter().skip(row + 1) {
        down += 1;
        if item[col] >= test {
            break;
        }
    }

    let mut left = 0;
    for x in 1..=col {
        left += 1;
        if trees[row][col - x] >= test {
            break;
        }
    }

    let mut right = 0;
    for x in col + 1..trees[row].len() {
        right += 1;
        if trees[row][x] >= test {
            break;
        }
    }

    up * down * left * right
}

fn process_data_a(data: &str) -> u32 {
    let mut rv = 0;
    let mut trees = vec![];
    for line in data.lines() {
        // Do something
        let row: Vec<u32> = line.chars().map(|x| x as u32 - '0' as u32).collect();
        trees.push(row);
    }
    for (row, line) in trees.iter().enumerate() {
        for col in 0..line.len() {
            if visible(&trees, row, col) {
                rv += 1;
            };
        }
    }
    rv
}

fn process_data_b(data: &str) -> u32 {
    let mut rv = 0;
    let mut trees = vec![];
    for line in data.lines() {
        // Do something
        let row: Vec<u32> = line.chars().map(|x| x as u32 - '0' as u32).collect();
        trees.push(row);
    }
    for (row, line) in trees.iter().enumerate() {
        for col in 0..line.len() {
            let test = scenic(&trees, row, col);
            if test > rv {
                rv = test;
            };
        }
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
            "30373
    25512
    65332
    33549
    35390
    "
        )),
        21
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "30373
    25512
    65332
    33549
    35390
    "
        )),
        8
    );
}
