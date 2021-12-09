type Grid2d = Vec<Vec<u8>>;

fn prepare_aoc_09a_input(filename: &str) -> Result<Grid2d, std::io::Error> {
    let content = std::fs::read_to_string(filename)?;
    Ok(content.lines().map(|line| line.as_bytes().to_vec()).collect())
}

fn get_adjacents(grid: &Grid2d, pos: (usize, usize)) -> [u8; 4] {
    let up = if pos.1 == 0 { u8::MAX } else { grid[pos.1 - 1][pos.0] };
    let down = if pos.1 == grid.len() - 1 { u8::MAX } else { grid[pos.1 + 1][pos.0] };
    let left = if pos.0 == 0 { u8::MAX } else { grid[pos.1][pos.0 - 1] };
    let right = if pos.0 == grid[0].len() - 1 { u8::MAX } else { grid[pos.1][pos.0 + 1] };

    [up, down, left, right]
}

fn solve_aoc_09a(input: &Grid2d) -> usize {
    let mut risk: usize = 0;

    for (i, row) in input.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            let adjacents = get_adjacents(input, (j, i));
            //println!("{:?}", adjacents);
            if adjacents.iter().all(|adjacent| adjacent > value) {
                risk += (*value as usize) - 48 + 1;
            }
        }
    }

    risk
}

fn main() -> Result<(), std::io::Error> {
    let input = prepare_aoc_09a_input("input/input.txt")?;
    let result = solve_aoc_09a(&input);

    println!("The result is {}!!", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aoc_09a() {
        let input = prepare_aoc_09a_input("input/test.txt").unwrap();
        let result = solve_aoc_09a(&input);

        assert_eq!(result, 15)
    }
}