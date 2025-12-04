advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let mut total_jotage = 0;
    for bank in input.lines() {
        let bank = bank
            .trim()
            .chars()
            .map(|c| c.to_digit(10).expect("should be a valid number") as u64)
            .collect::<Vec<_>>();

        let mut max = 0;
        let mut max_idx = 0;

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
        total_jotage += max_jotage;
    }
    Some(total_jotage)
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
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
