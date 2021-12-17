fn prepare_aoc_16a_input(filename: &str) -> Result<Grid2d, std::io::Error> {
    let content = std::fs::read_to_string(filename)?;
    let mut grid: Vec<u8> = Vec::new();
    let mut xsize: usize = 0;
    let mut ysize: usize = 0;
    for line in content.lines() {
        xsize = line.len();
        ysize += 1;
        grid.extend( line.as_bytes().iter().map(|c| *c - 48));
    }

    Ok(Grid2d{
        grid: grid,
        size: (xsize, ysize),
    })
}

fn get_adjacents(grid: &&Grid2d, point: Point) -> Vec<Point> {
    let mut adjacents: Vec<Point> = Vec::new();

    if point.1 > 0 { adjacents.push((point.0, point.1 - 1)); }
    if point.1 < (grid.size.1 - 1) { adjacents.push((point.0, point.1 + 1)); }
    if point.0 > 0 { adjacents.push((point.0 - 1, point.1)); }
    if point.0 < (grid.size.0 - 1) { adjacents.push((point.0 + 1, point.1)); }

    adjacents
}

fn solve_aoc_16a(grid: &Grid2d) -> usize {
    //println!("{:?}", grid);

    let start: Point = (0, 0);
    let end: Point = (grid.size.0 - 1, grid.size.1 - 1);

    let mut open_list: Vec<Point> = Vec::new();
    let mut came_from: HashMap<Point, Point> = HashMap::new();
    let mut costs: HashMap<Point, usize> = HashMap::new();

    open_list.push(start);
    costs.insert(start, 0);

    while open_list.len() != 0 {
        // Sort in reverse order by cost
        open_list.sort_unstable_by(|a , b| {
            if !costs.contains_key(&a) {
                if !costs.contains_key(&b) { Ordering::Equal } else { Ordering::Less }
            }
            else if !costs.contains_key(&b) {
                Ordering::Greater
            }
            else {
                costs[&b].cmp(&costs[&a])
            }
        });        
        
        let current = open_list.pop().unwrap();
        if current == end {
            break;
        }

        let adjacents = get_adjacents(&grid, current);
        for adjacent in adjacents {
            let cost = costs[&current] + grid[adjacent] as usize;

            if !costs.contains_key(&adjacent) || cost < costs[&adjacent] {
                costs.insert(adjacent, cost);
                came_from.insert(adjacent, current);

                if !open_list.contains(&adjacent) {
                    open_list.push(adjacent);
                }
            }
        }
    }

    costs[&end]
}

fn main() -> Result<(), std::io::Error> {
    let input = prepare_aoc_16a_input("input/input.txt")?;
    let result = solve_aoc_16a(&input);

    println!("The result is {}!!", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aoc_16a() {
        let input = prepare_aoc_16a_input("input/test.txt").unwrap();
        let result = solve_aoc_16a(&input);

        assert_eq!(result, 40)
    }
}