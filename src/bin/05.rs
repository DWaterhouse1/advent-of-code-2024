use itertools::Itertools;
use std::cmp;
use std::collections::{HashMap, HashSet};

advent_of_code::solution!(5);

type Rule = (u32, u32);
type PageUpdateList = Vec<u32>;

fn parse_input(input: &str) -> Option<(Vec<Rule>, Vec<PageUpdateList>)> {
    const RULES_DELIMITER: char = '|';
    const UPDATES_DELIMITER: char = ',';

    let end_rules = input.lines().position(|line| line.is_empty())?;

    let rules: Vec<Rule> = input
        .lines()
        .take(end_rules)
        .filter_map(|line| line.split_once(RULES_DELIMITER))
        .filter_map(|(left, right)| Some((left.parse::<u32>().ok()?, right.parse::<u32>().ok()?)))
        .collect();

    let page_updates: Vec<PageUpdateList> = input
        .lines()
        .skip(end_rules)
        .map(|line| {
            line.split(UPDATES_DELIMITER)
                .fold(Vec::new(), |mut acc, page| {
                    if let Ok(page_number) = page.parse::<u32>() {
                        acc.push(page_number);
                    }
                    acc
                })
        })
        .filter(|updates| !updates.is_empty())
        .collect();

    Some((rules, page_updates))
}

fn compute_precedent_rules(rules_list: &Vec<Rule>) -> HashMap<u32, Vec<u32>> {
    let mut rules = HashMap::<u32, Vec<u32>>::new();

    for (left, right) in rules_list {
        match rules.get_mut(right) {
            Some(requirements) => {
                requirements.push(*left);
            }
            None => {
                rules.insert(*right, vec![*left; 1]);
            }
        }
    }

    rules
}

fn satisfies_ordering_rules(
    update_list: &PageUpdateList,
    precedent_rules: &HashMap<u32, Vec<u32>>,
) -> bool {
    let mut deny_list = HashSet::new();
    for update in update_list.iter() {
        if deny_list.contains(&update) {
            return false;
        }

        if let Some(denials) = precedent_rules.get(update) {
            for number in denials {
                deny_list.insert(number);
            }
        }

        if let Some(denials) = precedent_rules.get(update) {
            for number in denials {
                deny_list.insert(number);
            }
        }
    }
    true
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules_list, page_updates) = parse_input(input)?;

    let rules = compute_precedent_rules(&rules_list);

    let total = page_updates
        .iter()
        .filter(|list| satisfies_ordering_rules(list, &rules))
        .fold(0, |acc, good_updates| {
            let middle_index = good_updates.len() / 2;
            acc + good_updates[middle_index]
        });

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules_list, page_updates) = parse_input(input)?;

    let rules = compute_precedent_rules(&rules_list);

    let total = page_updates
        .into_iter()
        .filter(|list| !satisfies_ordering_rules(list, &rules))
        .update(|list| {
            list.sort_by(|a, b| {
                if let Some(precedents) = rules.get(b) {
                    if precedents.contains(a) {
                        return cmp::Ordering::Less;
                    }
                }
                cmp::Ordering::Greater
            });
        })
        .fold(0, |acc, good_updates| {
            let middle_index = good_updates.len() / 2;
            acc + good_updates[middle_index]
        });

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
