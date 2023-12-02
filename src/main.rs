mod day_one;
mod day_two;
use crate::day_one::get_calibrated_value_sum;
use crate::day_one::get_calibrated_value_sum_part_two;
use crate::day_two::sum_possible_games;
use crate::day_two::sum_power_min_cubes;

fn main() {
    let file_path_day_1: String = String::from("data/day-1.txt");
    let file_path_day_2: String = String::from("data/day-2.txt");

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
    let game_sum = sum_possible_games(12, 13, 14, &file_path_day_2).expect("error reading file");
    println!("\nDay 2 Task 1 Result:\t{}", game_sum);
    let power_sum = sum_power_min_cubes(&file_path_day_2).expect("error reading file");
    println!("Day 2 Task 1 Result:\t{}", power_sum);
}
