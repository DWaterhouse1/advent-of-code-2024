use std::cmp;

advent_of_code::solution!(3);

fn calculate_mults(input: &str) -> Option<u32> {
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

fn find_disablement_regions(input: &str) -> Vec<(usize, usize)> {
    const DO: &str = "do()";
    const DONT: &str = "don't()";

    let mut disablement_regions = Vec::<(usize, usize)>::new();
    let mut current_index: usize = 0;

    while let Some(dont_pos) = input[current_index..].find(DONT) {
        let post_dont = current_index + dont_pos + DONT.len();

        if let Some(do_pos) = input[post_dont..].find(DO) {
            let do_idx = post_dont + do_pos;
            disablement_regions.push((post_dont, do_idx));
            current_index = do_idx + DO.len();
        } else {
            disablement_regions.push((post_dont, input.len()));
            break;
        }
    }

    disablement_regions
}

fn remove_disabled_regions(input: &str, regions: &[(usize, usize)]) -> String {
    let mut result = String::with_capacity(input.len());
    let mut last_end = 0;

    for &(start, end) in regions {
        result.push_str(&input[last_end..start]);
        last_end = end;
    }

    result.push_str(&input[last_end..]);
    result
}

pub fn part_one(input: &str) -> Option<u32> {
    calculate_mults(input)
}

pub fn part_two(input: &str) -> Option<u32> {
    let disablement_regions = find_disablement_regions(input);
    let processed_input = remove_disabled_regions(input, &disablement_regions);
    calculate_mults(processed_input.as_str())
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
        assert_eq!(result, Some(48));
    }
}
