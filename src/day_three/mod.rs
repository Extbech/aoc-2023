use std::fs;
use std::io::Error;

pub enum Either<A, B> {
    Left(A),
    Right(B),
}

pub fn sum_engine_parts(
    file_path: &str,
    test: bool,
) -> Either<Result<i32, Error>, Result<Vec<i32>, Error>> {
    if test {
        let mut engine_sum_vec: Vec<i32> = Vec::new();
        let file = fs::read_to_string(file_path).expect("error reading file!");
        let file_array = file.split("\n").collect::<Vec<&str>>();
        for i in 0..file_array.len() {
            let sum = row_sum(&file_array, i);
            engine_sum_vec.push(sum);
        }
        Either::Right(Ok(engine_sum_vec))
    } else {
        let mut engine_sum: i32 = 0;
        let file = fs::read_to_string(file_path).expect("error reading file!");
        let file_array = file.split("\n").collect::<Vec<&str>>();
        for i in 0..file_array.len() {
            let sum = row_sum(&file_array, i);
            engine_sum += sum;
        }
        Either::Left(Ok(engine_sum))
    }
}

pub fn sum_gear_parts(file_path: &str) -> Result<i32, Error> {
    let mut sum_gears: i32 = 0;
    let file = fs::read_to_string(file_path).expect("error reading file!");
    let file_array = file.split("\n").collect::<Vec<&str>>();
    for i in 0..file_array.len() - 1 {
        let sum = gear_sum_row(&file_array, i);
        sum_gears += sum;
    }
    Ok(sum_gears)
}
fn gear_sum_row(arr: &Vec<&str>, index: usize) -> i32 {
    let mut sum: i32 = 0;
    match index {
        i if i == 0 => {
            let char_arr: Vec<char> = arr[i].chars().collect();
            let next_char_arr: Vec<char> = arr[i + 1].chars().collect();
            let mut j: usize = 0;
            while j < char_arr.len() {
                if char_arr[j] == '*' {
                    let (is_num, num) = search_for_numbers_first_row(&char_arr, &next_char_arr, j);
                    if is_num {
                        sum += num;
                    }
                }
                j += 1;
            }
        }
        i if i > 0 && i < arr.len() - 1 => {
            let char_arr: Vec<char> = arr[i].chars().collect();
            let next_char_arr: Vec<char> = arr[i + 1].chars().collect();
            let prev_char_arr: Vec<char> = arr[i - 1].chars().collect();
            let mut j: usize = 0;
            while j < char_arr.len() {
                if char_arr[j] == '*' {
                    let (is_num, num) =
                        search_for_numbers_n_row(&char_arr, &prev_char_arr, &next_char_arr, j);
                    if is_num {
                        println!("row: {}\tsum: {}", i + 1, num);
                        println!("-----------------------------------------");
                        sum += num;
                    }
                }
                j += 1;
            }
        }
        i if i == arr.len() - 1 => {
            let char_arr: Vec<char> = arr[i].chars().collect();
            let prev_char_arr: Vec<char> = arr[i - 1].chars().collect();
            let mut j: usize = 0;
            while j < char_arr.len() {
                if char_arr[j] == '*' {
                    let (is_num, num) = search_for_numbers_last_row(&char_arr, &prev_char_arr, j);
                    if is_num {
                        sum += num;
                    }
                }
                j += 1;
            }
        }
        _ => (),
    }
    sum
}

