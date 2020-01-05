use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
struct Chemical {
    name: String,
}

impl Chemical {
    fn new(name: &str) -> Chemical {
        Chemical {
            name: name.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
struct Agent {
    name: Chemical,
    quantity: i64,
}
#[derive(Debug, Clone)]
struct Reaction {
    inputs: Vec<Agent>,
    output: Agent,
}

fn parse_recipe_to_list(path: &str) -> Vec<Reaction> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(why) => panic!("couldn't open  file: {}", why.description()),
    };
    let buff = BufReader::new(file);
    let mut reaction_list = Vec::new();
    for line in buff.lines() {
        let unwrap = line.unwrap();
        let token_split: Vec<&str> = unwrap.split("=>").collect();
        let lhs_comma_split: Vec<&str> = token_split[0].split(",").collect();
        let rhs: Vec<&str> = token_split[1].split(" ").collect();

        let output_quantity: i64 = rhs[1].parse().unwrap();
        let output_name = Chemical::new(rhs[2]);

        let number_inputs = lhs_comma_split.len();
        let mut reaction_inputs = Vec::new();
        for idx in 0..number_inputs {
            let lhs_input: Vec<&str> = lhs_comma_split[idx].trim().split(" ").collect();
            let input_quantity: i64 = lhs_input[0].trim().parse().unwrap();
            let input_name = Chemical::new(lhs_input[1].trim());
            reaction_inputs.push(Agent {
                name: input_name.clone(),
                quantity: input_quantity.clone(),
            });
        }
        reaction_list.push(Reaction {
            inputs: reaction_inputs.clone(),
            output: Agent {
                name: output_name.clone(),
                quantity: output_quantity,
            },
        });
    }

    reaction_list.push(Reaction {
        inputs: Vec::new(),
        output: Agent {
            name: Chemical::new("ORE"),
            quantity: 0,
        },
    });

    return reaction_list;
}

fn main() {
    let recipe_list = parse_recipe_to_list("/home/jeremy/advent_of_code/day_14/input.txt");

    let mut agent_list = HashMap::new();
    for i in 0..recipe_list.len() {
        agent_list.insert(recipe_list[i].output.name.name.clone(), 0);
    }
    agent_list.insert("ORE".to_string(), 0);
    let mut fuel_count = 0;
    let mut ore_count;

    let mut last_list = agent_list.clone();
    let mut fuel_quantity = 2 << 13;
    loop {
        loop {
            manufacture_fuel(fuel_quantity, &mut agent_list, &recipe_list);
            fuel_count += fuel_quantity;
            ore_count = agent_list["ORE"].abs() as i64;

            if ore_count >= 1000000000000 {
                fuel_count -= fuel_quantity;
                break;
            } else {
                last_list = agent_list.clone();
            }
        }
        agent_list = last_list.clone();
        ore_count = agent_list["ORE"].abs() as i64;
        fuel_quantity >>= 1;
        if fuel_quantity < 1 {
            break;
        }
    }
    println!("ore_count: {} fuel count: {}", ore_count, fuel_count);
}
fn manufacture_fuel(
    fuel_quantity: i64,
    agent_list: &mut HashMap<String, i64>,
    recipe_list: &Vec<Reaction>,
) {
    *agent_list.get_mut("FUEL").unwrap() = -1 * fuel_quantity;
    loop {
        let mut output: &str = "";
        let mut reaction_done = true;
        for (key, elem) in &*agent_list {
            if *elem < 0 && key != "ORE" {
                reaction_done = false;
                output = &key[..];
                break;
            }
        }
        if reaction_done == true {
            break;
        }
        let reaction = find_reaction_by_output(output, &recipe_list).unwrap();
        if agent_list[&reaction.output.name.name] < 0 {
            let num_react = (agent_list[&reaction.output.name.name].abs() as f32
                / reaction.output.quantity as f32)
                .ceil() as i64;
            let produced = num_react * reaction.output.quantity;
            *agent_list.get_mut(&reaction.output.name.name).unwrap() += produced;

            for input in &reaction.inputs {
                *agent_list.get_mut(&input.name.name).unwrap() -= input.quantity * num_react;
            }
        }
    }
}

fn find_reaction_by_output(output: &str, recipes: &Vec<Reaction>) -> Option<Reaction> {
    for i in 0..recipes.len() {
        if recipes[i].output.name.name == *output {
            return Some(recipes[i].clone());
        }
    }
    None
}
