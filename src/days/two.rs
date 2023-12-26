use std::collections::HashMap;

pub struct Game {
    sum_possible_games: usize,
    power_of_cubes: usize,
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}

impl Game {
    pub fn new() -> Game {
        Game {
            sum_possible_games: 0,
            power_of_cubes: 0,
        }
    }

    pub fn add_game(&mut self, s: &str, max_balls: &HashMap<String, usize>) {
        let split_str = s.split_whitespace().collect::<Vec<_>>();
        let polished_str = &split_str[1..split_str.len()];
        let mut balls: HashMap<String, usize> = HashMap::with_capacity(3);

        let idx = (polished_str[0][0..(polished_str[0].len() - 1)])
            .parse::<usize>()
            .unwrap();

        for i in (2..polished_str.len() - 1).step_by(2) {
            self.insert_balls(polished_str, &mut balls, i, 1);
        }
        self.insert_balls(polished_str, &mut balls, polished_str.len() - 1, 0);

        if self.is_game_possible(&balls, max_balls) {
            self.sum_possible_games += idx;
        }

        self.power_of_cubes += self.cube_balls(&balls);
    }

    fn insert_balls(
        &mut self,
        game_str: &[&str],
        balls: &mut HashMap<String, usize>,
        str_idx: usize,
        skip_chars: usize,
    ) {
        let key = &(game_str[str_idx][0..(game_str[str_idx].len() - skip_chars)]);
        let val = game_str[str_idx - 1].parse::<usize>().unwrap();
        let prev_val = balls.get(key);
        if prev_val.is_none() || *(prev_val.unwrap()) < val {
            balls.insert(key.to_string(), val);
        }
    }

    pub fn is_game_possible(
        &self,
        balls: &HashMap<String, usize>,
        max_balls: &HashMap<String, usize>,
    ) -> bool {
        (*(balls.get("green").unwrap()) <= *(max_balls.get("green").unwrap()))
            && (*(balls.get("red").unwrap()) <= *(max_balls.get("red").unwrap()))
            && (*(balls.get("blue").unwrap()) <= *(max_balls.get("blue").unwrap()))
    }

    pub fn get_sum_possible_games(&self) -> usize {
        self.sum_possible_games
    }

    pub fn get_power_of_cubes(&self) -> usize {
        self.power_of_cubes
    }

    fn cube_balls(&self, balls: &HashMap<String, usize>) -> usize {
        (*(balls.get("green").unwrap()))
            * (*(balls.get("red").unwrap()))
            * (*(balls.get("blue").unwrap()))
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::Game;

    #[test]
    fn test_string_parser() {
        let mut game = Game::new();
        let mut map = HashMap::with_capacity(3);
        map.insert(String::from("red"), 12);
        map.insert(String::from("green"), 13);
        map.insert(String::from("blue"), 14);
        game.add_game(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            &map,
        );
        game.add_game(
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            &map,
        );
        game.add_game(
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            &map,
        );
        game.add_game(
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            &map,
        );
        game.add_game(
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
            &map,
        );
        assert_eq!(8, game.get_sum_possible_games());
        assert_eq!(2286, game.get_power_of_cubes());
    }
}
