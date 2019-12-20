use std::fs;
use std::io::{prelude::*, BufReader};
fn main() {
    let file = fs::File::open("/home/jeremy/advent_of_code/day_1/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut module_fuel: i32;
    let mut fuel_fuel: i32;
    let mut total_fuel: i32 = 0;

    for line in reader.lines() {
        let module_weight: i32 = line.unwrap().parse().unwrap();
        module_fuel = fuel_for_module(module_weight);
        fuel_fuel = fuel_for_fuel(module_fuel);
        total_fuel += module_fuel + fuel_fuel;
    }
    println!("{}", total_fuel);
}

fn fuel_for_module(module_weight: i32) -> i32 {
    return (module_weight / 3) - 2;
}

fn fuel_for_fuel(fuel_weight: i32) -> i32 {
    let mut need_fuel = true;
    let mut additional_fuel = fuel_weight;
    let mut fuel_fuel = 0;
    while need_fuel {
        additional_fuel = fuel_for_module(additional_fuel);

        if additional_fuel > 0 {
            fuel_fuel += additional_fuel;
        } else {
            need_fuel = false;
        }
    }
    return fuel_fuel;
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fuel_fuel() {
        assert_eq!(fuel_for_fuel(2), 0);
        assert_eq!(fuel_for_fuel(654), 966 - 654);
        assert_eq!(fuel_for_fuel(33583), 50346 - 33583);
    }
}
