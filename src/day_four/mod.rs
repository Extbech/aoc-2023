use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Error};

pub fn get_winning_number_sum(file_path: &str) -> Result<i32, Error> {
    let mut sum: i32 = 0;
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Some(line_split) = line?.split(":").collect::<Vec<&str>>().get(1) {
            let work_line = line_split.split(" | ").collect::<Vec<&str>>();
            let winning_set: HashSet<i32> = work_line[0]
                .replace("  ", " ")
                .trim_start()
                .split(" ")
                .map(|num| num.parse::<i32>().unwrap())
                .collect();
            let selected_set: HashSet<i32> = work_line[1]
                .replace("  ", " ")
                .trim_start()
                .split(" ")
                .map(|num| num.parse::<i32>().unwrap())
                .collect();

            let winning: Vec<i32> = winning_set.intersection(&selected_set).copied().collect();

            if winning.len() > 0 {
                sum += i32::pow(2, (winning.len() as u32) - 1);
            }
        }
    }
    Ok(sum)
}

pub fn find_num_scratchcards(file_path: &str) -> Result<i32, Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut card_count: HashMap<usize, i32> = (1..=214).map(|num| (num, 1)).collect();
    for (idx, line) in reader.lines().enumerate() {
        if let Some(line_split) = line?.split(":").collect::<Vec<&str>>().get(1) {
            let work_line = line_split.split(" | ").collect::<Vec<&str>>();
            let winning_set: HashSet<i32> = work_line[0]
                .replace("  ", " ")
                .trim_start()
                .split(" ")
                .map(|num| num.parse::<i32>().unwrap())
                .collect();
            let selected_set: HashSet<i32> = work_line[1]
                .replace("  ", " ")
                .trim_start()
                .split(" ")
                .map(|num| num.parse::<i32>().unwrap())
                .collect();

            let winning: Vec<i32> = winning_set.intersection(&selected_set).copied().collect();
            let current_card_copies = *card_count.get(&(idx + 1)).unwrap();
            for i in idx + 2..idx + 2 + winning.len() {
                *card_count.entry(i).or_insert(1) += current_card_copies;
            }
        }
    }
    Ok(card_count.values().sum())
}
