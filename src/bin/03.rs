use std::collections::HashSet;

pub fn evaluate_score(letter: &char) -> u32 {
    if *letter >= 'a' && *letter <= 'z' {
        (*letter as u8 - b'a' + 1) as u32
    } else if *letter >= 'A' && *letter <= 'Z' {
        (*letter as u8 - 38) as u32
    } else {
        panic!("ERROR: Could not compute priority for letter {}", letter);
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(
                |line| {
                    let len = line.len();
                    let (first_h_str, second_h_str) = line.split_at(len / 2);

                    let first_h: HashSet<_> = first_h_str.chars().collect::<HashSet<_>>();
                    let second_h: HashSet<_> = second_h_str.chars().collect::<HashSet<_>>();

                    first_h
                        .into_iter()
                        .map(
                            |c| {
                                match second_h.get(&c) {
                                    Some(_) => evaluate_score(&c),
                                    None => 0,
                                }
                            }
                        )
                        .sum::<u32>()
                }
            )
            .sum()
    )

}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(
            |triple| {
                let mut common_chars: HashSet<char> = HashSet::new();
                for (i, line) in triple.iter().enumerate() {
                    let line_chars: HashSet<_> = line.chars().collect();
                    if i == 0 {
                        common_chars = line_chars;
                    } else {
                        common_chars = common_chars.intersection(&line_chars).cloned().collect();
                    }
                }
                common_chars
                    .iter()
                    .map(|c| evaluate_score(&c))
                    .sum::<u32>()
            }
        )
        .sum()
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_multiple_common_chars() {
        assert_eq!(part_one(&"zawMqvLMZHhHMvwLHjbvcjnnSBnvTQFnza"), Some(49));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }

    #[test]
    fn test_evaluating_score() {
        assert_eq!(evaluate_score(&'a'), 1);
        assert_eq!(evaluate_score(&'v'), 22);
        assert_eq!(evaluate_score(&'z'), 26);
        assert_eq!(evaluate_score(&'A'), 27);
        assert_eq!(evaluate_score(&'Z'), 52);
    }
}
