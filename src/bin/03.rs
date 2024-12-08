use std::cmp;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    const MUL_OPENER: &str = "mul(";
    const MUL_CLOSER: &str = ")";
    const MUL_DELIMITER: &str = ",";
    const NUM_ARGUMENTS: usize = 2;
    const MAX_ARG_DIGITS: usize = 3;

    let mut multiplies = Vec::<u32>::new();

    for index in input.match_indices(MUL_OPENER).map(|(idx, _)| idx) {
        let start_index = cmp::min(index + MUL_OPENER.len(), input.len());
        let end_index = cmp::min(
            start_index
                + (MAX_ARG_DIGITS * NUM_ARGUMENTS)
                + (MUL_DELIMITER.len() * (NUM_ARGUMENTS - 1))
                + MUL_CLOSER.len(),
            input.len(),
        );

        let closer_index = match input[start_index..end_index].find(MUL_CLOSER) {
            Some(offset) => cmp::min(offset + start_index, input.len()),
            None => continue,
        };

        let args: Vec<u32> = input[start_index..closer_index]
            .split(MUL_DELIMITER)
            .filter_map(|arg| arg.parse::<u32>().ok())
            .collect();

        if args.len() == NUM_ARGUMENTS {
            multiplies.push(args.iter().product());
        }
    }

    Some(multiplies.into_iter().sum::<u32>())
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