fn search_for_numbers_first_row(
    main_row: &Vec<char>,
    next_row: &Vec<char>,
    index: usize,
) -> (bool, i32) {
    let mut first: bool = false;
    let mut second: bool = false;
    let mut first_num: i32 = 0;
    let mut second_num: i32 = 0;

    //Main Row
    let mut j: usize = index - 1;
    let mut k: usize = index + 1;
    let mut placeholder_num: String = String::new();
    if main_row[j].is_ascii_digit() {
        first = true;
        while j > 0 && main_row[j].is_ascii_digit() {
            placeholder_num.insert(0, main_row[j]);
            j -= 1;
        }
        first_num = placeholder_num.parse::<i32>().unwrap();
        j = index - 1;
        placeholder_num.clear();
    }
    if main_row[k].is_ascii_digit() {
        if first {
            second = true;
        } else {
            first = true;
        }
        while j > 0 && main_row[k].is_ascii_digit() {
            placeholder_num.insert(0, main_row[j]);
            k += 1;
        }
        if second {
            second_num = placeholder_num.parse::<i32>().unwrap();
        } else {
            first_num = placeholder_num.parse::<i32>().unwrap();
        }
        k = index + 1;
        placeholder_num.clear();
    }
    // Next Row
    if next_row[j].is_ascii_digit() {
        if first {
            second = true;
        } else {
            first = true;
        }
        if next_row[j - 1].is_ascii_digit() && next_row[j + 1].is_ascii_digit() {
            placeholder_num += &next_row[j - 1].to_string();
            placeholder_num += &next_row[j].to_string();
            placeholder_num += &next_row[j + 1].to_string();
        } else if next_row[j - 1].is_ascii_digit() && next_row[j - 2].is_ascii_digit() {
            placeholder_num += &next_row[j - 2].to_string();
            placeholder_num += &next_row[j - 1].to_string();
            placeholder_num += &next_row[j].to_string();
        } else if next_row[j - 1].is_ascii_digit() {
            placeholder_num += &next_row[j - 1].to_string();
            placeholder_num += &next_row[j].to_string();
        } else if next_row[j + 1].is_ascii_digit() && next_row[j + 2].is_ascii_digit() {
            placeholder_num += &next_row[j].to_string();
            placeholder_num += &next_row[j + 1].to_string();
            placeholder_num += &next_row[j + 2].to_string();
        } else if next_row[j + 1].is_ascii_digit() {
            placeholder_num += &next_row[j].to_string();
            placeholder_num += &next_row[j + 1].to_string();
        } else {
            placeholder_num += &next_row[j].to_string();
        }
        if second {
            second_num = placeholder_num.parse::<i32>().unwrap();
        } else {
            first_num = placeholder_num.parse::<i32>().unwrap();
        }
    }
    if next_row[index].is_ascii_digit() {
        if first {
            second = true;
        } else {
            first = true;
        }
        if next_row[index - 1].is_ascii_digit() && next_row[index + 1].is_ascii_digit() {
            placeholder_num += &next_row[index - 1].to_string();
            placeholder_num += &next_row[index].to_string();
            placeholder_num += &next_row[index + 1].to_string();
        } else if next_row[index - 1].is_ascii_digit() && next_row[index - 2].is_ascii_digit() {
            placeholder_num += &next_row[index - 2].to_string();
            placeholder_num += &next_row[index - 1].to_string();
            placeholder_num += &next_row[index].to_string();
        } else if next_row[index - 1].is_ascii_digit() {
            placeholder_num += &next_row[index - 1].to_string();
            placeholder_num += &next_row[index].to_string();
        } else if next_row[index + 1].is_ascii_digit() && next_row[index + 2].is_ascii_digit() {
            placeholder_num += &next_row[index].to_string();
            placeholder_num += &next_row[index + 1].to_string();
            placeholder_num += &next_row[index + 2].to_string();
        } else if next_row[index + 1].is_ascii_digit() {
            placeholder_num += &next_row[index].to_string();
            placeholder_num += &next_row[index + 1].to_string();
        } else {
            placeholder_num += &next_row[index].to_string();
        }
        if second {
            second_num = placeholder_num.parse::<i32>().unwrap();
        } else {
            first_num = placeholder_num.parse::<i32>().unwrap();
        }
    }
    if next_row[k].is_ascii_digit() {
        if first {
            second = true;
        } else {
            first = true;
        }
        if next_row[k - 1].is_ascii_digit() && next_row[k + 1].is_ascii_digit() {
            placeholder_num += &next_row[k - 1].to_string();
            placeholder_num += &next_row[k].to_string();
            placeholder_num += &next_row[k + 1].to_string();
        } else if next_row[k - 1].is_ascii_digit() && next_row[k - 2].is_ascii_digit() {
            placeholder_num += &next_row[k - 2].to_string();
            placeholder_num += &next_row[k - 1].to_string();
            placeholder_num += &next_row[k].to_string();
        } else if next_row[k - 1].is_ascii_digit() {
            placeholder_num += &next_row[k - 1].to_string();
            placeholder_num += &next_row[k].to_string();
        } else if next_row[k + 1].is_ascii_digit() && next_row[k + 2].is_ascii_digit() {
            placeholder_num += &next_row[k].to_string();
            placeholder_num += &next_row[k + 1].to_string();
            placeholder_num += &next_row[k + 2].to_string();
        } else if next_row[k + 1].is_ascii_digit() {
            placeholder_num += &next_row[k].to_string();
            placeholder_num += &next_row[k + 1].to_string();
        } else {
            placeholder_num += &next_row[k].to_string();
        }
        if second {
            second_num = placeholder_num.parse::<i32>().unwrap();
        } else {
            first_num = placeholder_num.parse::<i32>().unwrap();
        }
    }
    if first && second {
        (true, first_num * second_num)
    } else {
        (false, 0)
    }
}

