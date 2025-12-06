#![feature(range_into_bounds)]
use std::{
    collections::{BTreeSet, HashSet},
    ops::{IntoBounds, RangeInclusive},
};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let split = input.trim().split("\n\n").collect::<Vec<_>>();

    let fresh_ranges = split[0]
        .trim()
        .lines()
        .map(|r_str| {
            let r_vec = r_str
                .split('-')
                .map(|s| s.parse::<u64>().expect("must be a valid number"))
                .collect::<Vec<_>>();

            RangeInclusive::new(r_vec[0], r_vec[1])
        })
        .collect::<Vec<_>>();

    let ids = split[1]
        .trim()
        .lines()
        .map(|e| e.parse::<u64>().expect("should be a valid number"))
        .collect::<Vec<_>>();

    let mut fresh = 0;
    'id: for id in ids {
        for range in &fresh_ranges {
            if range.contains(&id) {
                fresh += 1;
                continue 'id;
            }
        }
    }
    Some(fresh)
}

pub fn part_two(input: &str) -> Option<u64> {
    let split = input.trim().split("\n\n").collect::<Vec<_>>();

    let mut fresh_ranges = split[0]
        .trim()
        .lines()
        .map(|r_str| {
            let r_vec = r_str
                .split('-')
                .map(|s| s.parse::<u64>().expect("must be a valid number"))
                .collect::<Vec<_>>();

            (r_vec[0], r_vec[1])
        })
        .collect::<Vec<_>>();

    fresh_ranges.sort();
    fresh_ranges.reverse();

    let mut discrete_ranges = vec![];

    loop {
        let Some((curr_start, mut curr_end)) = fresh_ranges.pop() else {
            break;
        };

        loop {
            let Some((next_start, next_end)) = fresh_ranges.last() else {
                break;
            };

            if *next_start <= curr_end {
                curr_end = curr_end.max(*next_end);
                fresh_ranges.pop();
            } else {
                break;
            }
        }

        discrete_ranges.push((curr_start, curr_end));
    }

    Some(discrete_ranges.iter().map(|(s, e)| e - s + 1).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}
