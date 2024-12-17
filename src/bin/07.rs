advent_of_code::solution!(7);

use std::ops::MulAssign;

use num::{Bounded, Integer, NumCast};
use strum_macros::Display;

struct Equation {
    test_value: u64,
    numbers: Vec<u32>,
}

fn possibly_satisfiable(equation: &Equation) -> bool {
    if equation.numbers.is_empty() {
        return false;
    }

    let num_operators = equation.numbers.len() - 1;
    for operator_choices in 0u32..=2u32.pow(num_operators as u32) - 1 {
        let result = equation.numbers.iter().skip(1).enumerate().fold(
            equation.numbers[0] as u64,
            |acc, (idx, number)| {
                if operator_choices & (1 << idx) == 0 {
                    acc * (*number as u64)
                } else {
                    acc + *number as u64
                }
            },
        );
        if result == equation.test_value {
            return true;
        }
    }
    false
}

fn concatenate<N: Integer + NumCast + MulAssign + Bounded + Copy>(lhs: N, rhs: N) -> N {
    let base: N = NumCast::from(10usize).unwrap();
    let ceil: N = N::max_value() / base;
    let mut power = base;

    while rhs >= power {
        if power > ceil {
            return rhs;
        }
        power *= base;
    }
    (lhs * power) + rhs
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display)]
enum Operation {
    Add,
    Multiply,
    Concatenate,
}

fn all_operator_combinations(count: usize) -> Vec<Vec<Operation>> {
    let mut out = Vec::<Vec<Operation>>::new();
    let mut ops = vec![Operation::Add; count];

    for _ in 0..=3u32.pow(count as u32) - 1 {
        out.push(ops.clone());
        for op in ops.iter_mut() {
            let next = match op {
                Operation::Add => Operation::Multiply,
                Operation::Multiply => Operation::Concatenate,
                Operation::Concatenate => Operation::Add,
            };
            *op = next;

            if next != Operation::Add {
                break;
            }
        }
    }

    out
}

fn possibly_satisfiable_with_concatenation(equation: &Equation) -> bool {
    // TODO This is very slow. Can it be improved?

    if equation.numbers.is_empty() {
        return false;
    }

    let num_operators = equation.numbers.len() - 1;
    for operator_set in all_operator_combinations(num_operators) {
        let result = equation
            .numbers
            .iter()
            .skip(1)
            .zip(operator_set.iter())
            .fold(
                equation.numbers[0] as u64,
                |acc, (number, operator)| match operator {
                    Operation::Add => acc + (*number as u64),
                    Operation::Multiply => acc * (*number as u64),
                    Operation::Concatenate => concatenate(acc, *number as u64),
                },
            );
        if result == equation.test_value {
            return true;
        }
    }
    false
}

fn parse_equations(input: &str) -> Option<Vec<Equation>> {
    const EQUATION_DELIMITER: &str = ": ";
    input
        .lines()
        .map(|line| {
            let (lhs, rhs) = line.split_once(EQUATION_DELIMITER)?;
            let test_value = lhs.parse::<u64>().ok()?;
            let numbers: Vec<u32> = rhs
                .split_whitespace()
                .map(|x| x.parse::<u32>().ok())
                .collect::<Option<Vec<u32>>>()?;
            Some(Equation {
                test_value,
                numbers,
            })
        })
        .collect::<Option<Vec<Equation>>>()
}

pub fn part_one(input: &str) -> Option<u64> {
    let equations = parse_equations(input)?;

    let total: u64 = equations
        .iter()
        .filter(|eq| possibly_satisfiable(eq))
        .map(|eq| eq.test_value)
        .sum();

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations = parse_equations(input)?;

    let total: u64 = equations
        .iter()
        .filter(|eq| possibly_satisfiable_with_concatenation(eq))
        .map(|eq| eq.test_value)
        .sum();

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }

    #[test]
    fn test_concatenate() {
        let test_data = vec![
            ((1234, 56789), 123456789),
            ((1984, 38), 198438),
            ((79, 2549385), 792549385),
            ((4, 38908639), 438908639),
            ((317, 739200), 317739200),
            ((85381, 235), 85381235),
            ((35629031, 7), 356290317),
            ((5, 3), 53),
            ((7342, 32941), 734232941),
        ];
        for ((lhs, rhs), expected) in test_data {
            assert_eq!(concatenate(lhs, rhs), expected);
        }
    }
}
