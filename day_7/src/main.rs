use intcode::{Cpu, CpuState, IOMode};
use permutohedron::heap_recursive;

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

fn main() {
    let memory_initial_state =
        load_instructions("/home/jeremy/advent_of_code/day_7/input.txt").unwrap();

    let mut phase_setting = vec![5, 6, 7, 8, 9];
    let mut permutations = Vec::new();
    heap_recursive(&mut phase_setting, |permutation| {
        permutations.push(permutation.to_vec())
    });
    let mut max_val = 0;
    for phase in permutations {
        let mut amp_1 = Cpu::new();
        let mut amp_2 = Cpu::new();
        let mut amp_3 = Cpu::new();
        let mut amp_4 = Cpu::new();
        let mut amp_5 = Cpu::new();
        amp_1.memory.resize(memory_initial_state.len(), 0);
        amp_1.memory.copy_from_slice(&memory_initial_state);
        amp_2.memory.resize(memory_initial_state.len(), 0);
        amp_2.memory.copy_from_slice(&memory_initial_state);
        amp_3.memory.resize(memory_initial_state.len(), 0);
        amp_3.memory.copy_from_slice(&memory_initial_state);
        amp_4.memory.resize(memory_initial_state.len(), 0);
        amp_4.memory.copy_from_slice(&memory_initial_state);
        amp_5.memory.resize(memory_initial_state.len(), 0);
        amp_5.memory.copy_from_slice(&memory_initial_state);

        amp_1.input_mode = IOMode::STACK;
        amp_1.output_mode = IOMode::STACK;
        amp_2.input_mode = IOMode::STACK;
        amp_2.output_mode = IOMode::STACK;
        amp_3.input_mode = IOMode::STACK;
        amp_3.output_mode = IOMode::STACK;
        amp_4.input_mode = IOMode::STACK;
        amp_4.output_mode = IOMode::STACK;
        amp_5.input_mode = IOMode::STACK;
        amp_5.output_mode = IOMode::STACK;

        amp_5.output_stack.push(0);

        let mut first_iter = true;
        loop {
            if (amp_1.state == CpuState::HALT)
                && (amp_2.state == CpuState::HALT)
                && (amp_3.state == CpuState::HALT)
                && (amp_4.state == CpuState::HALT)
                && (amp_5.state == CpuState::HALT)
            {
                break;
            }

            if first_iter == true {
                amp_1.input_stack.push(amp_5.output_stack[0]);
                amp_1.input_stack.push(phase[0]);
                amp_5.output_stack.clear();
                amp_1.run_program();

                amp_2.input_stack.push(amp_1.output_stack[0]);
                amp_2.input_stack.push(phase[1]);
                amp_1.output_stack.clear();
                amp_2.run_program();

                amp_3.input_stack.push(amp_2.output_stack[0]);
                amp_3.input_stack.push(phase[2]);
                amp_2.output_stack.clear();
                amp_3.run_program();

                amp_4.input_stack.push(amp_3.output_stack[0]);
                amp_4.input_stack.push(phase[3]);
                amp_3.output_stack.clear();
                amp_4.run_program();

                amp_5.input_stack.push(amp_4.output_stack[0]);
                amp_5.input_stack.push(phase[4]);
                amp_4.output_stack.clear();
                amp_5.run_program();
                first_iter = false;
            }

            amp_1.input_stack.push(amp_5.output_stack[0]);
            amp_5.output_stack.clear();
            amp_1.run_program();

            amp_2.input_stack.push(amp_1.output_stack[0]);
            amp_1.output_stack.clear();
            amp_2.run_program();

            amp_3.input_stack.push(amp_2.output_stack[0]);
            amp_2.output_stack.clear();
            amp_3.run_program();

            amp_4.input_stack.push(amp_3.output_stack[0]);
            amp_3.output_stack.clear();
            amp_4.run_program();

            amp_5.input_stack.push(amp_4.output_stack[0]);
            amp_4.output_stack.clear();
            amp_5.run_program();
        }
        if max_val < amp_5.output_stack[0] {
            max_val = amp_5.output_stack[0];
        }
    }

    println!("Max Thruster Value is: {}", max_val);
}
// Part 1

//    let memory_initial_state =
//        load_instructions("/home/jeremy/advent_of_code/day_7/input.txt").unwrap();
//
//    let mut phase_setting = vec![0, 1, 2, 3, 4];
//    let mut permutations = Vec::new();
//    heap_recursive(&mut phase_setting, |permutation| {
//        permutations.push(permutation.to_vec())
//    });
//    let mut max_val = 0;
//    for phase in permutations {
//        let mut amp_1 = Cpu::new();
//        let mut amp_2 = Cpu::new();
//        let mut amp_3 = Cpu::new();
//        let mut amp_4 = Cpu::new();
//        let mut amp_5 = Cpu::new();
//        amp_1.memory.resize(memory_initial_state.len(), 0);
//        amp_1.memory.copy_from_slice(&memory_initial_state);
//        amp_2.memory.resize(memory_initial_state.len(), 0);
//        amp_2.memory.copy_from_slice(&memory_initial_state);
//        amp_3.memory.resize(memory_initial_state.len(), 0);
//        amp_3.memory.copy_from_slice(&memory_initial_state);
//        amp_4.memory.resize(memory_initial_state.len(), 0);
//        amp_4.memory.copy_from_slice(&memory_initial_state);
//        amp_5.memory.resize(memory_initial_state.len(), 0);
//        amp_5.memory.copy_from_slice(&memory_initial_state);
//
//        amp_1.input_mode = IOMode::STACK;
//        amp_1.output_mode = IOMode::STACK;
//        amp_2.input_mode = IOMode::STACK;
//        amp_2.output_mode = IOMode::STACK;
//        amp_3.input_mode = IOMode::STACK;
//        amp_3.output_mode = IOMode::STACK;
//        amp_4.input_mode = IOMode::STACK;
//        amp_4.output_mode = IOMode::STACK;
//        amp_5.input_mode = IOMode::STACK;
//        amp_5.output_mode = IOMode::STACK;
//
//        amp_1.input_stack.push(0);
//        amp_1.input_stack.push(phase[0]);
//        amp_1.run_program();
//
//        amp_2.input_stack.push(amp_1.output_stack[0]);
//        amp_2.input_stack.push(phase[1]);
//        amp_2.run_program();
//
//        amp_3.input_stack.push(amp_2.output_stack[0]);
//        amp_3.input_stack.push(phase[2]);
//        amp_3.run_program();
//
//        amp_4.input_stack.push(amp_3.output_stack[0]);
//        amp_4.input_stack.push(phase[3]);
//        amp_4.run_program();
//
//        amp_5.input_stack.push(amp_4.output_stack[0]);
//        amp_5.input_stack.push(phase[4]);
//        amp_5.run_program();
//
//        if max_val < amp_5.output_stack[0] {
//            max_val = amp_5.output_stack[0];
//        }
//    }
//
//    println!("Max Thruster Value is: {}", max_val);
//
