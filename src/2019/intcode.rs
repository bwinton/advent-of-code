use std::collections::VecDeque;

use custom_error::custom_error;

custom_error! {
    #[derive(PartialEq,PartialOrd)]
    pub IntcodeError
        ModeNotFound {flag: i64} = "Mode {flag} not understood.",
        OpcodeNotFound {opcode: i64} = "Opcode {opcode} not understood",
        MachineHalted = "Attempting to run halted machine.",
        MissingValue = "Missing value.",
        MachineNotWaiting = "Machine should be waiting for input, but isn't",
        InvalidPosition {position: usize, len: usize} = "Invalid position {position}, should be less than {len}",
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum State {
    Ready,
    WaitingForInput,
    Halted,
}


#[derive(Copy, Clone, Debug)]
enum Mode {
    Immediate,
    Position,
}

impl Mode {
    fn get_mode(flag: i64) -> Result<Mode, IntcodeError> {
        match flag % 10 {
            0 => Ok(Mode::Position),
            1 => Ok(Mode::Immediate),
            _ => Err(IntcodeError::ModeNotFound {flag}),
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
    fn get_opcode(memory: &[i64], position: usize) -> Result<Opcode, IntcodeError> {
        let code = memory[position];
        match code % 100 {
            1 => {
                let mode = (Mode::get_mode(code / 100)?, Mode::get_mode(code / 1000)?);
                let a = memory[position + 1];
                let b = memory[position + 2];
                let dest = memory[position + 3] as usize;
                Ok(Opcode::Add { mode, a, b, dest })
            }
            2 => {
                let mode = (Mode::get_mode(code / 100)?, Mode::get_mode(code / 1000)?);
                let a = memory[position + 1];
                let b = memory[position + 2];
                let dest = memory[position + 3] as usize;
                Ok(Opcode::Multiply { mode, a, b, dest })
            }
            3 => {
                let dest = memory[position + 1] as usize;
                Ok(Opcode::Input { dest })
            }
            4 => {
                let mode = Mode::get_mode(code / 100)?;
                let src = memory[position + 1];
                Ok(Opcode::Output { mode, src })
            }
            5 => {
                let mode = (Mode::get_mode(code / 100)?, Mode::get_mode(code / 1000)?);
                let test = memory[position + 1];
                let pos = memory[position + 2];
                Ok(Opcode::JumpIfTrue { mode, test, pos })
            }
            6 => {
                let mode = (Mode::get_mode(code / 100)?, Mode::get_mode(code / 1000)?);
                let test = memory[position + 1];
                let pos = memory[position + 2];
                Ok(Opcode::JumpIfFalse { mode, test, pos })
            }
            7 => {
                let mode = (Mode::get_mode(code / 100)?, Mode::get_mode(code / 1000)?);
                let a = memory[position + 1];
                let b = memory[position + 2];
                let dest = memory[position + 3] as usize;
                Ok(Opcode::LessThan { mode, a, b, dest })
            }
            8 => {
                let mode = (Mode::get_mode(code / 100)?, Mode::get_mode(code / 1000)?);
                let a = memory[position + 1];
                let b = memory[position + 2];
                let dest = memory[position + 3] as usize;
                Ok(Opcode::Equals { mode, a, b, dest })
            }
            99 => Ok(Opcode::Halt),
            _ => Err(IntcodeError::OpcodeNotFound {opcode:memory[position]}),
        }
    }
}

fn get_value(mode: Mode, value: i64, memory: &[i64]) -> i64 {
    match mode {
        Mode::Immediate => value,
        Mode::Position => memory[value as usize],
    }
}

#[derive(Debug)]
pub struct Intcode {
    position: usize,
    state: State,
    pub memory: Vec<i64>,
    pub inputs: VecDeque<i64>,
    pub outputs: Vec<i64>
}

impl Intcode {
    pub fn new(memory: Vec<i64>, inputs: Vec<i64>) -> Self {
        Intcode {
            position: 0,
            state: State::Ready,
            memory,
            inputs: VecDeque::from(inputs),
            outputs: vec![]
        }
    }

    pub fn run_tape(&mut self) -> Result<State, IntcodeError> {
        if self.state == State::Halted {
            return Err(IntcodeError::MachineHalted);
        }
        loop {
            let opcode = Opcode::get_opcode(&self.memory, self.position)?;
            match &opcode {
                Opcode::Add { mode, a, b, dest } => {
                    self.memory[*dest] = get_value(mode.0, *a, &self.memory) + get_value(mode.1, *b, &self.memory);
                }
                Opcode::Multiply { mode, a, b, dest } => {
                    self.memory[*dest] = get_value(mode.0, *a, &self.memory) * get_value(mode.1, *b, &self.memory);
                }
                Opcode::Input { dest } => match self.inputs.pop_front() {
                    Some(value) => self.memory[*dest] = value,
                    None => {
                        self.state = State::WaitingForInput;
                        break;
                    },
                },
                Opcode::Output { mode, src } => {
                    self.outputs.push(get_value(*mode, *src, &self.memory));
                }
                Opcode::JumpIfTrue { mode, test, pos } => {
                    if get_value(mode.0, *test, &self.memory) != 0 {
                        self.position = (get_value(mode.1, *pos, &self.memory) as usize) - opcode.get_size();
                    }
                }
                Opcode::JumpIfFalse { mode, test, pos } => {
                    if get_value(mode.0, *test, &self.memory) == 0 {
                        self.position = (get_value(mode.1, *pos, &self.memory) as usize) - opcode.get_size();
                    }
                }
                Opcode::LessThan { mode, a, b, dest } => {
                    self.memory[*dest] = if get_value(mode.0, *a, &self.memory) < get_value(mode.1, *b, &self.memory) {
                        1
                    } else {
                        0
                    }
                }
                Opcode::Equals { mode, a, b, dest } => {
                    self.memory[*dest] = if get_value(mode.0, *a, &self.memory) == get_value(mode.1, *b, &self.memory) {
                        1
                    } else {
                        0
                    }
                }
                Opcode::Halt => {
                    self.state = State::Halted;
                    break;
                }
            }
            self.position += opcode.get_size();
            if self.position >= self.memory.len() {
                return Err(IntcodeError::InvalidPosition{position: self.position, len: self.memory.len()});
            }
        }
        Ok(self.state)
    }
}
