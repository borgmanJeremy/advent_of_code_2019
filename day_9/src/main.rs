use intcode::Cpu;

use std::error::Error;
// Not doing the proper error handling here
fn load_instructions(path: &str) -> Result<Vec<i128>, Box<dyn Error>> {
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
        load_instructions("/home/jeremy/advent_of_code/day_9/input.txt").unwrap();

    let mut boost = Cpu::new();
    boost.memory.resize(10 * 1024 * 1024, 0);

    for idx in 0..memory_initial_state.len() {
        boost.memory[idx] = memory_initial_state[idx];
    }

    boost.run_program();
}
