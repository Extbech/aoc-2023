use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufReader, Error};

pub fn sum_possible_games(
    num_red_cubes: i32,
    num_green_cubes: i32,
    num_blue_cubes: i32,
    file_path: &str,
) -> Result<i32, Error> {
    let mut game_id_sum: i32 = 0;
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let mut game_id = 0;
        let mut game_not_possible = false;
        for section in line?.split(";") {
            // Find Game Id.
            if section.find("Game").is_some() {
                let game_section = section.split(":").collect::<Vec<&str>>();
                for word in game_section[0].split(" ") {
                    let Ok(id) = word.parse::<i32>() else {
                        continue;
                    };
                    game_id = id;
                }
                let not_possible = is_not_possible(
                    game_section[1],
                    num_red_cubes,
                    num_green_cubes,
                    num_blue_cubes,
                );
                if not_possible {
                    game_not_possible = true;
                    break;
                }
            } else {
                let not_possible =
                    is_not_possible(section, num_red_cubes, num_green_cubes, num_blue_cubes);
                if not_possible {
                    game_not_possible = true;
                    break;
                }
            }
        }
        if !game_not_possible {
            game_id_sum += game_id;
        }
    }
    Ok(game_id_sum)
}

pub fn sum_power_min_cubes(file_path: &str) -> Result<i32, Error> {
    let mut power_sum: i32 = 0;
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut min_cubes: HashMap<&str, u32> = HashMap::new();

    for line in reader.lines() {
        let mut l: &str = &line?.replace(" ", "");
        l = l.split(":").collect::<Vec<&str>>()[1];
        for section in l.split(";") {
            section.split(",").for_each(|c| match c {
                x if x.contains("red") => {
                    let count: u32 = x.replace("red", "").parse::<u32>().unwrap();
                    if !min_cubes.contains_key("red") {
                        min_cubes.insert("red", count);
                    } else {
                        if min_cubes.get(&"red").unwrap() < &count {
                            *min_cubes.entry("red").or_default() = count;
                        }
                    }
                }
                x if x.contains("green") => {
                    let count: u32 = x.replace("green", "").parse::<u32>().unwrap();
                    if !min_cubes.contains_key("green") {
                        min_cubes.insert("green", count);
                    } else {
                        if min_cubes.get(&"green").unwrap() < &count {
                            *min_cubes.entry("green").or_default() = count;
                        }
                    }
                }
                x if x.contains("blue") => {
                    let count: u32 = x.replace("blue", "").parse::<u32>().unwrap();
                    if !min_cubes.contains_key("blue") {
                        min_cubes.insert("blue", count);
                    } else {
                        if min_cubes.get(&"blue").unwrap() < &count {
                            *min_cubes.entry("blue").or_default() = count;
                        }
                    }
                }
                _ => (),
            })
        }
        let mut power: u32 = 0;
        println!("{:?}", min_cubes);
        min_cubes.values().for_each(|val| {
            if power == 0 {
                power = *val;
            } else {
                power *= val
            }
        });
        power_sum += power as i32;
        min_cubes.clear();
    }
    Ok(power_sum)
}

fn is_not_possible(
    game_section: &str,
    num_red_cubes: i32,
    num_green_cubes: i32,
    num_blue_cubes: i32,
) -> bool {
    return game_section
        .split(",")
        .collect::<Vec<&str>>()
        .iter()
        .any(|c| {
            //println!("{:?}", c.split(" ").collect::<Vec<&str>>());
            c.contains("red")
                && c.split(" ").collect::<Vec<&str>>()[1]
                    .parse::<i32>()
                    .unwrap()
                    > num_red_cubes
                || c.contains("green")
                    && c.split(" ").collect::<Vec<&str>>()[1]
                        .parse::<i32>()
                        .unwrap()
                        > num_green_cubes
                || c.contains("blue")
                    && c.split(" ").collect::<Vec<&str>>()[1]
                        .parse::<i32>()
                        .unwrap()
                        > num_blue_cubes
        });
}

#[test]
fn test_power_sum() {
    let file_path = String::from("data/day-2.txt");
    let result = sum_power_min_cubes(&file_path);

    let res = result.unwrap();

    assert_eq!(res[0], 55);

    assert_eq!(res[res.len() - 1], 55);

    assert_eq!(res[6], 88);

    assert_eq!(res.len(), 1000);
}
