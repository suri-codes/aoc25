advent_of_code::solution!(4);

#[derive(PartialEq, Debug)]
enum Tile {
    Roll,
    Free,
}

pub fn part_one(input: &str) -> Option<u64> {
    // I need to turn this grid into a what exactly
    let grid = input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| match c {
                    '.' => Tile::Free,
                    '@' => Tile::Roll,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut accessible_rolls = 0;
    // we need to walk the tile
    for (x, row) in grid.iter().enumerate() {
        for (y, tile) in row.iter().enumerate() {
            if *tile == Tile::Roll {
                // we need to check the 8 adjacent tiles
                let mut adjacent_rolls = 0;

                let mut roll_check = |row: &Vec<Tile>, y_offset: isize| {
                    let idx = y as isize + y_offset;

                    if idx == -1 {
                        return;
                    }

                    let idx = idx as usize;

                    adjacent_rolls += row
                        .get(idx)
                        .map(|t| match t {
                            Tile::Roll => 1,
                            Tile::Free => 0,
                        })
                        .unwrap_or(0);
                };

                // lets check to our left and right first
                roll_check(row, -1);
                roll_check(row, 1);

                // lets get the row above; ours if it exists
                if x > 0
                    && let Some(row_above) = grid.get(x - 1)
                {
                    roll_check(row_above, -1);
                    roll_check(row_above, 0);
                    roll_check(row_above, 1);
                }

                if let Some(row_below) = grid.get(x + 1) {
                    roll_check(row_below, -1);
                    roll_check(row_below, 0);
                    roll_check(row_below, 1);
                }

                if adjacent_rolls < 4 {
                    accessible_rolls += 1;
                }
            }
        }
    }

    Some(accessible_rolls)
}

pub fn part_two(input: &str) -> Option<u64> {
    // I need to turn this grid into a what exactly
    let mut grid = input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|c| match c {
                    '.' => Tile::Free,
                    '@' => Tile::Roll,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut total_accessible_rolls = 0;
    loop {
        let mut epoch_rolls = 0;
        let mut to_remove = vec![];

        // first we process the grid, marking everything thats removable

        for (x, row) in grid.iter().enumerate() {
            for (y, tile) in row.iter().enumerate() {
                if *tile == Tile::Roll {
                    // println!("checking tile {x}, {y}");
                    // we need to check the 8 adjacent tiles
                    let mut adjacent_rolls = 0;

                    let mut roll_check = |row: &Vec<Tile>, y_offset: isize| {
                        let idx = y as isize + y_offset;

                        if idx == -1 {
                            return;
                        }

                        let idx = idx as usize;

                        adjacent_rolls += row
                            .get(idx)
                            .map(|t| match t {
                                Tile::Roll => 1,
                                Tile::Free => 0,
                            })
                            .unwrap_or(0);
                    };

                    // lets check to our left and right first
                    roll_check(row, -1);
                    roll_check(row, 1);

                    // lets get the row above; ours if it exists

                    if x > 0
                        && let Some(row_above) = grid.get(x - 1)
                    {
                        roll_check(row_above, -1);
                        roll_check(row_above, 0);
                        roll_check(row_above, 1);
                    }

                    if let Some(row_below) = grid.get(x + 1) {
                        roll_check(row_below, -1);
                        roll_check(row_below, 0);
                        roll_check(row_below, 1);
                    }

                    if adjacent_rolls < 4 {
                        epoch_rolls += 1;
                        to_remove.push((x, y))
                    }
                }
            }
        }
        // then we remove
        for (x, y) in to_remove {
            let tile = grid.get_mut(x).unwrap().get_mut(y).unwrap();
            *tile = Tile::Free;
        }

        if epoch_rolls == 0 {
            break;
        } else {
            total_accessible_rolls += epoch_rolls;
        }
        //
    }

    Some(total_accessible_rolls)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
