use std::collections::HashMap;

use crate::days::one;
use crate::days::two::Game;

pub mod days;
pub mod utils;

fn main() {
    let mut sum: u32 = 0;

    let mut game = Game::new();
    let mut max_balls = HashMap::with_capacity(3);
    max_balls.insert(String::from("red"), 12);
    max_balls.insert(String::from("green"), 13);
    max_balls.insert(String::from("blue"), 14);

    // File hosts.txt must exist in the current path
    if let Ok(lines) = utils::file_reader::read_lines("input/problem-2.txt") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            game.add_game(&line, &max_balls);
        }
    } else {
        println!("No such filename");
    }
    println!("Calibration value: {}", game.get_power_of_cubes());
}
