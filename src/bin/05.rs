use itertools::Itertools;

pub const MAX_STACKS: usize = 9;

pub fn parse_stacks(input: &str) -> Vec<(usize, char)> {
    input
                        .lines()
                        .rev()
                        // skip the line with numbers
                        .skip(1) 
                        .map(|line| {
                            line.chars()
                                .skip(1)
                                .enumerate()
                                // chars will be on indexes 0..9
                                .filter_map(|(i, c)| {
                                    match i {
                                        i if i % 4 == 0 && c.is_alphabetic() => Some((i / 4, c)),
                                        _ => None,
                                    }
                                })
                                .collect::<Vec<(usize, char)>>()
                        })
                        .flatten()
                        .collect::<Vec<(usize, char)>>()
}

/// Parse the procedures in the same form to the vector holding numbers
pub fn parse_procedures(procedures: &str) -> Vec<u32> {
    procedures
        .replace("move", "")
        .replace("from", "")
        .replace("to", "")
        .split_whitespace()
        .filter_map(|elem| elem.parse::<u32>().ok())
        .collect()
}

pub fn get_message(stacks: &[Vec<char>; MAX_STACKS]) -> String {
    let mut message: String = String::new();

    for i in 0..MAX_STACKS {
        message.push(
            match stacks[i].last().cloned() {
                Some(c) => c,
                _ => continue,
            }
        );
    }

    message
}

pub fn get_input(input: &str) -> (&str, &str) {
    match input.split("\n\n").collect_tuple() {
                Some((a, b)) => (a, b),
                _ => panic!("ERROR: Could not parse stacks and procedures."),
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let (stacks_str, procedures_str) = get_input(input);

    let mut stacks: [Vec<char>; MAX_STACKS] = Default::default();
    for (i, c) in parse_stacks(stacks_str) {
        stacks[i].push(c);
    }

    let procedures = parse_procedures(procedures_str);

    for chunk in procedures.chunks_exact(3) {
        let count = chunk[0];
        let from_stack = chunk[1] as usize;
        let to_stack = chunk[2] as usize;

        for _ in 0..count { 
            let to_push = match stacks[from_stack-1].pop() {
                Some(c) => c,
                    _ => panic!("ERROR: Could not pop out of empty stack."),
            };
            stacks[to_stack-1].push(to_push);
        }
    }

        Some(get_message(&stacks))
}

pub fn part_two(input: &str) -> Option<String> {
    let (stacks_str, procedures_str) = get_input(input);

    let mut stacks: [Vec<char>; MAX_STACKS] = Default::default();
    for (i, c) in parse_stacks(stacks_str) {
        stacks[i].push(c);
    }

    let procedures = parse_procedures(procedures_str);

    for chunk in procedures.chunks_exact(3) {
        let count = chunk[0];
        let from_stack = chunk[1] as usize;
        let to_stack = chunk[2] as usize;

        let mut to_push_vec: Vec<char> = vec![];

        // fill crates into the vector to_push_vec
        for _ in 0..count { 
            let to_push = match stacks[from_stack-1].pop() {
                Some(c) => c,
                    _ => panic!("ERROR: Could not pop out of empty stack."),
            };
            to_push_vec.push(to_push);
        }

        // empty out the vector so that the crates are in the right order
        for _ in 0..count {
            let to_push = match to_push_vec.pop() {
                Some(c) => c,
                _ => panic!("ERROR: Could not pop out of empty stack."),
            };

            stacks[to_stack-1].push(to_push);
        }
    }

    Some(get_message(&stacks))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), None);
    }

    #[test]
    fn test_parse_procedures() {
        assert_eq!(parse_procedures(&"move 12 from 2 to 1"), [12, 2, 1])
    }
}
