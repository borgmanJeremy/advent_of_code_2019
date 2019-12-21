use std::error::Error;
// Not doing the proper error handling here
fn load_instructions(path: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    let mut ret_vec = Vec::new();
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path)?;
    for result in rdr.records() {
        let record = result?;
        for elem in record.iter() {
            ret_vec.push(elem.parse().unwrap());
        }
    }
    Ok(ret_vec)
}

#[derive(Debug)]
enum CpuState {
    HALT,
    RUN,
}

#[derive(Debug)]
pub struct Cpu {
    pc: usize,
    state: CpuState,
    memory: Vec<i32>,
}
impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            pc: 0,
            state: CpuState::RUN,
            memory: Vec::new(),
        }
    }
    pub fn get_value(&self, address: i32) -> i32 {
        let idx = address as usize;
        self.memory[idx]
    }

    pub fn write_value(&mut self, address: i32, value: i32) {
        let idx = address as usize;
        self.memory[idx] = value;
    }
    pub fn run_program(&mut self) {
        loop {
            match self.state {
                CpuState::RUN => {
                    self.exec();
                }
                CpuState::HALT => {
                    break;
                }
            }
        }
    }

    pub fn exec(&mut self) {
        match self.memory[self.pc] {
            1 => {
                let val = self.get_value(self.memory[self.pc + 1])
                    + self.get_value(self.memory[self.pc + 2]);

                self.write_value(self.memory[self.pc + 3], val);

                self.pc += 4;
            }
            2 => {
                let val = self.get_value(self.memory[self.pc + 1])
                    * self.get_value(self.memory[self.pc + 2]);

                self.write_value(self.memory[self.pc + 3], val);

                self.pc += 4;
            }
            99 => {
                self.state = CpuState::HALT;
            }
            _ => {
                panic!("Illegal Op Code");
            }
        }
    }
}

fn main() {
    let mut cpu = Cpu::new();
    let memory_initial_state =
        load_instructions("/home/jeremy/advent_of_code/day_2/input.txt").unwrap();
    cpu.memory.resize(memory_initial_state.len(), 0);
    cpu.memory.copy_from_slice(&memory_initial_state);
    let noun_list: Vec<i32> = (0..100).collect();
    let verb_list: Vec<i32> = (0..100).collect();

    for noun in &noun_list {
        for verb in &verb_list {
            cpu.memory.copy_from_slice(&memory_initial_state);
            cpu.pc = 0;
            cpu.state = CpuState::RUN;
            cpu.memory[1] = *noun;
            cpu.memory[2] = *verb;
            cpu.run_program();
            if cpu.memory[0] == 19690720 {
                println!("noun {}, verb {} answer {}", noun, verb, 100 * noun + verb);
                //break;
            }
        }
    }
}
