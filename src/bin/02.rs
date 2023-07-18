pub enum Game {
    Rock,
    Paper,
    Scissors
}

pub enum GameResult {
    Win,
    Lose,
    Draw
}

pub fn calculate_bonus_score(play: &Game) -> u32 {
    match play {
        Game::Rock => 1,
        Game::Paper => 2,
        Game::Scissors => 3,
    }
}

pub fn choose_play_to_win(play: &Game) -> Game {
    match play {
        // in format player1 => player2 (winner)
        Game::Rock => Game::Paper,
        Game::Paper => Game::Scissors,
        Game::Scissors => Game::Rock
    }
}

pub fn choose_play_to_lose(play: &Game) -> Game {
    match play {
        // in format player1 => player2 (loser)
        Game::Rock => Game::Scissors,
        Game::Paper => Game::Rock,
        Game::Scissors => Game::Paper
    }

}

pub fn assign_type(player: &char) -> Game {
    match player {
        'X' | 'A' => Game::Rock,
        'Y' | 'B' => Game::Paper,
        'Z' | 'C'  => Game::Scissors,
        _ => panic!("Invalid input: {}", player)
    }
}

pub fn get_result_of_game(first: &Game, second: &Game) -> GameResult {
    // decide whether the 2nd player wins, loses or draws
    match (first, second) {
        (Game::Rock, Game::Rock) => GameResult::Draw,
        (Game::Paper, Game::Paper) => GameResult::Draw,
        (Game::Scissors, Game::Scissors) => GameResult::Draw,
        (Game::Rock, Game::Paper) => GameResult::Win,
        (Game::Paper, Game::Scissors) => GameResult::Win,
        (Game::Scissors, Game::Rock) => GameResult::Win,
        (Game::Rock, Game::Scissors) => GameResult::Lose,
        (Game::Paper, Game::Rock) => GameResult::Lose,
        (Game::Scissors, Game::Paper) => GameResult::Lose,
    }
}

pub fn get_wanted_result(second: &char) -> GameResult {
    match second {
        'X' => GameResult::Lose,
        'Y' => GameResult::Draw,
        'Z' => GameResult::Win,
        _ => panic!("Invalid input: {}", second)
    }
}

pub fn get_pair_of_letters(line: &str) -> Option<(char, char)> {
    if line.len() != 3 {
        return None;
    }
    let first = line.chars().nth(0)?;
    let second = line.chars().nth(2)?;
    Some((first, second))
}


pub fn part_one(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;
    for line in input.lines() {
        match get_pair_of_letters(line) {
            Some((first, second)) => {
                let first = assign_type(&first);
                let second = assign_type(&second);
                sum += calculate_bonus_score(&second);
                sum += match get_result_of_game(&first, &second) {
                    GameResult::Win => 6,
                    GameResult::Draw => 3,
                    GameResult::Lose => 0
                }
            },
            None => {}
        };
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;

    for line in input.lines() {
        match get_pair_of_letters(line) {
            Some((first, second)) => {
                let result = get_wanted_result(&second);

                let first = assign_type(&first);

                match result {
                    GameResult::Win     => sum += 6 + calculate_bonus_score(&choose_play_to_win(&first)),
                    GameResult::Draw    => sum += 3 + calculate_bonus_score(&first),
                    GameResult::Lose    => sum += calculate_bonus_score(&choose_play_to_lose(&first))
                }
            },
            None => {}
        }
    }

    Some(sum)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
