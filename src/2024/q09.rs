//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q09.data");

const ZERO: usize = '0' as usize;

fn parse(data: &str) -> Vec<Option<usize>> {
    let mut id: usize = 0;
    let mut file = true;
    let mut memory: Vec<Option<_>> = vec![];
    for cell in data.chars() {
        if !cell.is_ascii_digit() {
            continue;
        }
        let size = cell as usize - ZERO;
        if file {
            memory.extend(&[Some(id)].repeat(size));
            id += 1;
        } else {
            memory.extend(&[None].repeat(size));
        }
        file = !file;
    }
    memory
}

fn parse2(data: &str) -> Vec<(usize, Option<usize>)> {
    let mut id: usize = 0;
    let mut file = true;
    let mut memory: Vec<(usize, Option<usize>)> = vec![];
    for cell in data.chars() {
        if !cell.is_ascii_digit() {
            continue;
        }
        let size = cell as usize - ZERO;
        if file {
            if size != 0 {
                memory.push((size, Some(id)));
            }
            id += 1;
        } else if size != 0 {
            memory.push((size, None));
        }
        file = !file;
    }
    memory
}

fn calculate(memory: &[Option<usize>]) -> usize {
    let mut rv = 0;
    for (i, value) in memory.iter().enumerate() {
        rv += i * value.unwrap_or_default();
    }
    rv
}

#[allow(dead_code)]
fn print_mem(memory: &[(usize, Option<usize>)]) {
    for &(size, id) in memory.iter() {
        for _ in 0..size {
            print!("{}", id.map(|x| x.to_string()).unwrap_or(".".to_owned()));
        }
    }
    println!();
}

fn consolidate_free_space(memory: &mut Vec<(usize, Option<usize>)>, right: usize) -> usize {
    let mut rv = 0;
    if right < memory.len() - 1 && memory[right + 1].1.is_none() {
        memory[right].0 += memory[right + 1].0;
        memory.remove(right + 1);
        rv += 1;
    }
    if memory[right - 1].1.is_none() {
        memory[right - 1].0 += memory[right].0;
        memory.remove(right);
        rv += 1;
    }
    rv
}

fn process_data_a(data: &str) -> usize {
    let mut memory = parse(data);

    let mut left = 0;
    let mut right = memory.len() - 1;
    loop {
        while memory[left].is_some() {
            left += 1;
        }
        while right > 0 && memory[right].is_none() {
            right -= 1;
        }
        if left > right {
            // We're done!
            break;
        }
        memory.swap(left, right);
    }
    calculate(&memory)
}

fn process_data_b(data: &str) -> usize {
    let mut memory = parse2(data);

    let mut left = 0;
    let mut right = memory.len() - 1;
    loop {
        while memory[left].1.is_some() {
            left += 1;
        }
        while right > 0 && memory[right].1.is_none() {
            right -= 1;
        }
        if left > right {
            // We're done!
            break;
        }

        // Find the next free block, starting here, that can contain the file.
        let mut next = left;
        while (memory[next].0 < memory[right].0 || memory[next].1.is_some()) && next < right {
            next += 1;
        }
        if next == right {
            // Couldn't find a place, move on.
            right -= 1;
            continue;
        }

        if memory[next].0 > memory[right].0 {
            memory[next].0 -= memory[right].0;
            memory.insert(next, (memory[right].0, None));
            right += 1;
        }
        memory.swap(next, right);

        right -= consolidate_free_space(&mut memory, right);
    }
    // print_mem(&memory);
    let mut calc = vec![];
    for (size, id) in memory {
        calc.extend(&[id].repeat(size));
    }
    calculate(&calc)
}

//-----------------------------------------------------
// Questions.

q_impl!("9");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_a(indoc!("2333133121414131402")), 1928);
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(process_data_b(indoc!("2333133121414131402")), 2858);
}
