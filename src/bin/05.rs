use std::{collections::HashSet, ops::RangeInclusive};

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

pub fn reduce(ranges: &[RangeInclusive<u64>]) -> Option<Vec<RangeInclusive<u64>>> {
    println!("reducing!");
    dbg!(ranges);
    let mut reduced = HashSet::new();
    let mut some_reduction = false;
    'candidate: for candidate in ranges.iter() {
        for range in ranges.iter() {
            println!("");
            dbg!(candidate);
            dbg!(range);

            let cs = *candidate.start();
            let ce = *candidate.end();
            let rs = *range.start();
            let re = *range.end();

            // if the candidate starts outside of the range, but end inside of the range
            let start_b4 = cs < rs && ce <= re && ce > rs;
            let end_after = cs >= rs && cs < re && ce > re;
            let encapsulated = cs < rs && ce > re || rs < cs && re > ce;

            if start_b4 || end_after || encapsulated {
                dbg!(start_b4);
                dbg!(end_after);
                dbg!(encapsulated);
                println!("mergd");
                reduced.insert(RangeInclusive::new(rs.min(cs), re.max(ce)));
                some_reduction = true;
                continue 'candidate;
            }
        }

        reduced.insert(candidate.clone());
    }

    if !some_reduction {
        return None;
    }

    Some(reduced.into_iter().collect::<Vec<_>>())
}

pub fn part_two(input: &str) -> Option<u64> {
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

    let mut curr = fresh_ranges;

    while let Some(reduced) = reduce(&curr) {
        // dbg!(&curr);
        curr = reduced;
    }

    let mut total = 0;

    for range in curr {
        total += range.end() - range.start() + 1
    }

    Some(total)
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
