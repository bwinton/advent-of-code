use std::collections::VecDeque;

#[derive(Copy, Clone, Debug)]
enum Mode {
    Immediate,
    Position,
}

impl Mode {
    fn get_mode(flag: i64) -> Result<Mode, Vec<i64>> {
        match flag % 10 {
            0 => Ok(Mode::Position),
            1 => Ok(Mode::Immediate),
            _ => Err(vec![flag]),
        }
    }
}

#[derive(Debug)]
enum Opcode {
    Add {
        mode: (Mode, Mode),
        a: i64,
        b: i64,
        dest: usize,
    },
    Multiply {
        mode: (Mode, Mode),
        a: i64,
        b: i64,
        dest: usize,
    },
    Input {
        dest: usize,
    },
    Output {
        mode: Mode,
        src: i64,
    },
    JumpIfTrue {
        mode: (Mode, Mode),
        test: i64,
        pos: i64,
    },
    JumpIfFalse {
        mode: (Mode, Mode),
        test: i64,
        pos: i64,
    },
    LessThan {
        mode: (Mode, Mode),
        a: i64,
        b: i64,
        dest: usize,
    },
    Equals {
        mode: (Mode, Mode),
        a: i64,
        b: i64,
        dest: usize,
    },
    Halt,
}

impl Opcode {
    fn get_size(&self) -> usize {
        match self {
            Opcode::Add { .. } => 4,
            Opcode::Multiply { .. } => 4,
            Opcode::Input { .. } => 2,
            Opcode::Output { .. } => 2,
            Opcode::JumpIfTrue { .. } => 3,
            Opcode::JumpIfFalse { .. } => 3,
            Opcode::LessThan { .. } => 4,
            Opcode::Equals { .. } => 4,
            _ => {
                println!("ERROR!!!");
                999_999
            }
        }
    }
    fn get_opcode(ints: &[i64], position: usize) -> Result<Opcode, Vec<i64>> {
        let code = ints[position];
        match code % 100 {
            1 => {
                let mode = (Mode::get_mode(code / 100)?, Mode::get_mode(code / 1000)?);
                let a = ints[position + 1];
                let b = ints[position + 2];
                let dest = ints[position + 3] as usize;
                Ok(Opcode::Add { mode, a, b, dest })
            }
            2 => {
                let mode = (Mode::get_mode(code / 100)?, Mode::get_mode(code / 1000)?);
                let a = ints[position + 1];
                let b = ints[position + 2];
                let dest = ints[position + 3] as usize;
                Ok(Opcode::Multiply { mode, a, b, dest })
            }
            3 => {
                let dest = ints[position + 1] as usize;
                Ok(Opcode::Input { dest })
            }
            4 => {
                let mode = Mode::get_mode(code / 100)?;
                let src = ints[position + 1];
                Ok(Opcode::Output { mode, src })
            }
            5 => {
                let mode = (Mode::get_mode(code / 100)?, Mode::get_mode(code / 1000)?);
                let test = ints[position + 1];
                let pos = ints[position + 2];
                Ok(Opcode::JumpIfTrue { mode, test, pos })
            }
            6 => {
                let mode = (Mode::get_mode(code / 100)?, Mode::get_mode(code / 1000)?);
                let test = ints[position + 1];
                let pos = ints[position + 2];
                Ok(Opcode::JumpIfFalse { mode, test, pos })
            }
            7 => {
                let mode = (Mode::get_mode(code / 100)?, Mode::get_mode(code / 1000)?);
                let a = ints[position + 1];
                let b = ints[position + 2];
                let dest = ints[position + 3] as usize;
                Ok(Opcode::LessThan { mode, a, b, dest })
            }
            8 => {
                let mode = (Mode::get_mode(code / 100)?, Mode::get_mode(code / 1000)?);
                let a = ints[position + 1];
                let b = ints[position + 2];
                let dest = ints[position + 3] as usize;
                Ok(Opcode::Equals { mode, a, b, dest })
            }
            99 => Ok(Opcode::Halt),
            _ => Err(vec![ints[position]]),
        }
    }
}

fn get_value(mode: Mode, value: i64, ints: &[i64]) -> i64 {
    match mode {
        Mode::Immediate => value,
        Mode::Position => ints[value as usize],
    }
}

pub fn continue_tape(
    position: &mut usize,
    ints: &mut Vec<i64>,
    mut inputs: VecDeque<i64>,
) -> Result<Vec<i64>, Vec<i64>> {
    let mut outputs = vec![];
    loop {
        let opcode = Opcode::get_opcode(&ints, *position)?;
        match &opcode {
            Opcode::Add { mode, a, b, dest } => {
                ints[*dest] = get_value(mode.0, *a, ints) + get_value(mode.1, *b, ints);
            }
            Opcode::Multiply { mode, a, b, dest } => {
                ints[*dest] = get_value(mode.0, *a, ints) * get_value(mode.1, *b, ints);
            }
            Opcode::Input { dest } => match inputs.pop_front() {
                Some(value) => ints[*dest] = value,
                None => return Err(outputs),
            },
            Opcode::Output { mode, src } => {
                outputs.push(get_value(*mode, *src, ints));
            }
            Opcode::JumpIfTrue { mode, test, pos } => {
                if get_value(mode.0, *test, ints) != 0 {
                    *position = (get_value(mode.1, *pos, ints) as usize) - opcode.get_size();
                }
            }
            Opcode::JumpIfFalse { mode, test, pos } => {
                if get_value(mode.0, *test, ints) == 0 {
                    *position = (get_value(mode.1, *pos, ints) as usize) - opcode.get_size();
                }
            }
            Opcode::LessThan { mode, a, b, dest } => {
                ints[*dest] = if get_value(mode.0, *a, ints) < get_value(mode.1, *b, ints) {
                    1
                } else {
                    0
                }
            }
            Opcode::Equals { mode, a, b, dest } => {
                ints[*dest] = if get_value(mode.0, *a, ints) == get_value(mode.1, *b, ints) {
                    1
                } else {
                    0
                }
            }
            Opcode::Halt => return Ok(outputs),
        }
        *position += opcode.get_size();
    }
}

pub fn run_tape(ints: &mut Vec<i64>, inputs: VecDeque<i64>) -> Result<Vec<i64>, Vec<i64>> {
    continue_tape(&mut 0, ints, inputs)
}
