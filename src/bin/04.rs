use itertools::Itertools;

fn get_pair_of_slices(line: &str) -> (&str, &str) {
    let mut slices = line.split(",");
    let slice1 = slices.nth(0).unwrap();
    let slice2 = slices.nth(0).unwrap();

    (slice1, slice2)
}

fn parse_slice(slice: &str) -> (u32, u32) {
    slice.split("-").map(|s| s.parse::<u32>().unwrap()).collect_tuple().unwrap()
}

fn contains_slice(slice1: &str, slice2: &str) -> bool {
    let (start_1, end_1) = parse_slice(slice1);
    let (start_2, end_2) = parse_slice(slice2);

    (start_1 <= start_2 && end_1 >= end_2) || (start_2 <= start_1 && end_2 >= end_1)
}

fn overlaps_slice(slice1: &str, slice2: &str) -> bool {
    let (start_1, end_1) = parse_slice(slice1);
    let (start_2, end_2) = parse_slice(slice2);

    start_1 <= end_2 && end_1 >= start_2
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(
                |line| {
                    let (slice1, slice2) = get_pair_of_slices(line);

                    contains_slice(slice1, slice2) as u32
                }
            )
            .sum::<u32>()
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(
                |line| {
                    let (slice1, slice2) = get_pair_of_slices(line);

                    overlaps_slice(slice1, slice2) as u32
                }
            )
            .sum::<u32>()
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}
