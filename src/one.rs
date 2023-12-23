use std::vec;

const NULL: char = '\0';

pub fn find_calibration_value(line: &str) -> u32 {
    let mut cal_val: Vec<char> = Vec::new();
    for c in line.chars() {
        if c.is_ascii_digit() {
            cal_val.push(c);
        } else {
        }
    }

    let joined_cal =
        (*cal_val.first().unwrap()).to_string() + &(*cal_val.last().unwrap()).to_string();
    joined_cal.parse().unwrap()
}

#[cfg(test)]
mod test {
    use crate::one::find_calibration_value;

    #[test]
    fn calculate_digits() {
        assert_eq!(12, find_calibration_value("1abc2"));
        assert_eq!(38, find_calibration_value("pqr3stu8vwx"));
        assert_eq!(15, find_calibration_value("a1b2c3d4e5f"));
        assert_eq!(77, find_calibration_value("treb7uchet"));
    }
}
