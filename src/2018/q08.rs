//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q08.data");

// #[derive(Clone,Debug)]
// struct Node {
//     child_count: i32,
//     metadata: i32,
//     children: Vec<Box<Node>>,
// }

fn get_metadata_a(numbers: &mut Vec<i32>) -> i32 {
    if numbers.is_empty() {
        return 0;
    }

    let child_count = numbers.pop().unwrap();
    let metadata_count = numbers.pop().unwrap();

    let mut metadata = 0;
    for _ in 0..child_count {
        metadata += get_metadata_a(numbers);
    }
    for _ in 0..metadata_count {
        metadata += numbers.pop().unwrap();
    }
    metadata
}

fn get_metadata_b(numbers: &mut Vec<i32>) -> i32 {
    if numbers.is_empty() {
        return 0;
    }

    let child_count = numbers.pop().unwrap();
    let metadata_count = numbers.pop().unwrap();

    let mut metadata = 0;
    if child_count == 0 {
        for _ in 0..metadata_count {
            metadata += numbers.pop().unwrap();
        }
    } else {
        let mut children = vec![];
        for _ in 0..child_count {
            children.push(get_metadata_b(numbers))
        }
        let mut indices = vec![];
        for _ in 0..metadata_count {
            indices.push(numbers.pop().unwrap());
        }
        for i in indices {
            let index = i - 1;
            if let Some(child) = children.get(index as usize) {
                metadata += child;
            }
        }
    }
    metadata
}

fn process_data_a(data: &str) -> i32 {
    let mut numbers: Vec<i32> = data.split(' ').map(|x| x.parse().unwrap()).collect();
    numbers.reverse();
    get_metadata_a(&mut numbers)
}

fn process_data_b(data: &str) -> i32 {
    let mut numbers: Vec<i32> = data.split(' ').map(|x| x.parse().unwrap()).collect();
    numbers.reverse();
    get_metadata_b(&mut numbers)
}

//-----------------------------------------------------
// Questions.

q_impl!("8");

#[test]
fn a() {
    assert_eq!(process_data_a("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"), 138);
}

#[test]
fn b() {
    assert_eq!(process_data_b("2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2"), 66);
}
