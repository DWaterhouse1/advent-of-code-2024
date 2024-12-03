use std::str;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    const ROW_DELIMITER: &str = "\n";
    const PAIR_DELIMITER: &str = "   ";

    let (mut lhs, mut rhs): (Vec<i64>, Vec<i64>) = input
        .split(ROW_DELIMITER)
        .take_while(|x| !x.is_empty())
        .map(|x| {
            let pair = x.split_once(PAIR_DELIMITER)?;
            Some((pair.0.parse::<i64>().ok()?, pair.1.parse::<i64>().ok()?))
        })
        .collect::<Option<Vec<_>>>()?
        .into_iter()
        .unzip();

    lhs.sort();
    rhs.sort();

    let total_distance = lhs
        .into_iter()
        .zip(rhs)
        .map(|(lhs, rhs)| (lhs - rhs).unsigned_abs() as u32)
        .sum();

    Some(total_distance)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
