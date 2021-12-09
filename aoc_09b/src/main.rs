type Grid2d = Vec<Vec<u8>>;

fn prepare_aoc_09b_input(filename: &str) -> Result<Grid2d, std::io::Error> {
    let content = std::fs::read_to_string(filename)?;
    Ok(content.lines().map(|line| line.as_bytes().to_vec()).collect())
}

fn find_adjacents(grid: &Grid2d, pos: (usize, usize)) -> [u8; 4] {
    let up = if pos.1 == 0 { u8::MAX } else { grid[pos.1 - 1][pos.0] };
    let down = if pos.1 == grid.len() - 1 { u8::MAX } else { grid[pos.1 + 1][pos.0] };
    let left = if pos.0 == 0 { u8::MAX } else { grid[pos.1][pos.0 - 1] };
    let right = if pos.0 == grid[0].len() - 1 { u8::MAX } else { grid[pos.1][pos.0 + 1] };

    [up, down, left, right]
}

fn find_low_points(grid: &Grid2d) -> Vec<(usize, usize)>  {
    let mut low_points: Vec<(usize, usize)> = Vec::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, value) in row.iter().enumerate() {
            let adjacents = find_adjacents(grid, (j, i));
            if adjacents.iter().all(|adjacent| adjacent > value) {
                low_points.push((j, i));
            }
        }
    }

    low_points
}

fn filter_adjacents(adjacents: [u8; 4], center: (usize, usize), excluded: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let mut filtered_points: Vec<(usize, usize)> = Vec::new();

    if (adjacents[0] - 48) < 9 {
        let point = (center.0, center.1 - 1);
        if !excluded.contains(&point) {
            filtered_points.push(point);
        }
    }

    if (adjacents[1] - 48) < 9 {
        let point = (center.0, center.1 + 1);
        if !excluded.contains(&point) {
            filtered_points.push(point);
        }
    }

    if (adjacents[2] - 48) < 9 {
        let point = (center.0 - 1, center.1);
        if !excluded.contains(&point) {
            filtered_points.push(point);
        }
    }

    if (adjacents[3] - 48) < 9 {
        let point = (center.0 + 1, center.1);
        if !excluded.contains(&point) {
            filtered_points.push(point);
        }
    }

    filtered_points
}

fn solve_aoc_09b(input: &Grid2d) -> usize {
    let low_points = find_low_points(input);
    //println!("{:?}", low_points);

    let mut basin_sizes: Vec<usize> = Vec::new();

    for low_point in low_points {
        let mut next_points: Vec<(usize, usize)> = Vec::new();
        let mut seen_points: Vec<(usize, usize)> = Vec::new();
        let mut basin_size: usize = 0;

        next_points.push(low_point);
        seen_points.push(low_point);

        loop {
            let next_point = next_points.pop().unwrap();
            //println!("n{:?}", next_point);

            let adjacents = find_adjacents(input, next_point);
            let points_found = filter_adjacents(adjacents, next_point, &seen_points);
            //println!("a{:?}", adjacents);

            next_points.extend(&points_found);
            seen_points.extend(&points_found);
            basin_size += 1;

            if next_points.len() == 0 {
                break;
            }
        }

        basin_sizes.push(basin_size);
        //println!("a{:?}",  basin_points);
    }

    //println!("a{:?}", basin_sizes);
    
    basin_sizes.sort_unstable_by(|a, b| b.partial_cmp(a).unwrap());
    basin_sizes[0] * basin_sizes[1] * basin_sizes[2]
}

fn main() -> Result<(), std::io::Error> {
    let input = prepare_aoc_09b_input("input/input.txt")?;
    let result = solve_aoc_09b(&input);

    println!("The result is {}!!", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aoc_09b() {
        let input = prepare_aoc_09b_input("input/test.txt").unwrap();
        let result = solve_aoc_09b(&input);

        assert_eq!(result, 1134)
    }
}