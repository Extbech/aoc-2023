mod day_one;
mod day_three;
mod day_two;
mod day_six;
mod day_seven;
use crate::day_one::get_calibrated_value_sum;
use crate::day_one::get_calibrated_value_sum_part_two;
use crate::day_three::sum_engine_parts;
use crate::day_three::sum_gear_parts;
use crate::day_three::Either;
use crate::day_two::sum_possible_games;
use crate::day_two::sum_power_min_cubes;
use crate::day_six::day_six_task_one;
use crate::day_six::day_six_task_two;
use crate::day_seven::day_seven_task_one;
use crate::day_seven::day_seven_task_two;

fn main() {
    // <----- FILE PATHS ----->
    let file_path_day_1: String = String::from("data/day-1.txt");
    let file_path_day_2: String = String::from("data/day-2.txt");
    let file_path_day_3: String = String::from("data/day-3.txt");
    let file_path_day_6: String = String::from("data/day-6.txt");
    let file_path_day_7: String = String::from("data/day-7.txt");


    // Day 1
    println!("----------------------------------------------");
    let cailbrated_result_sum =
        get_calibrated_value_sum(&file_path_day_1).expect("error reading file!");
    println!(
        "Day 1 Task 1 Result:\t {}",
        cailbrated_result_sum.iter().sum::<i32>()
    );
    let cailbrated_result_sum_part_two =
        get_calibrated_value_sum_part_two(&file_path_day_1).expect("error reading file!");
    println!(
        "Day 1 Task 2 Result:\t {:?}",
        cailbrated_result_sum_part_two.iter().sum::<i32>()
    );

    // Day 2
    println!("----------------------------------------------");
    let game_sum = sum_possible_games(12, 13, 14, &file_path_day_2).expect("error reading file");
    println!("Day 2 Task 1 Result:\t{}", game_sum);
    let power_sum = sum_power_min_cubes(&file_path_day_2).expect("error reading file");
    println!("Day 2 Task 1 Result:\t{}", power_sum);

    // Day 3
    println!("----------------------------------------------");
    match sum_engine_parts(&file_path_day_3, false) {
        Either::Left(val) => println!("Day 3 Task 1 Result:\t{}", val.unwrap()),
        Either::Right(_) => println!("failed"),
    };
    let gear_sum = sum_gear_parts(&file_path_day_3).expect("error reading file");
    println!("Day 3 Task 2 Result:\t{}", gear_sum);

    // Day 6
    println!("----------------------------------------------");
    let day_six_one = day_six_task_one(&file_path_day_6).expect("error reading file");
    println!("Day 6 Task 1 Result:\t{}", day_six_one);
    let day_six_two = day_six_task_two(&file_path_day_6).expect("error reading file");
    println!("Day 6 Task 2 Result:\t{}", day_six_two);

    // Day 7
    println!("----------------------------------------------");
    let day_six_one = day_seven_task_one(&file_path_day_7).expect("error reading file");
    println!("Day 7 Task 1 Result:\t{}", day_six_one);
    let day_six_two = day_seven_task_two(&file_path_day_7).expect("error reading file");
    println!("Day 7 Task 2 Result:\t{}", day_six_two);

}
