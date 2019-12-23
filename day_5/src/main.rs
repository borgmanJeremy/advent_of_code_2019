use intcode::Cpu;
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
    let mut cpu = Cpu::new();
    let memory_initial_state =
        load_instructions("/home/jeremy/advent_of_code/day_5/input.txt").unwrap();
    cpu.memory.resize(memory_initial_state.len(), 0);
    cpu.memory.copy_from_slice(&memory_initial_state);

    cpu.run_program();
}
