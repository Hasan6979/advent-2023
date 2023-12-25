use crate::utils::hash_checker::check_hash;
use std::collections::VecDeque;

pub fn find_calibration_value(line: &str) -> u32 {
    let mut cal_val: Vec<char> = Vec::with_capacity(2);
    let mut deque: VecDeque<char> = VecDeque::with_capacity(5);
    for c in line.chars() {
        if c.is_ascii_digit() {
            cal_val.push(c);
        } else {
            deque.push_back(c);
            if let 3..=5 = deque.len() {
                check_hash(&mut deque, &mut cal_val)
            }
        }
    }

    let joined_cal =
        (*cal_val.first().unwrap()).to_string() + &(*cal_val.last().unwrap()).to_string();
    joined_cal.parse().unwrap()
}

#[cfg(test)]
mod test {
    use crate::days::one::find_calibration_value;

    #[test]
    fn calculate_digits() {
        assert_eq!(29, find_calibration_value("two1nine"));
        assert_eq!(83, find_calibration_value("eightwothree"));
        assert_eq!(13, find_calibration_value("abcone2threexyz"));
        assert_eq!(24, find_calibration_value("xtwone3four"));
        assert_eq!(42, find_calibration_value("4nineeightseven2"));
        assert_eq!(14, find_calibration_value("zoneight234"));
        assert_eq!(76, find_calibration_value("7pqrstsixteen"));
    }
}
