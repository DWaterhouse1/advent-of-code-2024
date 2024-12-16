advent_of_code::solution!(7);

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

pub fn part_one(input: &str) -> Option<u64> {
    const EQUATION_DELIMITER: &str = ": ";
    let equations = input
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
        .collect::<Option<Vec<Equation>>>()?;

    let total: u64 = equations
        .iter()
        .filter(|eq| possibly_satisfiable(eq))
        .map(|eq| eq.test_value)
        .sum();

    Some(total)
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
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
