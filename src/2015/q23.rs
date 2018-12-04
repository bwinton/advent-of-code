//-----------------------------------------------------
// Setup.

use aoc::computer::{
    parse_instructions, Half, Increment, InstructionResult, Jump, JumpEven, JumpOne, Triple, CPU,
};

static INPUT: &'static str = include_str!("data/q23.data");

fn process_data_a_impl(data: &str, reg: char) -> i64 {
    let builders: Vec<fn(s: &str) -> InstructionResult> = vec![
        Half::build,
        Triple::build,
        Increment::build,
        Jump::build,
        JumpEven::build,
        JumpOne::build,
    ];

    let instructions = parse_instructions(data, &builders).unwrap();
    let mut state = CPU::new(hashmap!{ 'a' => 0, 'b' => 0 }, instructions);
    while let Some(new) = state.execute() {
        state = new;
    }
    state.get_register(reg)
}

fn process_data_b_impl(data: &str, reg: char) -> i64 {
    let builders: Vec<fn(s: &str) -> InstructionResult> = vec![
        Half::build,
        Triple::build,
        Increment::build,
        Jump::build,
        JumpEven::build,
        JumpOne::build,
    ];

    let instructions = parse_instructions(data, &builders).unwrap();
    let mut state = CPU::new(hashmap!{ 'a' => 1, 'b' => 0 }, instructions);
    while let Some(new) = state.execute() {
        state = new;
    }
    state.get_register(reg)
}

fn process_data_a(data: &str) -> i64 {
    process_data_a_impl(data, 'b')
}

fn process_data_b(data: &str) -> i64 {
    process_data_b_impl(data, 'b')
}


//-----------------------------------------------------
// Questions.

q_impl!("23");

#[test]
fn a() {
    assert_eq!(
        process_data_a_impl(
            "inc a
jio a, +2
tpl a
inc a",
            'a',
        ),
        2
    );
}

#[test]
fn b() {
    // assert_eq!(process_data_b_impl (""), 0);
}
