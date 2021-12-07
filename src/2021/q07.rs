//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q07.data");

fn process_data_a(data: &str) -> i64 {
    let line: Vec<i64> = data.trim().split(',').map(|x| x.parse().unwrap()).collect();
    let mut min_cost = i64::MAX;
    for position in 0..*line.iter().max().unwrap() {
        let cost = line.iter().map(|&x| (x - position).abs()).sum();
        if cost < min_cost {
            min_cost = cost;
        } else {
            break;
        }
    }

    min_cost
}

fn process_data_b(data: &str) -> i64 {
    let line: Vec<i64> = data.trim().split(',').map(|x| x.parse().unwrap()).collect();
    let mut min_cost = i64::MAX;
    for position in 0..*line.iter().max().unwrap() {
        let cost = line
            .iter()
            .map(|&x| {
                let d = (x - position).abs();
                d * (d + 1) / 2
            })
            .sum();
        if cost < min_cost {
            min_cost = cost;
        } else {
            break;
        }
    }

    min_cost
}

//-----------------------------------------------------
// Questions.

q_impl!("7");

#[test]
fn a() {
    assert_eq!(process_data_a(indoc!("16,1,2,0,4,2,7,1,2,14")), 37);
}

#[test]
fn b() {
    assert_eq!(process_data_b(indoc!("16,1,2,0,4,2,7,1,2,14")), 168);
}
