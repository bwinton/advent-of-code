#[derive(Copy, Clone, Debug)]
enum Mode {
    Immediate,
    Position,
}

impl Mode {
    fn get_mode(flag: i32) -> Result<Mode, i32> {
        match flag % 10 {
            0 => Ok(Mode::Position),
            1 => Ok(Mode::Immediate),
            _ => Err(flag),
        }
    }
}

#[derive(Debug)]
enum Opcode {
    Add {
        mode: (Mode, Mode),
        a: i32,
        b: i32,
        dest: usize,
    },
    Multiply {
        mode: (Mode, Mode),
        a: i32,
        b: i32,
        dest: usize,
    },
    Input {
        dest: usize,
    },
    Output {
        mode: Mode,
        src: i32,
    },
    JumpIfTrue {
        mode: (Mode, Mode),
        test: i32,
        pos: i32,
    },
    JumpIfFalse {
        mode: (Mode, Mode),
        test: i32,
        pos: i32,
    },
    LessThan {
        mode: (Mode, Mode),
        a: i32,
        b: i32,
        dest: usize,
    },
    Equals {
        mode: (Mode, Mode),
        a: i32,
        b: i32,
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
    fn get_opcode(ints: &[i32], position: usize) -> Result<Opcode, i32> {
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
            _ => Err(ints[position]),
        }
    }
}

fn get_value(mode: Mode, value: i32, ints: &[i32]) -> i32 {
    match mode {
        Mode::Immediate => value,
        Mode::Position => ints[value as usize],
    }
}

pub fn run_tape(ints: &mut Vec<i32>, mut inputs: Vec<i32>) -> Result<Vec<i32>, i32> {
    let mut position: usize = 0;
    let mut outputs = vec![];
    loop {
        let opcode = Opcode::get_opcode(&ints, position)?;
        match &opcode {
            Opcode::Add { mode, a, b, dest } => {
                ints[*dest] = get_value(mode.0, *a, ints) + get_value(mode.1, *b, ints);
            }
            Opcode::Multiply { mode, a, b, dest } => {
                ints[*dest] = get_value(mode.0, *a, ints) * get_value(mode.1, *b, ints);
            }
            Opcode::Input { dest } => match inputs.pop() {
                Some(value) => ints[*dest] = value,
                None => return Err(-1),
            },
            Opcode::Output { mode, src } => {
                outputs.push(get_value(*mode, *src, ints));
            }
            Opcode::JumpIfTrue { mode, test, pos } => {
                if get_value(mode.0, *test, ints) != 0 {
                    position = (get_value(mode.1, *pos, ints) as usize) - opcode.get_size();
                }
            }
            Opcode::JumpIfFalse { mode, test, pos } => {
                if get_value(mode.0, *test, ints) == 0 {
                    position = (get_value(mode.1, *pos, ints) as usize) - opcode.get_size();
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
        position += opcode.get_size();
    }
}
