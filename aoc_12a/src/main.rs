use multimap::MultiMap;
use std::collections::VecDeque;

type Edges = MultiMap<String,String>;

fn prepare_aoc_12a_input(filename: &str) -> Result<Edges, std::io::Error> {
    let content = std::fs::read_to_string(filename)?;
    let mut edges: Edges = Edges::new();
    for line in content.lines() {
        let parts = line.split_once("-");
        if parts.is_some() {
            edges.insert(parts.unwrap().0.to_string(), parts.unwrap().1.to_string());
            edges.insert(parts.unwrap().1.to_string(), parts.unwrap().0.to_string());
        }
    }
    Ok(edges)
}

fn solve_aoc_12a(input: &Edges) -> usize {
    //println!("{:?}", input);
    let start_cave = "start".to_string();
    let mut open_paths: VecDeque<Vec<&String>> = VecDeque::from([vec![&start_cave]]);
    let mut full_paths: Vec<Vec<&String>> = Vec::new();

    while open_paths.len() != 0 {
        let path = open_paths.pop_front().unwrap();
        let cave = path[path.len() - 1];
        for next_cave in input.get_vec(cave).unwrap() {
            if next_cave == "end" {
                let mut full_path = path.clone();
                full_path.push(next_cave);
                full_paths.push(full_path);
            }
            else if next_cave.chars().all(|c| c.is_uppercase()) || !path.contains(&next_cave) {
                let mut new_path = path.clone();
                new_path.push(next_cave);
                open_paths.push_back(new_path);
            }
        }
    }

    full_paths.len()
}

fn main() -> Result<(), std::io::Error> {
    let input = prepare_aoc_12a_input("input/input.txt")?;
    let result = solve_aoc_12a(&input);

    println!("The result is {}!!", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aoc_12a() {
        let input = prepare_aoc_12a_input("input/test03.txt").unwrap();
        let result = solve_aoc_12a(&input);

        //assert_eq!(result, 10)
        //assert_eq!(result, 19)
        assert_eq!(result, 226)
    }
}