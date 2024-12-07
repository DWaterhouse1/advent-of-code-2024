advent_of_code::solution!(2);

fn parse_reports(input: &str) -> Option<Vec<Vec<u32>>> {
    const LEVEL_DELIMITER: &str = " ";
    input
        .lines()
        .map(|report_str| {
            report_str
                .split(LEVEL_DELIMITER)
                .map(|level_str| level_str.parse::<u32>().ok())
                .collect::<Option<Vec<u32>>>()
        })
        .collect::<Option<Vec<Vec<u32>>>>()
}

#[derive(PartialEq, Eq)]
enum ReportSafety {
    Safe,
    Unsafe,
}

fn evaluate_report(report: &Vec<u32>) -> ReportSafety {
    #[derive(PartialEq, Eq)]
    enum Monotonicity {
        Increasing,
        Decreasing,
        Unknown,
    }

    let mut monotonicity = Monotonicity::Unknown;

    for pair in report.windows(2) {
        match pair[0] as i64 - pair[1] as i64 {
            // case in which the report is increasing
            -3..0 => {
                if monotonicity == Monotonicity::Decreasing {
                    return ReportSafety::Unsafe;
                }
                monotonicity = Monotonicity::Increasing;
            }

            // case in which the report is stationary
            0 => {
                return ReportSafety::Unsafe;
            }

            // case in which the report is decreasing
            1..=3 => {
                if monotonicity == Monotonicity::Increasing {
                    return ReportSafety::Unsafe;
                }
                monotonicity = Monotonicity::Decreasing;
            }

            // case in which the report is increasing or decreasing too quickly
            _ => {
                return ReportSafety::Unsafe;
            }
        }
    }

    ReportSafety::Safe
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        parse_reports(input)?
            .into_iter()
            .filter(|report| evaluate_report(report) == ReportSafety::Safe)
            .count() as u32,
    )
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
