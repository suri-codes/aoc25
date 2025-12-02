advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut dial: i32 = 50;

    let mut zeros = 0;
    for line in input.trim().lines() {
        let dir: char = line.chars().next().expect("first character must exist");

        let step: u64 = line[1..].parse().expect("steps must be valid u16");

        for _ in 0..step {
            match dir {
                'L' => {
                    if dial == 0 {
                        dial = 99;
                    } else {
                        dial -= 1;
                    }
                }

                'R' => {
                    if dial == 99 {
                        dial = 0;
                    } else {
                        dial += 1;
                    }
                }
                _ => {
                    panic!("Invalid Direction")
                }
            }
        }
        // done with rotation, we check for 0
        if dial == 0 {
            zeros += 1;
        }
    }

    Some(zeros)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut dial: i32 = 50;

    let mut zeros = 0;
    for line in input.trim().lines() {
        let dir: char = line.chars().next().expect("first character must exist");

        let step: u64 = line[1..].parse().expect("steps must be valid u16");

        for _ in 0..step {
            match dir {
                'L' => {
                    if dial == 0 {
                        dial = 99;
                    } else {
                        dial -= 1;
                    }
                }

                'R' => {
                    if dial == 99 {
                        dial = 0;
                    } else {
                        dial += 1;
                    }
                }
                _ => {
                    panic!("Invalid Direction")
                }
            }
            // done with rotation, we check for 0
            if dial == 0 {
                zeros += 1;
            }
        }
    }

    Some(zeros)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));

        // assert_eq!(result, Some(1));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
