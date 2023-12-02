mod dayOne;
use crate::dayOne;

fn main() {
    let file_path = String::from("data/data.txt");

    let cailbrated_result_sum = get_calibrated_value_sum(&file_path).expect("error reading file!");
    println!(
        "Task 1 Result:\t {}",
        cailbrated_result_sum.iter().sum::<i32>()
    );

    let cailbrated_result_sum_part_two =
        get_calibrated_value_sum_part_two(&file_path).expect("error reading file!");
    println!(
        "\nTask 2 Result:\t {:?}",
        cailbrated_result_sum_part_two.iter().sum::<i32>()
    );
}

#[cfg(test)]
mod tests;
