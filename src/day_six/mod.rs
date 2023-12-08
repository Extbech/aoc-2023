use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Error};

pub fn day_six_task_one(file_path: &str) -> Result<i32, Error> {
        let mut sum_vec: Vec<i32> = Vec::new();
        let file = File::open(file_path)?;
        let reader = BufReader::new(file);
        let mut time_vec: Vec<i32> = Vec::new(); 
        let mut distance_vec: Vec<i32> = Vec::new();
        for line in reader.lines() {
            let res_line = line.unwrap();
            match res_line.contains("Time") {
                true => {
                    let num_str = res_line.split(":").collect::<Vec<&str>>()[1];
                    time_vec = num_str.trim().split("     ").map(|t| t.parse::<i32>().unwrap()).collect::<Vec<i32>>();


                },
                false => {
                    let num_str = res_line.split(":").collect::<Vec<&str>>()[1];
                    distance_vec = num_str.trim().split("   ").map(|d| d.parse::<i32>().unwrap()).collect::<Vec<i32>>();
                }
            }
        }
        for i in 0..time_vec.len() {
            let mut variations = 0;
            for j in 0..time_vec[i] {
                let distance: i32 = (time_vec[i] - j)*j;
                if distance > distance_vec[i] {
                    variations += 1
                }
            }
            if variations > 0 {
                sum_vec.push(variations);
            }
        }
        Ok(sum_vec.iter().product())
}

pub fn day_six_task_two(file_path: &str) -> Result<i32, Error> {
    let mut variations = 0;
    let mut time: i64 = 0; 
    let mut distance: i64 = 0;
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
        for line in reader.lines() {
            let res_line = line.unwrap();
            match res_line.contains("Time") {
                true => {
                    let num_str = res_line.split(":").collect::<Vec<&str>>()[1];
                    time = num_str.trim().split("     ").collect::<Vec<&str>>().join("").parse::<i64>().unwrap();
                    

                },
                false => {
                    let num_str = res_line.split(":").collect::<Vec<&str>>()[1];
                    distance = num_str.trim().split("   ").collect::<Vec<&str>>().join("").parse::<i64>().unwrap();
                }
            }
        }
        for j in 0..time {
            let d: i64 = (time - j)*j;
            if d > distance {
                variations += 1
            }
        }
        Ok(variations)
} 