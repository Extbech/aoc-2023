use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Error};

pub fn day_five_task_1(file_path: &str) -> Result<i32, Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(line) = line {
            let chunks = line.split("").collect::<Vec<&str>>();
            println!("{:?}", chunks);
        }
    }
    Ok(1)
}
