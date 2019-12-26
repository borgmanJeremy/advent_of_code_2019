use std::io;

#[derive(Debug, PartialEq)]
pub enum CpuState {
    Halt,
    Run,
    PendingInput,
}

#[derive(Debug, PartialEq, Eq)]
enum ParameterMode {
    POSITION,
    IMMEDIATE,
    RELATIVE,
}

#[derive(Debug)]
pub enum IOMode {
    TERMINAL,
    STACK,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Instruction {
    mode_1: ParameterMode,
    mode_2: ParameterMode,
    mode_3: ParameterMode,
    opcode: i128,
}

#[derive(Debug)]
pub struct Cpu {
    pc: usize,
    relative_offset: i128,
    pub state: CpuState,
    pub input_mode: IOMode,
    pub output_mode: IOMode,
    pub input_stack: Vec<i128>,
    pub output_stack: Vec<i128>,
    pub memory: Vec<i128>,
}

fn decompose_int_to_vec(int: i128) -> Vec<i128> {
    if int > 99999 {
        panic!("only supports five digits");
    }

    let mut breakdown = int;
    let ten_thousand = breakdown / 10000;
    breakdown -= ten_thousand * 10000;

    let thousand = breakdown / 1000;
    breakdown -= thousand * 1000;
    let hundred = breakdown / 100;
    breakdown -= hundred * 100;

    let ten = breakdown / 10;
    breakdown -= ten * 10;

    let one = breakdown;

    let decomposed = vec![one, ten, hundred, thousand, ten_thousand];

    // println!("{:?}", decomposed);
    return decomposed;
}
fn decode_parameter(instruction: i128) -> Result<ParameterMode, &'static str> {
    match instruction {
        0 => Ok(ParameterMode::POSITION),
        1 => Ok(ParameterMode::IMMEDIATE),
        2 => Ok(ParameterMode::RELATIVE),
        _ => Err("Parameter Not Found"),
    }
}
fn decode_instruction(instruction: i128) -> Instruction {
    let instruction_break = decompose_int_to_vec(instruction);

    let instruction_val = instruction_break[0] + instruction_break[1] * 10;
    let param_1 = match decode_parameter(instruction_break[2]) {
        Ok(x) => x,
        Err(error_msg) => panic!(error_msg),
    };
    let param_2 = match decode_parameter(instruction_break[3]) {
        Ok(x) => x,
        Err(error_msg) => panic!(error_msg),
    };

    let param_3 = match decode_parameter(instruction_break[4]) {
        Ok(x) => x,
        Err(error_msg) => panic!(error_msg),
    };
    return Instruction {
        mode_1: param_1,
        mode_2: param_2,
        mode_3: param_3,
        opcode: instruction_val,
    };
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            pc: 0,
            relative_offset: 0,
            state: CpuState::Run,
            input_mode: IOMode::TERMINAL,
            output_mode: IOMode::TERMINAL,
            input_stack: Vec::new(),
            output_stack: Vec::new(),
            memory: Vec::new(),
        }
    }

    fn get_arg(&self, address: i128, mode: ParameterMode) -> i128 {
        match mode {
            ParameterMode::POSITION => {
                let idx = address as usize;
                self.memory[idx]
            }
            ParameterMode::IMMEDIATE => address,
            ParameterMode::RELATIVE => {
                let idx = address as i128 + self.relative_offset as i128;
                self.memory[idx as usize]
            }
        }
    }

    fn write_arg(&mut self, address: i128, value: i128, mode: ParameterMode) {
        match mode {
            ParameterMode::POSITION => {
                let idx = address as usize;
                self.memory[idx] = value;
            }
            ParameterMode::IMMEDIATE => panic!("Immediate not allowed for writes"),
            ParameterMode::RELATIVE => {
                let idx = address as i128 + self.relative_offset as i128;
                self.memory[idx as usize] = value;
            }
        }
    }
    pub fn run_program(&mut self) {
        loop {
            match self.state {
                CpuState::Run => {
                    self.exec();
                }
                CpuState::Halt => {
                    break;
                }
                CpuState::PendingInput => {
                    self.exec();
                    match self.state {
                        CpuState::Run => {}
                        _ => {
                            break;
                        }
                    }
                }
            }
        }
    }

    pub fn exec(&mut self) {
        let instruction = decode_instruction(self.memory[self.pc]);

        match instruction.opcode {
            1 => {
                let val = self.get_arg(self.memory[self.pc + 1], instruction.mode_1)
                    + self.get_arg(self.memory[self.pc + 2], instruction.mode_2);

                self.write_arg(self.memory[self.pc + 3], val, instruction.mode_3);

                self.pc += 4;
            }
            2 => {
                let val = self.get_arg(self.memory[self.pc + 1], instruction.mode_1)
                    * self.get_arg(self.memory[self.pc + 2], instruction.mode_2);

                self.write_arg(self.memory[self.pc + 3], val, instruction.mode_3);

                self.pc += 4;
            }
            3 => match self.input_mode {
                IOMode::TERMINAL => {
                    println!("ID of System to Test:");
                    let mut input = String::new();
                    match io::stdin().read_line(&mut input) {
                        Ok(_n) => {
                            let num: i128 = input.trim().parse().unwrap();
                            self.write_arg(self.memory[self.pc + 1], num, instruction.mode_1);
                            self.pc += 2;
                        }
                        Err(error) => println!("error: {}", error),
                    }
                }
                IOMode::STACK => {
                    if self.input_stack.len() > 0 {
                        self.write_arg(
                            self.memory[self.pc + 1],
                            *self.input_stack.last().unwrap(),
                            instruction.mode_1,
                        );
                        self.input_stack.pop();
                        self.state = CpuState::Run;
                        self.pc += 2;
                    } else {
                        self.state = CpuState::PendingInput;
                    }
                }
            },
            4 => {
                match self.output_mode {
                    IOMode::TERMINAL => {
                        let output_val = self.get_arg(self.memory[self.pc + 1], instruction.mode_1);
                        println!("Program Output: {}", output_val);
                    }
                    IOMode::STACK => {
                        let output_val = self.get_arg(self.memory[self.pc + 1], instruction.mode_1);
                        self.output_stack.push(output_val);
                    }
                }
                self.pc += 2;
            }

            5 => {
                let arg = self.get_arg(self.memory[self.pc + 1], instruction.mode_1);
                if arg != 0 {
                    self.pc = self.get_arg(self.memory[self.pc + 2], instruction.mode_2) as usize;
                } else {
                    self.pc += 3;
                }
            }
            6 => {
                let arg = self.get_arg(self.memory[self.pc + 1], instruction.mode_1);
                if arg == 0 {
                    self.pc = self.get_arg(self.memory[self.pc + 2], instruction.mode_2) as usize;
                } else {
                    self.pc += 3;
                }
            }

            7 => {
                let arg_1 = self.get_arg(self.memory[self.pc + 1], instruction.mode_1);
                let arg_2 = self.get_arg(self.memory[self.pc + 2], instruction.mode_2);

                if arg_1 < arg_2 {
                    self.write_arg(self.memory[self.pc + 3], 1, instruction.mode_3);
                } else {
                    self.write_arg(self.memory[self.pc + 3], 0, instruction.mode_3);
                }

                self.pc += 4;
            }

            8 => {
                let arg_1 = self.get_arg(self.memory[self.pc + 1], instruction.mode_1);
                let arg_2 = self.get_arg(self.memory[self.pc + 2], instruction.mode_2);

                if arg_1 == arg_2 {
                    self.write_arg(self.memory[self.pc + 3], 1, instruction.mode_3);
                } else {
                    self.write_arg(self.memory[self.pc + 3], 0, instruction.mode_3);
                }

                self.pc += 4;
            }
            9 => {
                let arg_1 = self.get_arg(self.memory[self.pc + 1], instruction.mode_1);
                self.relative_offset += arg_1;
                self.pc += 2;
            }
            99 => {
                self.state = CpuState::Halt;
            }
            _ => {
                panic!("Illegal Op Code");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]

    fn test_decompose_int_to_vec() {
        assert_eq!(decompose_int_to_vec(12345), vec![5, 4, 3, 2, 1]);
        assert_eq!(decompose_int_to_vec(25), vec![5, 2, 0, 0, 0]);
    }

    #[test]
    fn test_decode_parameter() {
        assert_eq!(decode_parameter(0).unwrap(), ParameterMode::POSITION);
        assert_eq!(decode_parameter(1).unwrap(), ParameterMode::IMMEDIATE);
    }

    #[test]
    fn test_decode_instruction() {
        assert_eq!(
            decode_instruction(1002),
            Instruction {
                mode_1: ParameterMode::POSITION,
                mode_2: ParameterMode::IMMEDIATE,
                mode_3: ParameterMode::POSITION,
                opcode: 2
            }
        );
    }
}