fn search_for_numbers_n_row(
    main_row: &Vec<char>,
    prev_row: &Vec<char>,
    next_row: &Vec<char>,
    index: usize,
) -> (bool, i32) {
    let mut first: bool = false;
    let mut second: bool = false;
    let mut first_num: i32 = 0;
    let mut second_num: i32 = 0;
    let mut num_next_row: i32 = 0;
    let mut num_index_row: i32 = 0;

    //Main Row
    let mut j: usize = index - 1;
    let mut k: usize = index + 1;
    let mut placeholder_num: String = String::new();
    if main_row[j].is_ascii_digit() {
        first = true;
        while j > 0 && main_row[j].is_ascii_digit() {
            placeholder_num.insert(0, main_row[j]);
            j -= 1;
        }
        first_num = placeholder_num.parse::<i32>().unwrap();
        j = index - 1;
        placeholder_num.clear();
    }
    if main_row[k].is_ascii_digit() {
        if first {
            second = true;
        } else {
            first = true;
        }
        while j > 0 && main_row[k].is_ascii_digit() {
            placeholder_num += &main_row[k].to_string();
            k += 1;
        }
        if second {
            second_num = placeholder_num.parse::<i32>().unwrap();
        } else {
            first_num = placeholder_num.parse::<i32>().unwrap();
        }
        k = index + 1;
        placeholder_num.clear();
    }
    // Next Row
    if next_row[j].is_ascii_digit() {
        if first {
            second = true;
        } else {
            first = true;
        }
        if next_row[j - 1].is_ascii_digit() && next_row[j + 1].is_ascii_digit() {
            placeholder_num += &next_row[j - 1].to_string();
            placeholder_num += &next_row[j].to_string();
            placeholder_num += &next_row[j + 1].to_string();
        } else if next_row[j - 1].is_ascii_digit() && next_row[j - 2].is_ascii_digit() {
            placeholder_num += &next_row[j - 2].to_string();
            placeholder_num += &next_row[j - 1].to_string();
            placeholder_num += &next_row[j].to_string();
        } else if next_row[j - 1].is_ascii_digit() {
            placeholder_num += &next_row[j - 1].to_string();
            placeholder_num += &next_row[j].to_string();
        } else if next_row[j + 1].is_ascii_digit() && next_row[j + 2].is_ascii_digit() {
            placeholder_num += &next_row[j].to_string();
            placeholder_num += &next_row[j + 1].to_string();
            placeholder_num += &next_row[j + 2].to_string();
        } else if next_row[j + 1].is_ascii_digit() {
            placeholder_num += &next_row[j].to_string();
            placeholder_num += &next_row[j + 1].to_string();
        } else {
            placeholder_num += &next_row[j].to_string();
        }
        if second {
            second_num = placeholder_num.parse::<i32>().unwrap();
            placeholder_num.clear();
        } else {
            first_num = placeholder_num.parse::<i32>().unwrap();
            num_next_row = first_num;
            placeholder_num.clear();
        }
    }
    if next_row[index].is_ascii_digit() {
        if first {
            second = true;
        } else {
            first = true;
        }
        if next_row[index - 1].is_ascii_digit() && next_row[index + 1].is_ascii_digit() {
            placeholder_num += &next_row[index - 1].to_string();
            placeholder_num += &next_row[index].to_string();
            placeholder_num += &next_row[index + 1].to_string();
        } else if next_row[index - 1].is_ascii_digit() && next_row[index - 2].is_ascii_digit() {
            placeholder_num += &next_row[index - 2].to_string();
            placeholder_num += &next_row[index - 1].to_string();
            placeholder_num += &next_row[index].to_string();
        } else if next_row[index - 1].is_ascii_digit() {
            placeholder_num += &next_row[index - 1].to_string();
            placeholder_num += &next_row[index].to_string();
        } else if next_row[index + 1].is_ascii_digit() && next_row[index + 2].is_ascii_digit() {
            placeholder_num += &next_row[index].to_string();
            placeholder_num += &next_row[index + 1].to_string();
            placeholder_num += &next_row[index + 2].to_string();
        } else if next_row[index + 1].is_ascii_digit() {
            placeholder_num += &next_row[index].to_string();
            placeholder_num += &next_row[index + 1].to_string();
        } else {
            placeholder_num += &next_row[index].to_string();
        }
        if second {
            let temp = placeholder_num.parse::<i32>().unwrap();
            if temp != num_next_row {
                second_num = temp;
            }
            placeholder_num.clear();
        } else {
            first_num = placeholder_num.parse::<i32>().unwrap();
            num_index_row = first_num;
            placeholder_num.clear();
        }
    }
    if next_row[k].is_ascii_digit() {
        if first {
            second = true;
        } else {
            first = true;
        }
        if next_row[k - 1].is_ascii_digit() && next_row[k + 1].is_ascii_digit() {
            placeholder_num += &next_row[k - 1].to_string();
            placeholder_num += &next_row[k].to_string();
            placeholder_num += &next_row[k + 1].to_string();
        } else if next_row[k - 1].is_ascii_digit() && next_row[k - 2].is_ascii_digit() {
            placeholder_num += &next_row[k - 2].to_string();
            placeholder_num += &next_row[k - 1].to_string();
            placeholder_num += &next_row[k].to_string();
        } else if next_row[k - 1].is_ascii_digit() {
            placeholder_num += &next_row[k - 1].to_string();
            placeholder_num += &next_row[k].to_string();
        } else if next_row[k + 1].is_ascii_digit() && next_row[k + 2].is_ascii_digit() {
            placeholder_num += &next_row[k].to_string();
            placeholder_num += &next_row[k + 1].to_string();
            placeholder_num += &next_row[k + 2].to_string();
        } else if next_row[k + 1].is_ascii_digit() {
            placeholder_num += &next_row[k].to_string();
            placeholder_num += &next_row[k + 1].to_string();
        } else {
            placeholder_num += &next_row[k].to_string();
        }
        if second {
            let temp = placeholder_num.parse::<i32>().unwrap();
            if temp != num_index_row {
                second_num = temp;
            }
            placeholder_num.clear();
        } else {
            first_num = placeholder_num.parse::<i32>().unwrap();
            placeholder_num.clear();
        }
    }
    // Prev Row
    if prev_row[j].is_ascii_digit() {
        if first {
            second = true;
        } else {
            first = true;
        }
        if prev_row[j - 1].is_ascii_digit() && prev_row[j + 1].is_ascii_digit() {
            placeholder_num += &prev_row[j - 1].to_string();
            placeholder_num += &prev_row[j].to_string();
            placeholder_num += &prev_row[j + 1].to_string();
        } else if prev_row[j - 1].is_ascii_digit() && prev_row[j - 2].is_ascii_digit() {
            placeholder_num += &prev_row[j - 2].to_string();
            placeholder_num += &prev_row[j - 1].to_string();
            placeholder_num += &prev_row[j].to_string();
        } else if prev_row[j - 1].is_ascii_digit() {
            placeholder_num += &prev_row[j - 1].to_string();
            placeholder_num += &prev_row[j].to_string();
        } else if prev_row[j + 1].is_ascii_digit() && prev_row[j + 2].is_ascii_digit() {
            placeholder_num += &prev_row[j].to_string();
            placeholder_num += &prev_row[j + 1].to_string();
            placeholder_num += &prev_row[j + 2].to_string();
        } else if prev_row[j + 1].is_ascii_digit() {
            placeholder_num += &prev_row[j].to_string();
            placeholder_num += &prev_row[j + 1].to_string();
        } else {
            placeholder_num += &prev_row[j].to_string();
        }
        if second {
            second_num = placeholder_num.parse::<i32>().unwrap();
            placeholder_num.clear();
        } else {
            first_num = placeholder_num.parse::<i32>().unwrap();
            placeholder_num.clear();
        }
    }
    if prev_row[index].is_ascii_digit() {
        if first {
            second = true;
        } else {
            first = true;
        }
        if prev_row[index - 1].is_ascii_digit() && prev_row[index + 1].is_ascii_digit() {
            placeholder_num += &prev_row[index - 1].to_string();
            placeholder_num += &prev_row[index].to_string();
            placeholder_num += &prev_row[index + 1].to_string();
        } else if prev_row[index - 1].is_ascii_digit() && prev_row[index - 2].is_ascii_digit() {
            placeholder_num += &prev_row[index - 2].to_string();
            placeholder_num += &prev_row[index - 1].to_string();
            placeholder_num += &prev_row[index].to_string();
        } else if prev_row[index - 1].is_ascii_digit() {
            placeholder_num += &prev_row[index - 1].to_string();
            placeholder_num += &prev_row[index].to_string();
        } else if prev_row[index + 1].is_ascii_digit() && prev_row[index + 2].is_ascii_digit() {
            placeholder_num += &prev_row[index].to_string();
            placeholder_num += &prev_row[index + 1].to_string();
            placeholder_num += &prev_row[index + 2].to_string();
        } else if prev_row[index + 1].is_ascii_digit() {
            placeholder_num += &prev_row[index].to_string();
            placeholder_num += &prev_row[index + 1].to_string();
        } else {
            placeholder_num += &prev_row[index].to_string();
        }
        if second {
            let temp = placeholder_num.parse::<i32>().unwrap();
            if temp != first_num {
                second_num = temp;
            }
            placeholder_num.clear();
        } else {
            first_num = placeholder_num.parse::<i32>().unwrap();
            placeholder_num.clear();
        }
    }
    if prev_row[k].is_ascii_digit() {
        if first {
            second = true;
        } else {
            first = true;
        }
        if prev_row[k - 1].is_ascii_digit() && prev_row[k + 1].is_ascii_digit() {
            placeholder_num += &prev_row[k - 1].to_string();
            placeholder_num += &prev_row[k].to_string();
            placeholder_num += &prev_row[k + 1].to_string();
        } else if prev_row[k - 1].is_ascii_digit() && prev_row[k - 2].is_ascii_digit() {
            placeholder_num += &prev_row[k - 2].to_string();
            placeholder_num += &prev_row[k - 1].to_string();
            placeholder_num += &prev_row[k].to_string();
        } else if prev_row[k - 1].is_ascii_digit() {
            placeholder_num += &prev_row[k - 1].to_string();
            placeholder_num += &prev_row[k].to_string();
        } else if prev_row[k + 1].is_ascii_digit() && prev_row[k + 2].is_ascii_digit() {
            placeholder_num += &prev_row[k].to_string();
            placeholder_num += &prev_row[k + 1].to_string();
            placeholder_num += &prev_row[k + 2].to_string();
        } else if prev_row[k + 1].is_ascii_digit() {
            placeholder_num += &prev_row[k].to_string();
            placeholder_num += &prev_row[k + 1].to_string();
        } else {
            placeholder_num += &prev_row[k].to_string();
        }
        if second {
            let temp = placeholder_num.parse::<i32>().unwrap();
            if temp != first_num {
                second_num = temp;
            }
            placeholder_num.clear();
        } else {
            first_num = placeholder_num.parse::<i32>().unwrap();
            placeholder_num.clear();
        }
    }
    if first && second {
        println!("first num: {}\tsecond num: {}", first_num, second_num);
        (true, first_num * second_num)
    } else {
        (false, 0)
    }
}

