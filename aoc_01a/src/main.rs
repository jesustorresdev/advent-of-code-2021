use std::str::FromStr;

fn aoc_01a(input: &str) -> u32 {
    let mut increments_counter: u32 = 0;
    let report: Vec<u16> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| u16::from_str(line).unwrap()).collect();

    for i in 1..report.len() {
        if report[i] > report[i-1] {
            increments_counter += 1;
        }
    }

    increments_counter
}

fn main() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let increments_counter = aoc_01a(&input);

    println!("{} increments!!", increments_counter);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aoc_01b() {
        let input = std::fs::read_to_string("input/test.txt").unwrap();
        assert_eq!(aoc_01a(&input), 7)
    }
}