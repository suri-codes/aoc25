advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let mut total_joltage = 0;
    for bank in input.lines() {
        let bank = bank
            .trim()
            .chars()
            .map(|c| c.to_digit(10).expect("should be a valid number") as u64)
            .collect::<Vec<_>>();

        let mut max = 0;
        let mut max_idx = 0;

        // we basically need to have a recursive function that does this?

        // we get the maximum value, excluding the final item
        for (idx, digit) in bank[0..bank.len() - 1].iter().enumerate() {
            if *digit > max {
                max = *digit;
                max_idx = idx;
            }
        }

        // now we find the largest digit from the subslice
        let max_2 = *bank[max_idx + 1..]
            .iter()
            .max()
            .expect("should be atleast one element left");

        let max_jotage = max * 10 + max_2;
        total_joltage += max_jotage;
    }
    Some(total_joltage)
}

fn max_digit(input: &[(usize, u64)]) -> (usize, u64) {
    let mut max = 0;
    let mut max_idx = 0;

    for (idx, digit) in input {
        if *digit > max {
            max = *digit;
            max_idx = *idx;
        }
    }

    (max_idx, max)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total_joltage = 0;

    for bank in input.lines() {
        let bank = bank
            .trim()
            .chars()
            .map(|c| c.to_digit(10).expect("should be a valid number") as u64)
            .enumerate()
            .collect::<Vec<_>>();

        let mut joltage_digits = vec![];

        let mut prev_max_idx = 0;
        for i in 0..12 {
            let range = {
                if i == 0 {
                    prev_max_idx
                } else {
                    prev_max_idx + 1
                }
            }..(bank.len() - (11 - i));

            let subslice = &bank[range];

            let (max_idx, max) = max_digit(subslice);
            prev_max_idx = max_idx;
            joltage_digits.push(max);
        }

        let joltage = joltage_digits
            .iter()
            .rev()
            .enumerate()
            .map(|(exp, val)| val * 10_u64.pow(exp as u32))
            .sum::<u64>();

        total_joltage += joltage;
    }

    Some(total_joltage)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
