//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q04.data");

fn find_xmas(x: usize, y: usize, grid: &[Vec<char>]) -> usize {
    let mut rv = 0;
    let width = grid[0].len();
    let height = grid.len();
    // forward
    if x + 4 <= width && grid[y][x + 1] == 'M' && grid[y][x + 2] == 'A' && grid[y][x + 3] == 'S' {
        rv += 1;
    }

    // backward
    if x >= 3 && grid[y][x - 1] == 'M' && grid[y][x - 2] == 'A' && grid[y][x - 3] == 'S' {
        rv += 1;
    }

    // down
    if y + 4 <= height && grid[y + 1][x] == 'M' && grid[y + 2][x] == 'A' && grid[y + 3][x] == 'S' {
        rv += 1;
    }

    // up
    if y >= 3 && grid[y - 1][x] == 'M' && grid[y - 2][x] == 'A' && grid[y - 3][x] == 'S' {
        rv += 1;
    }

    // forward-down
    if x + 4 <= width
        && y + 4 <= height
        && grid[y + 1][x + 1] == 'M'
        && grid[y + 2][x + 2] == 'A'
        && grid[y + 3][x + 3] == 'S'
    {
        rv += 1;
    }

    // forward-up
    if x + 4 <= width
        && y >= 3
        && grid[y - 1][x + 1] == 'M'
        && grid[y - 2][x + 2] == 'A'
        && grid[y - 3][x + 3] == 'S'
    {
        rv += 1;
    }

    // backward-down
    if x >= 3
        && y + 4 <= height
        && grid[y + 1][x - 1] == 'M'
        && grid[y + 2][x - 2] == 'A'
        && grid[y + 3][x - 3] == 'S'
    {
        rv += 1;
    }

    // backward-up
    if x >= 3
        && y >= 3
        && grid[y - 1][x - 1] == 'M'
        && grid[y - 2][x - 2] == 'A'
        && grid[y - 3][x - 3] == 'S'
    {
        rv += 1;
    }

    // 2364 is too low.
    rv
}

fn find_x_mas(x: usize, y: usize, grid: &[Vec<char>]) -> usize {
    let width = grid[0].len();
    let height = grid.len();

    if x + 1 >= width || x < 1 || y + 1 >= height || y < 1 {
        return 0;
    }
    // forward-down
    // forward-up
    // backward-down
    // backward-up
    let fd = grid[y + 1][x + 1];
    let fu = grid[y - 1][x + 1];
    let bd = grid[y + 1][x - 1];
    let bu = grid[y - 1][x - 1];

    let chars = [fd, fu, bd, bu];
    let mut m_count = 0;
    let mut s_count = 0;
    for c in chars {
        match c {
            'M' => m_count += 1,
            'S' => s_count += 1,
            _ => {}
        }
    }
    if m_count != 2 || s_count != 2 {
        return 0;
    }
    if fd == bu {
        return 0;
    }
    // 2364 is too low.
    1
}
fn process_data_a(data: &str) -> usize {
    let mut rv = 0;
    let mut grid = vec![];
    for line in data.lines() {
        grid.push(line.chars().collect::<Vec<_>>());
    }
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell == &'X' {
                rv += find_xmas(x, y, &grid);
            }
        }
    }

    rv
}

fn process_data_b(data: &str) -> usize {
    let mut rv = 0;
    let mut grid = vec![];
    for line in data.lines() {
        grid.push(line.chars().collect::<Vec<_>>());
    }
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if cell == &'A' {
                rv += find_x_mas(x, y, &grid);
            }
        }
    }

    rv
}

//-----------------------------------------------------
// Questions.

q_impl!("4");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "
    MMMSXXMASM
    MSAMXMSMSA
    AMXSXMAAMM
    MSAMASMSMX
    XMASAMXAMM
    XXAMMXXAMA
    SMSMSASXSS
    SAXAMASAAA
    MAMMMXMMMM
    MXMXAXMASX
    "
        )),
        18
    );

    assert_eq!(
        process_data_a(indoc!(
            "
    XMAS
    SAMX
    XMAS
    SAMX
    "
        )),
        4
    );

    assert_eq!(
        process_data_a(indoc!(
            "
    XSXS
    MAMA
    AMAM
    SXSX
    "
        )),
        4
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "MMMSXXMASM
    MSAMXMSMSA
    AMXSXMAAMM
    MSAMASMSMX
    XMASAMXAMM
    XXAMMXXAMA
    SMSMSASXSS
    SAXAMASAAA
    MAMMMXMMMM
    MXMXAXMASX"
        )),
        9
    );
}
