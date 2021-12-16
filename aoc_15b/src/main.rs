use std::{ops::Index, collections::HashMap, cmp::Ordering, fmt::Debug};

type Point = (usize, usize);

struct Grid2d {
    grid: Vec<u8>,
    size: (usize, usize),
    real_size: (usize, usize),
}

impl Grid2d {
    fn index(&self, index: Point) -> u8 {
        //println!("a{:?}", index);
        let mut cost = self[(index.0 % self.real_size.0, index.1 % self.real_size.1)] as usize;
        cost += index.1 / self.real_size.1 + index.0 / self.real_size.0;
        ( if cost > 9 { cost % 9 } else { cost } ) as u8
    }
}

impl Index<Point> for Grid2d {
    type Output = u8;

    fn index(&self, index: Point) -> &Self::Output {
        if index.0 >= self.real_size.0 || index.1 >= self.real_size.0 {
            panic!("Out of bound point. Grid size = {:?}. Point = {:?}", self.size, index);
        }

        &self.grid[index.1 * self.real_size.0 + index.0]
    }
}

impl Debug for Grid2d {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for j in 0..self.size.0 {
            for i in 0..self.size.1 {
                write!(f, "{} ", self.index((i, j)))?
            }
            write!(f, "\n")?

        }
        Ok(())
    }
}

fn prepare_aoc_15b_input(filename: &str) -> Result<Grid2d, std::io::Error> {
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
        real_size: (xsize, ysize),
        size: (xsize * 5, ysize * 5),
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

fn solve_aoc_15b(grid: &Grid2d) -> usize {
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
            let cost = costs[&current] + grid.index(adjacent) as usize;

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
    let input = prepare_aoc_15b_input("input/input.txt")?;
    let result = solve_aoc_15b(&input);

    println!("The result is {}!!", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aoc_15b() {
        let input = prepare_aoc_15b_input("input/test.txt").unwrap();
        let result = solve_aoc_15b(&input);

        assert_eq!(result, 315)
    }
}