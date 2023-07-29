const MARKER_PART_ONE: usize = 4;
const MARKER_PART_TWO: usize = 14;

use std::collections::HashSet;

pub fn get_first_marker(input: &str, size: usize) -> usize {
    let windows: Vec<&[u8]> = input.as_bytes().windows(size).collect();

    windows
        .iter()
        .enumerate()
        .find_map(|(index, window)| {
            let mut seen_bytes = HashSet::new();
            let mut contains_duplicates = false;

            for &byte in *window {
                if seen_bytes.insert(byte) == false {
                    contains_duplicates = true;
                    break;
                }
            }

            if contains_duplicates == false {
                Some(index + 4)
            } else {
                None
            }
        })
        .unwrap()
}

pub fn part_one(input: &str) -> Option<usize> {
    Some(get_first_marker(input, MARKER_PART_ONE))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(get_first_marker(input, MARKER_PART_TWO))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(&"mjqjpqmgbljsphdztnvjfqwrcgsmlb"), Some(7));
        assert_eq!(part_one(&"bvwbjplbgvbhsrlpgdmjqwftvncz"), Some(5));
        assert_eq!(part_one(&"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), Some(11));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), None);
    }
}