use angle::*;
use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn parse_map_to_vector(path: &str) -> Vec<Point> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open  file: {}", why.description()),
    };
    let buff = BufReader::new(file);

    let mut line_count = 0;
    let mut point_vec = Vec::new();

    for line in buff.lines() {
        let unwrap = line.unwrap();
        let char_array = unwrap.as_bytes();

        for idx in 0..char_array.len() {
            if char_array[idx] as char == '#' {
                point_vec.push(Point {
                    x: idx as i128,
                    y: line_count,
                })
            }
        }
        line_count += 1;
    }
    return point_vec;
}

fn part_1() {
    let map = parse_map_to_vector("/home/jeremy/advent_of_code/day_10/input.txt");

    let mut max = 0;
    let mut max_point = Point { x: 0, y: 0 };
    for origin in &map {
        let mut raw_angle = Vec::new();
        for destination in &map {
            let angle = calculate_angle(origin, destination);
            raw_angle.push(angle);
        }
        normalize_angles_to_lcm(&mut raw_angle);
        let set: HashSet<Fraction> = raw_angle.into_iter().collect();
        let count = set.len() - 1;
        if count > max {
            max = count;
            max_point.x = origin.x;
            max_point.y = origin.y;
        }
    }
    println!("Pos: {:?} Num seen: {}", max_point, max);
}

fn main() {
    part_1();

    let map = parse_map_to_vector("/home/jeremy/advent_of_code/day_10/input.txt");

    let origin = Point { x: 29, y: 28 };
    let mut raw_angle = Vec::new();
    for destination in &map {
        let angle = calculate_angle(&origin, destination);
        raw_angle.push(angle);
    }
    normalize_angles_to_lcm(&mut raw_angle);
    let lcm = calc_lcm(&raw_angle);

    let mut dx = Vec::new();
    for idx in 0..=lcm {
        dx.push(idx);
    }
}
