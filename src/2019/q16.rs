//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q16.data");

fn run_fft(data: &[char], iterations: usize) -> String {
    let mut curr: Vec<i32> = data
        .iter()
        .map(|&i| i.to_digit(10).unwrap() as i32)
        .collect();
    let mut patterns = vec![];
    for i in 0..curr.len() {
        patterns.push(
            vec![0, 1, 0, -1]
                .into_iter()
                .flat_map(move |x| [x].repeat(i + 1))
                .cycle()
                .skip(1),
        );
    }
    for _ in 0..iterations {
        let mut next: Vec<i32> = vec![];
        for pattern in &patterns {
            let sums: Vec<(_, _)> = curr.iter().zip(pattern.clone()).collect();
            let sum: i32 = sums.into_iter().map(|(x, y)| x * y).sum();
            next.push((sum % 10).abs());
        }
        curr = next;
    }
    curr.into_iter().map(|x| x.to_string()).take(8).collect()
}

fn process_data_a(data: &str) -> String {
    run_fft(&data.chars().collect::<Vec<_>>(), 100)
}

// This only works if your offset is bigger than half the length of your data…
fn run_big_fft(data: &[char], iterations: usize, offset: usize) -> String {
    let mut curr: Vec<i32> = data
        .iter()
        .skip(offset)
        .map(|&i| i.to_digit(10).unwrap() as i32)
        .collect();
    for _ in 0..iterations {
        let mut next: Vec<i32> = vec![];
        let mut sum: i32 = curr.iter().sum();
        for elem in curr {
            next.push((sum % 10).abs());
            sum -= elem;
        }
        curr = next;
    }
    curr.into_iter().map(|x| x.to_string()).take(8).collect()
}

fn process_data_b(data: &str) -> String {
    let offset: usize = data[0..7].parse().unwrap();
    run_big_fft(
        &data.chars().collect::<Vec<_>>().repeat(10_000),
        100,
        offset,
    )
}

//-----------------------------------------------------
// Questions.

q_impl!("16");

#[test]
fn a() {
    assert_eq!(
        run_fft(&"12345678".chars().collect::<Vec<_>>(), 1),
        "48226158".to_owned()
    );
    assert_eq!(
        run_fft(&"12345678".chars().collect::<Vec<_>>(), 2),
        "34040438".to_owned()
    );
    assert_eq!(
        run_fft(&"12345678".chars().collect::<Vec<_>>(), 3),
        "03415518".to_owned()
    );
    assert_eq!(
        run_fft(&"12345678".chars().collect::<Vec<_>>(), 4),
        "01029498".to_owned()
    );
}

#[test]
fn b() {
    assert_eq!(
        process_data_b("03036732577212944063491565474664"),
        "84462026".to_owned()
    );
    assert_eq!(
        process_data_b("02935109699940807407585447034323"),
        "78725270".to_owned()
    );
    assert_eq!(
        process_data_b("03081770884921959731165446850517"),
        "53553731".to_owned()
    );
}
