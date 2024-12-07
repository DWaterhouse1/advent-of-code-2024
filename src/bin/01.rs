use std::{collections::HashMap, str};

advent_of_code::solution!(1);

fn parse_and_unzip(input: &str) -> Option<(Vec<i64>, Vec<i64>)> {
    const ROW_DELIMITER: &str = "\n";
    const PAIR_DELIMITER: &str = "   ";

    Some(
        input
            .split(ROW_DELIMITER)
            .take_while(|x| !x.is_empty())
            .map(|x| {
                let pair = x.split_once(PAIR_DELIMITER)?;
                Some((pair.0.parse::<i64>().ok()?, pair.1.parse::<i64>().ok()?))
            })
            .collect::<Option<Vec<_>>>()?
            .into_iter()
            .unzip(),
    )
}

pub fn part_one(input: &str) -> Option<u32> {
    let (mut lhs, mut rhs) = parse_and_unzip(input)?;

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
    let (lhs, rhs) = parse_and_unzip(input)?;

    let mut id_counts = HashMap::<i64, u32>::new();

    for x in rhs {
        match id_counts.get(&x) {
            Some(count) => {
                id_counts.insert(x, count + 1);
            }
            None => {
                id_counts.insert(x, 1);
            }
        }
    }

    Some(lhs.into_iter().fold(0, |acc, x| match id_counts.get(&x) {
        Some(count) => acc + (x as u32) * count,
        None => acc,
    }))
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
        assert_eq!(result, Some(31));
    }
}
