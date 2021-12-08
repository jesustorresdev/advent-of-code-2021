fn prepare_aoc_08a_input(filename: &str) -> Result<Vec<String>, std::io::Error> {
    let content = std::fs::read_to_string(filename)?;
    let segments: Vec<String> = content.lines().flat_map(|line| {
        line.split("|").last().unwrap().split_whitespace().map(|word| word.to_owned()).collect::<Vec<_>>()
    }).collect();
    //println!("{:?}", segments);
    Ok(segments)
}

fn solve_aoc_08a(input: &[String]) -> usize {
    input.iter().filter(|segments| [2, 3, 4, 7].contains(&segments.len())).count()
}

fn main() -> Result<(), std::io::Error> {
    let input = prepare_aoc_08a_input("input/input.txt")?;
    let result = solve_aoc_08a(&input);

    println!("The result is {}!!", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aoc_08a() {
        let input = prepare_aoc_08a_input("input/test.txt").unwrap();
        let result = solve_aoc_08a(&input);

        assert_eq!(result, 26)
    }
}