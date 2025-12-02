advent_of_code::solution!(1);

// The attached document (your puzzle input) contains a sequence
// of rotations, one per line, which tell you how to open the
// safe. A rotation starts with an L or R which indicates whether
// the rotation should be to the left (toward lower numbers) or
// to the right (toward higher numbers). Then, the rotation has
// a distance value which indicates how many clicks the dial
// should be rotated in that direction.

// So, if the dial were pointing at 11, a rotation of R8 would
// cause the dial to point at 19. After that, a rotation of L19
// would cause it to point at 0.

// Because the dial is a circle, turning the dial left from 0 one
// click makes it point at 99. Similarly, turning the dial right
// from 99 one click makes it point at 0.

// So, if the dial were pointing at 5, a rotation of L10 would
// cause it to point at 95. After that, a rotation of R5 could
// cause it to point at 0.

// The dial starts by pointing at 50.

// You could follow the instructions, but your recent required
// official North Pole secret entrance security training seminar
// taught you that the safe is actually a decoy. The actual
// password is the number of times the dial is left pointing at
// 0 after any rotation in the sequence.

// Example input:
// L68
// L30
// R48
// L5
// R60
// L55
// L1
// L99
// R14
// L82
pub fn part_one(input: &str) -> Option<u64> {
    None
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