fn search_for_numbers_last_row(
    main_row: &Vec<char>,
    prev_row: &Vec<char>,
    index: usize,
) -> (bool, i32) {
    let mut first: bool = false;
    let mut second: bool = false;
    let mut first_num: i32 = 0;
    let mut second_num: i32 = 0;

    //Main Row
    let mut j: usize = index - 1;
    let mut k: usize = index + 1;
    let mut placeholder_num: String = String::new();
    if main_row[j].is_ascii_digit() {
        first = true;
        while j > 0 && main_row[j].is_ascii_digit() {
            placeholder_num.insert(0, main_row[j]);
            j -= 1;
        }
        first_num = placeholder_num.parse::<i32>().unwrap();
        j = index - 1;
        placeholder_num.clear();
    }
    if main_row[k].is_ascii_digit() {
        if first {
            second = true;
        } else {
            first = true;
        }
        while j > 0 && main_row[k].is_ascii_digit() {
            placeholder_num.insert(0, main_row[j]);
            k += 1;
        }
        if second {
            second_num = placeholder_num.parse::<i32>().unwrap();
        } else {
            first_num = placeholder_num.parse::<i32>().unwrap();
        }
        k = index + 1;
        placeholder_num.clear();
    }
    // Prev Row
    if prev_row[j].is_ascii_digit() {
        if first {
            second = true;
        } else {
            first = true;
        }
        if prev_row[j - 1].is_ascii_digit() && prev_row[j + 1].is_ascii_digit() {
            placeholder_num += &prev_row[j - 1].to_string();
            placeholder_num += &prev_row[j].to_string();
            placeholder_num += &prev_row[j + 1].to_string();
        } else if prev_row[j - 1].is_ascii_digit() && prev_row[j - 2].is_ascii_digit() {
            placeholder_num += &prev_row[j - 2].to_string();
            placeholder_num += &prev_row[j - 1].to_string();
            placeholder_num += &prev_row[j].to_string();
        } else if prev_row[j - 1].is_ascii_digit() {
            placeholder_num += &prev_row[j - 1].to_string();
            placeholder_num += &prev_row[j].to_string();
        } else if prev_row[j + 1].is_ascii_digit() && prev_row[j + 2].is_ascii_digit() {
            placeholder_num += &prev_row[j].to_string();
            placeholder_num += &prev_row[j + 1].to_string();
            placeholder_num += &prev_row[j + 2].to_string();
        } else if prev_row[j + 1].is_ascii_digit() {
            placeholder_num += &prev_row[j].to_string();
            placeholder_num += &prev_row[j + 1].to_string();
        } else {
            placeholder_num += &prev_row[j].to_string();
        }
        if second {
            second_num = placeholder_num.parse::<i32>().unwrap();
        } else {
            first_num = placeholder_num.parse::<i32>().unwrap();
        }
    }
    if prev_row[index].is_ascii_digit() {
        if first {
            second = true;
        } else {
            first = true;
        }
        if prev_row[index - 1].is_ascii_digit() && prev_row[index + 1].is_ascii_digit() {
            placeholder_num += &prev_row[index - 1].to_string();
            placeholder_num += &prev_row[index].to_string();
            placeholder_num += &prev_row[index + 1].to_string();
        } else if prev_row[index - 1].is_ascii_digit() && prev_row[index - 2].is_ascii_digit() {
            placeholder_num += &prev_row[index - 2].to_string();
            placeholder_num += &prev_row[index - 1].to_string();
            placeholder_num += &prev_row[index].to_string();
        } else if prev_row[index - 1].is_ascii_digit() {
            placeholder_num += &prev_row[index - 1].to_string();
            placeholder_num += &prev_row[index].to_string();
        } else if prev_row[index + 1].is_ascii_digit() && prev_row[index + 2].is_ascii_digit() {
            placeholder_num += &prev_row[index].to_string();
            placeholder_num += &prev_row[index + 1].to_string();
            placeholder_num += &prev_row[index + 2].to_string();
        } else if prev_row[index + 1].is_ascii_digit() {
            placeholder_num += &prev_row[index].to_string();
            placeholder_num += &prev_row[index + 1].to_string();
        } else {
            placeholder_num += &prev_row[index].to_string();
        }
        if second {
            second_num = placeholder_num.parse::<i32>().unwrap();
        } else {
            first_num = placeholder_num.parse::<i32>().unwrap();
        }
    }
    if prev_row[k].is_ascii_digit() {
        if first {
            second = true;
        } else {
            first = true;
        }
        if prev_row[k - 1].is_ascii_digit() && prev_row[k + 1].is_ascii_digit() {
            placeholder_num += &prev_row[k - 1].to_string();
            placeholder_num += &prev_row[k].to_string();
            placeholder_num += &prev_row[k + 1].to_string();
        } else if prev_row[k - 1].is_ascii_digit() && prev_row[k - 2].is_ascii_digit() {
            placeholder_num += &prev_row[k - 2].to_string();
            placeholder_num += &prev_row[k - 1].to_string();
            placeholder_num += &prev_row[k].to_string();
        } else if prev_row[k - 1].is_ascii_digit() {
            placeholder_num += &prev_row[k - 1].to_string();
            placeholder_num += &prev_row[k].to_string();
        } else if prev_row[k + 1].is_ascii_digit() && prev_row[k + 2].is_ascii_digit() {
            placeholder_num += &prev_row[k].to_string();
            placeholder_num += &prev_row[k + 1].to_string();
            placeholder_num += &prev_row[k + 2].to_string();
        } else if prev_row[k + 1].is_ascii_digit() {
            placeholder_num += &prev_row[k].to_string();
            placeholder_num += &prev_row[k + 1].to_string();
        } else {
            placeholder_num += &prev_row[k].to_string();
        }
        if second {
            second_num = placeholder_num.parse::<i32>().unwrap();
        } else {
            first_num = placeholder_num.parse::<i32>().unwrap();
        }
    }
    if first && second {
        (true, first_num * second_num)
    } else {
        (false, 0)
    }
}

