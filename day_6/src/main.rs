use graph::Graph;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn main() -> io::Result<()> {
    println!("Hello world");

    let file = File::open("/home/jeremy/advent_of_code/day_6/input_simple.txt")?;
    let buff = BufReader::new(file);

    let mut graph = Graph::new();
    for line in buff.lines() {
        let unwrap = line.unwrap();
        let split = unwrap.split(")").collect::<Vec<&str>>();
        println!("{} -> {}", &split[1], &split[0]);

        match graph.add_unique_node(String::from(split[1])) {
            Ok(_) => {}
            Err(_) => {}
        }

        match graph.add_unique_node(String::from(split[0])) {
            Ok(_) => {}
            Err(_) => {}
        }

        match graph.add_edge_by_name(&split[1], &split[0]) {
            Ok(_) => {}
            Err(_) => {}
        }
    }
    let mut count = 0;
    for node in &graph.nodes {
        count += graph.count_to_head(&node.name).unwrap();
    }
    println!("Count is: {}", count);

    let ancestor = graph.least_common_ancestor("I", "K").unwrap();
    let travel_dist =
        graph.count_to_node("I", &ancestor).unwrap() + graph.count_to_node("K", &ancestor).unwrap();
    println! {"Ancestor is: {}", ancestor};
    println! {"Travel Dist is: {}", travel_dist};
    Ok(())
}
