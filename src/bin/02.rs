#![feature(new_range_api)]

use std::range::RangeInclusive;

advent_of_code::solution!(2);

// need a function that takes a range of numbers and then returns a vector of numbers
// that returns all numbers in that range who have repeating id's
//
pub fn invalid_in_range(range: RangeInclusive<u64>) -> Vec<u64> {
    let mut invalids = vec![];

    'num: for num in range {
        let digits: Vec<u64> = num
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u64)
            .collect();

        let mut cmp = vec![];

        'digit: for digit in digits.iter() {
            cmp.push(*digit);

            if !digits.len().is_multiple_of(cmp.len()) {
                continue 'digit;
            }

            if digits.len() < cmp.len() * 2 {
                continue 'num;
            }
            let mut matches = 0;
            'cmp: for i in 1..=digits.len() / cmp.len() {
                let Some(subslice) = digits.get(cmp.len() * i..cmp.len() * (i + 1)) else {
                    break 'cmp;
                };

                if cmp == subslice {
                    matches += 1
                } else {
                    continue 'digit;
                }
            }

            if matches == 1 {
                println!("Invalid Number: {num}");
                invalids.push(num);
                continue 'num;
            }
        }
    }

    invalids
}

pub fn part_one(input: &str) -> Option<u64> {
    let ranges = input
        .split(',')
        .map(|range| {
            let mut iter = range.split('-');

            let start: u64 = iter
                .next()
                .unwrap()
                .parse()
                .inspect_err(|_| {
                    dbg!(range);
                })
                .expect("should be a valid number");

            let end: u64 = iter
                .next()
                .unwrap()
                .parse()
                .inspect_err(|_| {
                    dbg!(range);
                })
                .expect("should be a valid number");

            start..=end
        })
        .collect::<Vec<_>>();

    let mut all_invalids = vec![];

    for range in ranges {
        all_invalids.extend_from_slice(invalid_in_range(range.into()).as_slice());
    }

    Some(all_invalids.into_iter().sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
