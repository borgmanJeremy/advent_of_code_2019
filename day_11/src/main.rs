use day_11::intcode::*;
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
        load_instructions("/home/jeremy/advent_of_code/day_11/input.txt").unwrap();
    let mut cpu = Cpu::new();
    cpu.input_mode = day_11::intcode::IOMode::CAMERA;
    cpu.output_mode = day_11::intcode::IOMode::CAMERA;
   
    cpu.memory.resize(10 * 1024 * 1024, 0);

    for idx in 0..memory_initial_state.len() {
        cpu.memory[idx] = memory_initial_state[idx];
    }    
    cpu.run_program();
    cpu.camera.tx_main_color_query.send(day_11::color_robot::QueryType::PrintMap).unwrap();
    let msg = cpu.camera.rx_main_color_query.recv().unwrap();
}
