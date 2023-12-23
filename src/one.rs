use std::vec;

const NULL: char = '\0';

pub fn find_calibration_value(line: &str) -> u32 {
    let mut cal_val: Vec<char> = vec![NULL, NULL];
    for c in line.chars() {
        if c <= '9' {
            match cal_val[0] {
                NULL => cal_val[0] = c,
                _ => cal_val[1] = c,
            }
        }
    }
    if cal_val[1] == NULL {
        cal_val[1] = cal_val[0];
    }
    let joined_cal: String = cal_val.iter().collect();
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
