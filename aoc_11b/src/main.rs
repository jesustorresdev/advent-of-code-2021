type Grid2d = Vec<Vec<u8>>;

fn prepare_aoc_11b_input(filename: &str) -> Result<Grid2d, std::io::Error> {
    let content = std::fs::read_to_string(filename)?;
    Ok(content.lines().map(|line| line.as_bytes().iter().map(|c| *c - 48).collect()).collect())
}

fn get_adjacents(grid: &Grid2d, pos: (usize, usize)) -> Vec<(u8, u8)> {
    let mut adjacents: Vec<(u8, u8)> = Vec::new();

    for j in -1isize..=1 {
        for i in -1isize..=1 {
            let x = pos.0 as isize + i;
            let y = pos.1 as isize + j;

            if x >= 0 && y >= 0 && x < grid[0].len() as isize && y < grid.len() as isize {
                adjacents.push((x as u8, y as u8));
            }
        }
    }

    adjacents
}

fn solve_aoc_11b(input: &Grid2d) -> usize {
    //println!("{:?}", input);
    let mut grid = input.clone();

    for t in 0..1000 {
        let mut flashed_grid: Vec<Vec<bool>> = grid.iter().map(|row| vec![false; row.len()]).collect();

        for row in grid.iter_mut() {
            for energy in row.iter_mut() {
                *energy += 1;
            }
        }

        loop {
            let mut repeat: bool = false;

            for j in 0..grid.len() {
                for i in 0..grid[j].len() {
                    if grid[j][i] >= 10 && !flashed_grid[j][i] {
                        flashed_grid[j][i] = true;
                        repeat = true;

                        let adjacents = get_adjacents(&grid, (i, j));
                        // println!("{:?}", adjacents);
                        for (x, y) in adjacents {
                            grid[y as usize][x as usize] += 1;
                        }
                    }
                }
            }

            if !repeat {
                break;
            }
        }

        for row in grid.iter_mut() {
            for energy in row.iter_mut() {
                if *energy > 9 {
                    *energy = 0;
                }
            }
        }

        if flashed_grid.iter().flatten().all(|flashed| *flashed == true) {
            return t + 1;
        }
    }

    usize::MAX
}

fn main() -> Result<(), std::io::Error> {
    let input = prepare_aoc_11b_input("input/input.txt")?;
    let result = solve_aoc_11b(&input);

    println!("The result is {}!!", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aoc_11b() {
        let input = prepare_aoc_11b_input("input/test.txt").unwrap();
        let result = solve_aoc_11b(&input);

        assert_eq!(result, 195)
    }
}