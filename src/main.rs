use std::fs::File;
use std::io::{BufReader, Error};
use std::io::prelude::*;
use std::collections::HashMap;


fn main() {
    let file_path = String::from("data/data.txt");

    let cailbrated_result_sum = get_calibrated_value_sum(&file_path).expect("error reading file!");
    println!("Task 1 Result:\t {}", cailbrated_result_sum.iter().sum::<i32>());

    let cailbrated_result_sum_part_two = get_calibrated_value_sum_part_two(&file_path).expect("error reading file!");
    println!("\nTask 2 Result:\t {:?}", cailbrated_result_sum_part_two.iter().sum::<i32>());
}

fn get_calibrated_value_sum(file_path: &String) -> Result<Vec<i32>, Error> {
    let mut sum: Vec<i32> = Vec::new(); 
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut line_vec: Vec<u32> = Vec::new();
    for line in reader.lines() {
        for c in line?.chars().into_iter() {
            match c.to_digit(10) {
                Some(x) => {line_vec.push(x)},
                _ => (),
            }
        }
        match line_vec.len() {
            x if x == 1 => {sum.push((line_vec[0].to_string() + &line_vec[0].to_string()).parse::<i32>().unwrap())},
            x if x > 1 => {sum.push((line_vec[0].to_string() + &line_vec[line_vec.len()-1].to_string()).parse::<i32>().unwrap())},
            _ => ()
        }
        line_vec.clear();

    }
    Ok(sum)

}

fn get_calibrated_value_sum_part_two(file_path: &String) -> Result<Vec<i32>, Error> {
    let mut sum: Vec<i32> = Vec::new();
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut line_vec: Vec<u32> = Vec::new();
    for line in reader.lines() {
        let correct_line = convert_vec(line?);
        for c in correct_line.into_iter() {
            match c.to_digit(10) {
                Some(x) => {line_vec.push(x)},
                _ => (),
            }
        }
        match line_vec.len() {
            x if x == 1 => {sum.push((line_vec[0].to_string() + &line_vec[0].to_string()).parse::<i32>().unwrap())},
            x if x > 1 => {sum.push((line_vec[0].to_string() + &line_vec[line_vec.len()-1].to_string()).parse::<i32>().unwrap())},
            _ => ()
        }
        line_vec.clear();
        
    }
    Ok(sum)
    
}

fn convert_vec(mut data: String) -> Vec<char> {
    let digits: Vec<String> = String::from("one two three four five six seven eight nine").split(" ").map(|s| s.to_string()).collect::<Vec<String>>();
    let mut first: HashMap<usize, (String, &String)> = HashMap::new();
    let mut last: HashMap<usize, (String, &String)> = HashMap::new();

    for (idx, digit) in digits.iter().enumerate() {

        if data.contains(digit) {
            let index: Vec<_> = data.match_indices(digit).collect();

            if first.is_empty() {
                first.insert(index[0].0, ((idx+1).to_string(), digit));
            } else {
                let first_key = first.keys().collect::<Vec<&usize>>()[0];
                if index[0].0 < *first_key {
                    first.clear();
                    first.insert(index[0].0, ((idx+1).to_string(), digit));
                }
            }
            if last.is_empty() {
                last.insert(index[index.len()-1].0, ((idx+1).to_string(), digit));
            } else {
                let last_key = last.keys().collect::<Vec<&usize>>()[0];
                if index[index.len()-1].0 > *last_key {
                    last.clear();
                    last.insert(index[index.len()-1].0, ((idx+1).to_string(), digit));
                }
            }

        }
    };
    let first_key = first.keys().collect::<Vec<&usize>>();
    let last_key = last.keys().collect::<Vec<&usize>>();
    if first_key.len() != 0 && last_key.len() != 0{
        let first_val = first.get(first_key[0]).unwrap();
        let second_val = last.get(last_key[0]).unwrap();
        if first_val.0 == second_val.0 {
            data = data.replace(first_val.1, &first_val.0);
        } else {
            if is_not_digit_before(first_key[0], &data) {
                data = data.replacen(first_val.1, &first_val.0, 1);
            }
            if is_not_digit_after(last_key[0], &data) {
                data = data.replace(second_val.1, &second_val.0);
            }
        }
        return data.chars().collect();
    }
    if first_key.len() != 0 {
        let first_val = first.get(first_key[0]).unwrap();
        data = data.replace(first_val.1, &first_val.0);
    }
    if last_key.len() != 0 {
        let second_val = last.get(last_key[0]).unwrap();
        data = data.replace(second_val.1, &second_val.0);
    }
    return data.chars().collect();

}

fn is_not_digit_before(index: &usize, data: &String) -> bool {
    let chars: Vec<char> = data.chars().collect();
    for i in 0..=*index {
        if chars[i].is_digit(10) {
            return false
        }
    }
    return true
}

fn is_not_digit_after(index: &usize, data: &String) -> bool {
    let chars: Vec<char> = data.chars().collect();
    for i in *index..=chars.len()-1 {
        if chars[i].is_digit(10) {
            return false
        }
    }
    return true
}

#[cfg(test)]
mod tests;