use std::collections::HashSet;
use std::error::Error;
// Not doing the proper error handling here
fn load_wires(path: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let mut ret_vec = Vec::new();
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path)?;
    for result in rdr.records() {
        let record = result?;
        let mut temp_vec = Vec::new();
        for elem in record.iter() {
            temp_vec.push(elem.parse().unwrap());
        }
        ret_vec.push(temp_vec);
    }
    Ok(ret_vec)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug)]
pub struct Command {
    direction: Direction,
    distance: i32,
}

#[derive(Debug)]
pub struct Wire {
    path: Vec<Point>,
    grid: HashSet<Point>,
    origin: Point,
    head: Point,
}

impl Wire {
    pub fn new() -> Wire {
        Wire {
            path: Vec::new(),
            grid: HashSet::new(),
            origin: Point { x: 0, y: 0 },
            head: Point { x: 0, y: 0 },
        }
    }
    pub fn add_leg_to_grid(&mut self, command: &Command) {
        match command.direction {
            Direction::UP => {
                let end_pos = self.head.y + command.distance;
                while self.head.y < end_pos {
                    let new_point = Point {
                        x: self.head.x,
                        y: self.head.y + 1,
                    };
                    self.grid.insert(new_point);
                    self.path.push(new_point);
                    self.head.y += 1;
                }
            }
            Direction::DOWN => {
                let end_pos = self.head.y - command.distance;
                while self.head.y > end_pos {
                    let new_point = Point {
                        x: self.head.x,
                        y: self.head.y - 1,
                    };
                    self.path.push(new_point);
                    self.grid.insert(new_point);
                    self.head.y -= 1;
                }
            }
            Direction::LEFT => {
                let end_pos = self.head.x - command.distance;
                while self.head.x > end_pos {
                    let new_point = Point {
                        x: self.head.x - 1,
                        y: self.head.y,
                    };
                    self.path.push(new_point);
                    self.grid.insert(new_point);
                    self.head.x -= 1;
                }
            }
            Direction::RIGHT => {
                let end_pos = self.head.x + command.distance;
                while self.head.x < end_pos {
                    let new_point = Point {
                        x: self.head.x + 1,
                        y: self.head.y,
                    };
                    self.path.push(new_point);
                    self.grid.insert(new_point);
                    self.head.x += 1;
                }
            }
        }
    }
    pub fn generate_hash(&mut self, path: &Vec<String>) {
        self.grid.clear();

        for str_cmd in path {
            let cmd = string_to_cmd(&str_cmd);
            self.add_leg_to_grid(&cmd);
        }
    }
}

fn manhattan_distance(p1: &Point, p2: &Point) -> i32 {
    (p1.x - p2.x).abs() + (p1.y - p2.y).abs()
}

fn length_to_point(path: &Vec<Point>, target: Point) -> Option<i32> {
    let mut length = 0;
    for point in path {
        if *point == target {
            return Some(length + 1);
        }
        length += 1;
    }
    return None;
}

fn string_to_cmd(input_string: &String) -> Command {
    let byte_string = input_string.as_bytes();
    let mut command = Command {
        direction: Direction::UP,
        distance: 0,
    };

    command.distance = std::str::from_utf8(&byte_string[1..])
        .unwrap()
        .parse::<i32>()
        .unwrap();

    match byte_string[0] as char {
        'U' => {
            command.direction = Direction::UP;
        }
        'D' => {
            command.direction = Direction::DOWN;
        }
        'L' => {
            command.direction = Direction::LEFT;
        }
        'R' => {
            command.direction = Direction::RIGHT;
        }
        _ => panic!("Unknown char"),
    }

    return command;
}
fn main() {
    let mut wire_vec = load_wires("/home/jeremy/advent_of_code/day_3/input.txt").unwrap();
    assert_eq!(wire_vec.len(), 2);

    let mut wire_1 = Wire::new();
    wire_1.generate_hash(&wire_vec[0].drain(0..).collect());

    let mut wire_2 = Wire::new();
    wire_2.generate_hash(&wire_vec[1].drain(0..).collect());

    let inter: Vec<Point> = wire_1.grid.intersection(&wire_2.grid).cloned().collect();
    // println!("{:?}", inter);

    let mut min_dist = 999999999;
    let mut min_point = Point { x: 0, y: 0 };
    for point in &inter {
        let new_dist = manhattan_distance(&Point { x: 0, y: 0 }, point);
        if new_dist < min_dist {
            min_dist = new_dist;
            min_point = *point;
        }
    }
    println!(
        "Min Dist {} at x: {} y: {}",
        min_dist, min_point.x, min_point.y
    );

    let mut min_latency = 999999;
    for point in &inter {
        let latency_1 = match length_to_point(&wire_1.path, *point) {
            Some(iter_len) => iter_len,
            None => panic!("No point Found"),
        };
        let latency_2 = match length_to_point(&wire_2.path, *point) {
            Some(iter_len) => iter_len,
            None => panic!("No point Found"),
        };
        let total_latency = latency_1 + latency_2;
        if total_latency < min_latency {
            min_latency = total_latency;
        }
    }
    println!("Total Latency is: {}", min_latency);
}
