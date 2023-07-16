pub fn get_sum_of_calories(input: &str) -> Vec<u32> {
    let mut sums: Vec<u32> = input
        .split("\n\n")
        .map(|group| {
            group
                .split_whitespace()
                .map(|word| word.parse().unwrap_or(0))
                .sum()
        })
        .collect();

    sums.sort();

    sums
}

pub fn part_one(input: &str) -> Option<u32> {
    let calories = get_sum_of_calories(input);

    //Some(*calories.last().unwrap())
    calories.last()
        .and_then(|&max| Some(max))
}

pub fn part_two(input: &str) -> Option<u32> {
    let calories = get_sum_of_calories(input);

    let len = calories.len();

    println!("{}", calories[len-1]);
    println!("{}", calories[len-2]);
    println!("{}", calories[len-3]);
    Some(calories[len-1] + calories[len-2] + calories[len-3])
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}
