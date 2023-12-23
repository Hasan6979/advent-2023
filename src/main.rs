pub mod one;
pub mod utils;

fn main() {
    let mut sum: u32 = 0;
    // File hosts.txt must exist in the current path
    if let Ok(lines) = utils::file_reader::read_lines("input/problem-1.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            sum += one::find_calibration_value(&line);
        }
    } else {
        println!("No such filename");
    }
    println!("Calibration value: {}", sum);
}