fn row_sum(arr: &Vec<&str>, index: usize) -> i32 {
    let mut sum: i32 = 0;
    match index {
        i if i == 0 => {
            let char_arr = arr[i].chars().collect::<Vec<char>>();
            let next_char_arr = arr[i + 1].chars().collect::<Vec<char>>();
            let mut j: usize = 0;
            while j < char_arr.len() {
                if char_arr[j].is_ascii_digit() {
                    let mut is_valid: bool = false;
                    if j != 0 && !char_arr[j - 1].is_ascii_digit() && char_arr[j - 1] != '.' {
                        is_valid = true;
                    }
                    if j != 0
                        && !next_char_arr[j - 1].is_ascii_digit()
                        && next_char_arr[j - 1] != '.'
                    {
                        is_valid = true;
                    }
                    let mut number: String = String::new();
                    while j < char_arr.len() && char_arr[j].is_ascii_digit() {
                        if char_arr[j].is_ascii_digit() {
                            number += &char_arr[j].to_string();
                        }
                        if !next_char_arr[j].is_ascii_digit() && next_char_arr[j] != '.' {
                            is_valid = true;
                        }
                        j += 1;
                    }
                    if j < char_arr.len() && !char_arr[j].is_ascii_digit() && char_arr[j] != '.' {
                        is_valid = true;
                    }
                    if j < next_char_arr.len()
                        && !next_char_arr[j].is_ascii_digit()
                        && next_char_arr[j] != '.'
                    {
                        is_valid = true;
                    }
                    if is_valid && !number.is_empty() {
                        sum += number.parse::<i32>().unwrap();
                    }
                    j += 1;
                } else {
                    j += 1
                }
            }
        }
        i if i > 0 && i < arr.len() - 1 => {
            let char_arr = arr[i].chars().collect::<Vec<char>>();
            let next_char_arr = arr[i + 1].chars().collect::<Vec<char>>();
            let prev_char_arr = arr[i - 1].chars().collect::<Vec<char>>();
            let mut j: usize = 0;
            while j < char_arr.len() {
                if char_arr[j].is_ascii_digit() {
                    let mut is_valid: bool = false;
                    if j != 0 && !char_arr[j - 1].is_ascii_digit() && char_arr[j - 1] != '.' {
                        is_valid = true;
                    }
                    if j != 0
                        && !next_char_arr[j - 1].is_ascii_digit()
                        && next_char_arr[j - 1] != '.'
                    {
                        is_valid = true;
                    }
                    if j != 0
                        && !prev_char_arr[j - 1].is_ascii_digit()
                        && prev_char_arr[j - 1] != '.'
                    {
                        is_valid = true;
                    }
                    let mut number: String = String::new();
                    while j < char_arr.len() && char_arr[j].is_ascii_digit() {
                        if char_arr[j].is_ascii_digit() {
                            number += &char_arr[j].to_string();
                        }
                        if !next_char_arr[j].is_ascii_digit() && next_char_arr[j] != '.' {
                            is_valid = true;
                        }
                        if !prev_char_arr[j].is_ascii_digit() && prev_char_arr[j] != '.' {
                            is_valid = true;
                        }
                        j += 1;
                    }
                    if j < char_arr.len() && !char_arr[j].is_ascii_digit() && char_arr[j] != '.' {
                        is_valid = true;
                    }
                    if j < next_char_arr.len()
                        && !next_char_arr[j].is_ascii_digit()
                        && next_char_arr[j] != '.'
                    {
                        is_valid = true;
                    }
                    if j < prev_char_arr.len()
                        && !prev_char_arr[j].is_ascii_digit()
                        && prev_char_arr[j] != '.'
                    {
                        is_valid = true;
                    }
                    if is_valid && !number.is_empty() {
                        sum += number.parse::<i32>().unwrap();
                    }
                    j += 1;
                } else {
                    j += 1
                }
            }
        }
        i if i == arr.len() - 1 => {
            let char_arr = arr[i].chars().collect::<Vec<char>>();
            let prev_char_arr = arr[i - 1].chars().collect::<Vec<char>>();
            let mut j: usize = 0;
            while j < char_arr.len() {
                if char_arr[j].is_ascii_digit() {
                    let mut is_valid: bool = false;
                    if j != 0 && !char_arr[j - 1].is_ascii_digit() && char_arr[j - 1] != '.' {
                        is_valid = true;
                    }
                    if j != 0
                        && !prev_char_arr[j - 1].is_ascii_digit()
                        && prev_char_arr[j - 1] != '.'
                    {
                        is_valid = true;
                    }
                    let mut number: String = String::new();
                    while char_arr[j].is_ascii_digit() {
                        if char_arr[j].is_ascii_digit() {
                            number += &char_arr[j].to_string();
                        }
                        if !prev_char_arr[j].is_ascii_digit() && prev_char_arr[j] != '.' {
                            is_valid = true;
                        }
                        j += 1;
                    }
                    if j < char_arr.len() && !char_arr[j].is_ascii_digit() && char_arr[j] != '.' {
                        is_valid = true;
                    }
                    if j + 1 < prev_char_arr.len()
                        && !prev_char_arr[j].is_ascii_digit()
                        && prev_char_arr[j] != '.'
                    {
                        is_valid = true;
                    }
                    if is_valid && !number.is_empty() {
                        sum += number.parse::<i32>().unwrap();
                    }
                } else {
                    j += 1
                }
            }
        }
        _ => (),
    }
    sum
}

#[test]
fn test_engine_sum_real_data() {
    let file_path = String::from("data/day-3.txt");
    match sum_engine_parts(&file_path, true) {
        Either::Left(_) => println!("failed"),

        Either::Right(val) => {
            let sum = val.unwrap();
            assert_eq!(sum[sum.len() - 1], 1532);

            assert_eq!(sum[123], 6384);
        }
    }
}
