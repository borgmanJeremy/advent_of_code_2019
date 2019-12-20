use std::error::Error;
use std::process;

fn example(path: &str) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path(path)?;
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

fn main() {
    if let Err(err) = example("/home/jeremy/advent_of_code/day_2/input.txt") {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
