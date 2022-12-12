//-----------------------------------------------------
// Setup.

static INPUT: &str = include_str!("data/q10.data");

fn process_data_a(data: &str) -> usize {
    let mut rv = 0;
    'line: for line in data.lines() {
        let mut stack = vec![];
        // Do something
        for character in line.chars() {
            match character {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                '<' => stack.push('>'),

                ')' => {
                    if stack.pop() != Some(character) {
                        rv += 3;
                        continue 'line;
                    }
                }
                ']' => {
                    if stack.pop() != Some(character) {
                        rv += 57;
                        continue 'line;
                    }
                }
                '}' => {
                    if stack.pop() != Some(character) {
                        rv += 1197;
                        continue 'line;
                    }
                }
                '>' => {
                    if stack.pop() != Some(character) {
                        rv += 25137;
                        continue 'line;
                    }
                }
                _ => {
                    println!("Unknown character: {}", character);
                }
            }
        }
    }
    rv
}

fn process_data_b(data: &str) -> usize {
    let mut rv = vec![];
    'line: for line in data.lines() {
        let mut stack = vec![];
        let mut line_rv = 0;
        // Do something
        for character in line.chars() {
            match character {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                '<' => stack.push('>'),

                ')' | ']' | '}' | '>' => {
                    if stack.pop() != Some(character) {
                        continue 'line;
                    }
                }
                _ => {
                    println!("Unknown character: {}", character);
                }
            }
        }
        for character in stack.into_iter().rev() {
            line_rv *= 5;
            match character {
                ')' => {
                    line_rv += 1;
                }
                ']' => {
                    line_rv += 2;
                }
                '}' => {
                    line_rv += 3;
                }
                '>' => {
                    line_rv += 4;
                }
                _ => {
                    println!("Unknown character: {}", character);
                }
            }
        }
        rv.push(line_rv);
    }
    rv.sort_unstable();
    rv[rv.len() / 2]
}
//-----------------------------------------------------
// Questions.

q_impl!("10");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_a(indoc!(
            "[({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]
        "
        )),
        26397
    );
}

#[test]
fn b() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        process_data_b(indoc!(
            "[({(<(())[]>[[{[]{<()<>>
        [(()[<>])]({[<{<<[]>>(
        {([(<{}[<>[]}>{[]{[(<()>
        (((({<>}<{<{<>}{[]{[]{}
        [[<[([]))<([[{}[[()]]]
        [{[{({}]{}}([{[{{{}}([]
        {<[[]]>}<{[{[{[]{()[[[]
        [<(<(<(<{}))><([]([]()
        <{([([[(<>()){}]>(<<{{
        <{([{{}}[<[[[<>{}]]]>[]]
        "
        )),
        288957
    );
}
