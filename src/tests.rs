use super::*;

#[test]
fn test_get_calibrated_value_sum() {
    let file_path = String::from("data/data.txt");
    let result = get_calibrated_value_sum(&file_path);

    assert!(result.is_ok());
    let res = result.unwrap();
    
    assert_eq!(res[0], 55);

    assert_eq!(res[res.len()-1], 55);

    assert_eq!(res[6], 88);

    assert_eq!(res.len(), 1000);
}

#[test]
fn test_get_calibrated_value_sum_part_two() {
    let file_path = String::from("data/data.txt");
    let result = get_calibrated_value_sum_part_two(&file_path);

    assert!(result.is_ok());
    let res = result.unwrap();
    
    assert_eq!(res[0], 55);

    assert_eq!(res[4], 27);

    assert_eq!(res[16], 88);

    assert_eq!(res[432], 98);

    assert_eq!(res[289], 42);

    assert_eq!(res.len(), 1000);
}