//-----------------------------------------------------
// Setup.

use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, i128, line_ending, space1},
    combinator::complete,
    multi::many1,
    sequence::{terminated, tuple},
    IResult,
};

static INPUT: &str = include_str!("data/q24.data");

#[derive(Debug)]
enum Instruction {
    Input(char),
    AddLiteral(char, i128),
    AddRegister(char, char),
    MulLiteral(char, i128),
    MulRegister(char, char),
    DivLiteral(char, i128),
    DivRegister(char, char),
    ModLiteral(char, i128),
    ModRegister(char, char),
    EqlLiteral(char, i128),
    EqlRegister(char, char),
}

fn inp(i: &str) -> IResult<&str, Instruction> {
    let (input, (_, result)) = tuple((tag("inp "), anychar))(i)?;
    Ok((input, Instruction::Input(result)))
}

fn add_literal(i: &str) -> IResult<&str, Instruction> {
    let (input, (_, a, _, b)) = tuple((tag("add "), anychar, space1, i128))(i)?;
    Ok((input, Instruction::AddLiteral(a, b)))
}

fn add_register(i: &str) -> IResult<&str, Instruction> {
    let (input, (_, a, _, b)) = tuple((tag("add "), anychar, space1, anychar))(i)?;
    Ok((input, Instruction::AddRegister(a, b)))
}

fn mul_literal(i: &str) -> IResult<&str, Instruction> {
    let (input, (_, a, _, b)) = tuple((tag("mul "), anychar, space1, i128))(i)?;
    Ok((input, Instruction::MulLiteral(a, b)))
}

fn mul_register(i: &str) -> IResult<&str, Instruction> {
    let (input, (_, a, _, b)) = tuple((tag("mul "), anychar, space1, anychar))(i)?;
    Ok((input, Instruction::MulRegister(a, b)))
}

fn div_literal(i: &str) -> IResult<&str, Instruction> {
    let (input, (_, a, _, b)) = tuple((tag("div "), anychar, space1, i128))(i)?;
    Ok((input, Instruction::DivLiteral(a, b)))
}

fn div_register(i: &str) -> IResult<&str, Instruction> {
    let (input, (_, a, _, b)) = tuple((tag("div "), anychar, space1, anychar))(i)?;
    Ok((input, Instruction::DivRegister(a, b)))
}

fn mod_literal(i: &str) -> IResult<&str, Instruction> {
    let (input, (_, a, _, b)) = tuple((tag("mod "), anychar, space1, i128))(i)?;
    Ok((input, Instruction::ModLiteral(a, b)))
}

fn mod_register(i: &str) -> IResult<&str, Instruction> {
    let (input, (_, a, _, b)) = tuple((tag("mod "), anychar, space1, anychar))(i)?;
    Ok((input, Instruction::ModRegister(a, b)))
}

fn eql_literal(i: &str) -> IResult<&str, Instruction> {
    let (input, (_, a, _, b)) = tuple((tag("eql "), anychar, space1, i128))(i)?;
    Ok((input, Instruction::EqlLiteral(a, b)))
}

fn eql_register(i: &str) -> IResult<&str, Instruction> {
    let (input, (_, a, _, b)) = tuple((tag("eql "), anychar, space1, anychar))(i)?;
    Ok((input, Instruction::EqlRegister(a, b)))
}

fn instruction(i: &str) -> IResult<&str, Instruction> {
    terminated(
        alt((
            inp,
            add_literal,
            add_register,
            mul_literal,
            mul_register,
            div_literal,
            div_register,
            mod_literal,
            mod_register,
            eql_literal,
            eql_register,
        )),
        line_ending,
    )(i)
}

fn parser(i: &str) -> IResult<&str, Vec<Instruction>> {
    complete(many1(instruction))(i)
}

#[allow(dead_code)]
struct Computer<'a> {
    instructions: &'a [Instruction],
    registers: [i128; 4],
}

#[allow(dead_code)]
impl<'a> Computer<'a> {
    fn new(instructions: &'a [Instruction]) -> Self {
        let registers = [0; 4];
        Computer {
            instructions,
            registers,
        }
    }

    fn get_register_index(c: char) -> usize {
        match c {
            'w' => 0,
            'x' => 1,
            'y' => 2,
            'z' => 3,
            _ => panic!("Unknown Register! {}", c),
        }
    }

    fn run(&mut self, input: &[i128]) {
        let mut input = input.iter().collect::<Vec<_>>();
        for instruction in self.instructions {
            match *instruction {
                Instruction::Input(a) => {
                    let &value = input.pop().unwrap();
                    let index = Computer::get_register_index(a);
                    self.registers[index] = value;
                }
                Instruction::AddLiteral(a, value) => {
                    let index = Computer::get_register_index(a);
                    self.registers[index] += value;
                }
                Instruction::AddRegister(a, b) => {
                    let index = Computer::get_register_index(a);
                    let value = Computer::get_register_index(b);
                    self.registers[index] += self.registers[value];
                }
                Instruction::MulLiteral(a, value) => {
                    let index = Computer::get_register_index(a);
                    self.registers[index] *= value;
                }
                Instruction::MulRegister(a, b) => {
                    let index = Computer::get_register_index(a);
                    let value = Computer::get_register_index(b);
                    self.registers[index] *= self.registers[value];
                }
                Instruction::DivLiteral(a, value) => {
                    let index = Computer::get_register_index(a);
                    self.registers[index] /= value;
                }
                Instruction::DivRegister(a, b) => {
                    let index = Computer::get_register_index(a);
                    let value = Computer::get_register_index(b);
                    self.registers[index] /= self.registers[value];
                }
                Instruction::ModLiteral(a, value) => {
                    let index = Computer::get_register_index(a);
                    self.registers[index] %= value;
                }
                Instruction::ModRegister(a, b) => {
                    let index = Computer::get_register_index(a);
                    let value = Computer::get_register_index(b);
                    self.registers[index] %= self.registers[value];
                }
                Instruction::EqlLiteral(a, value) => {
                    let index = Computer::get_register_index(a);
                    self.registers[index] = if self.registers[index] == value { 1 } else { 0 };
                }
                Instruction::EqlRegister(a, b) => {
                    let index = Computer::get_register_index(a);
                    let value = Computer::get_register_index(b);
                    self.registers[index] = if self.registers[index] == self.registers[value] {
                        1
                    } else {
                        0
                    };
                }
            }
        }
    }

