use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Error};
use std::collections::HashMap;

pub fn day_eight_task_one(file_path: &str) -> Result<i32, Error> {
    let file = File::open(file_path)?;
    let mut instructions: Vec<usize> = Vec::new();
    let mut network: HashMap<String, Vec<String>> = HashMap::new();
    let mut first_key = String::new();
    let mut last_key = String::new();
    let reader = BufReader::new(file);
    for (idx, line) in reader.lines().enumerate() {
        let res_line = line.unwrap();
        if idx == 0 {
            instructions = res_line.chars().map(|x| match x {
                'L' => 0,
                'R' => 1,
                _ => panic!("Only Expecting Left and Right Instructiuons")          
            }).collect();
            continue
        }
        if idx == 1 {
            continue
        }
        let split_line: Vec<String> = res_line.split('=').map(String::from).collect();
        let key = split_line[0].trim().to_string();
        if idx == 2 {
            first_key = key.clone();
        }
        if idx == 707 {
            last_key = key.clone();
        }
        let coords = split_line[1].trim().replace("(", "").replace(")", "").replace(" ", "");
        let split_coords: Vec<String> = coords.split(',').map(String::from).collect();
        network.insert(key, split_coords);
    }
    let coords = network.get(&first_key).expect("Key Not Found");
    let first_instruction = instructions[0];
    let mut node = &coords[first_instruction];
    if node.to_string() == last_key {
        return Ok(1)
    }
    let mut i = 0;
    let mut steps = 0;
    let mut not_found = true;
    while not_found {
        steps += 1;
        if i == instructions.len()-1 {
            i = 0;
        } else {
            i += 1;
        }
        let n_instruction = instructions[i];
        let next_node = network.get(node).expect("Key Not Found");
        node = &next_node[n_instruction];
        if node.to_string() == last_key {
            println!("{}\t{}", node, last_key);
            not_found = false;
        }
    }
    Ok(steps)

}

pub fn day_eight_trees(file_path: &str) -> Result<i32, Error> {
    
}