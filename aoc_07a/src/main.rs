
fn prepare_aoc_07a_input(filename: &str) -> Result<Vec<u32>, std::io::Error> {
    let content = std::fs::read_to_string(filename)?;
    Ok(content.split(",").map(|value| value.parse::<u32>().unwrap()).collect())
}

fn solve_aoc_07a(input: &[u32]) -> u32 {
    //println!("{:?}", input);
    let mut positions = input.to_vec();
    positions.sort_unstable();

    let median = positions[positions.len() / 2];
    positions.iter().fold(0, |acc, pos| acc + if *pos > median { pos - median } else { median - pos }) as u32
}

fn main() -> Result<(), std::io::Error> {
    let input = prepare_aoc_07a_input("input/input.txt")?;
    let result = solve_aoc_07a(&input);

    println!("The result is {}!!", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aoc_07a() {
        let input = prepare_aoc_07a_input("input/test.txt").unwrap();
        let result = solve_aoc_07a(&input);

        assert_eq!(result, 37)
    }
}