    fn reset(&mut self) {
        self.registers = [0; 4];
    }
}

#[allow(dead_code)]
fn get_digits(curr: i64) -> [i128; 14] {
    let mut curr = curr;
    let mut rv = [0; 14];
    let nine: i64 = 9;
    for i in 0..14u32 {
        rv[i as usize] = (curr / nine.pow(13 - i) + 1) as i128;
        curr %= nine.pow(13 - i);
    }
    rv
}

fn get_constants(instructions: &[Instruction]) -> Vec<(i128, i128, i128)> {
    let mut constants = vec![];
    for i in 0..14 {
        let offset = i * 18;
        let a = match &instructions[4 + offset] {
            Instruction::DivLiteral(_, a) => a,
            x => panic!("Invalid Instruction! {} {:?} != DivLiteral", 4 + offset, x),
        };
        let b = match &instructions[5 + offset] {
            Instruction::AddLiteral(_, b) => b,
            x => panic!("Invalid Instruction! {} {:?} != AddLiteral", 5 + offset, x),
        };
        let c = match &instructions[15 + offset] {
            Instruction::AddLiteral(_, c) => c,
            x => panic!("Invalid Instruction! {} {:?} != AddLiteral", 15 + offset, x),
        };
        constants.push((*a, *b, *c));
    }
    constants
}

fn build_deps(
    constants: &[(i128, i128, i128)],
    i: usize,
    zl: Vec<i128>,
) -> Vec<HashMap<i128, Vec<(i128, i128)>>> {
    let (max_a, max_b, max_c) = constants[i];
    let mut solutions: HashMap<i128, Vec<(i128, i128)>> = HashMap::new();
    for w in 0..9 {
        for z in &zl {
            for a in 0..max_a {
                let pz = z * max_a + a;
                if pz % 26 + max_b == w && pz / max_a == *z {
                    solutions.entry(pz).or_default().push((w, *z));
                }
                let pz = (z - w - max_c) / 26 * max_a + a;
                if pz % 26 + max_b != w && pz / max_a * 26 + w + max_c == *z {
                    solutions.entry(pz).or_default().push((w, *z));
                }
            }
        }
    }

    let mut rv = if i > 0 {
        build_deps(constants, i - 1, solutions.keys().cloned().collect())
    } else {
        vec![]
    };

    rv.push(solutions);
    rv
}

fn solve(
    i: usize,
    z: i128,
    solutions: &str,
    levels: &[HashMap<i128, Vec<(i128, i128)>>],
    largest: bool,
) -> Option<i64> {
    if i == 14 {
        return Some(solutions.parse().unwrap());
    }

    let mut level = levels[i].get(&z)?.clone();
    level.sort_unstable();
    if largest {
        level.reverse();
    }
    for (w, nz) in level {
        let ts = solutions.to_string() + &(w + 1).to_string();
        if let Some(result) = solve(i + 1, nz, &ts, levels, largest) {
            return Some(result);
        }
    }

    None
}

fn process_data_a(data: &str) -> i64 {
    let instructions = parser(data).unwrap().1;
    let constants = get_constants(&instructions);
    let levels = build_deps(&constants, 13, vec![0]);

    solve(0, 0, "", &levels, true).unwrap()
}

fn process_data_b(data: &str) -> i64 {
    let instructions = parser(data).unwrap().1;
    let constants = get_constants(&instructions);
    let levels = build_deps(&constants, 13, vec![0]);

    solve(0, 0, "", &levels, false).unwrap()
}

//-----------------------------------------------------
// Questions.

q_impl!("24");

#[test]
fn a() {
    use pretty_assertions::assert_eq;

    assert_eq!(
        parser(indoc!(
            "inp x
    mul x -1
    "
        ))
        .unwrap()
        .1
        .len(),
        2
    );
    assert_eq!(
        parser(indoc!(
            "inp z
    inp x
    mul z 3
    eql z x
    "
        ))
        .unwrap()
        .1
        .len(),
        4
    );
    assert_eq!(
        parser(indoc!(
            "inp w
    add z w
    mod z 2
    div w 2
    add y w
    mod y 2
    div w 2
    add x w
    mod x 2
    div w 2
    mod w 2
    "
        ))
        .unwrap()
        .1
        .len(),
        11
    );
}

#[test]
fn b() {
    // assert_eq!(process_data_b(indoc!("")), 0);
}
