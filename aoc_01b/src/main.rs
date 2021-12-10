use std::str::FromStr;

fn aoc_01b(input: &str) -> u32 {
    let report: Vec<u16> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| u16::from_str(line).unwrap()).collect();

    let mut averaged_report: Vec<u16> = Vec::with_capacity(report.len() - 2);
    for i in 0..report.len() - 2 {
        let average = report[i] + report[i + 1] + report[i + 2];
        averaged_report.push(average);
    }

    let mut increments_counter: u32 = 0;
    for i in 1..averaged_report.len() {
        if averaged_report[i] > averaged_report[i-1] {
            increments_counter += 1;
        }
    }

    increments_counter
}

fn main() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let increments_counter = aoc_01b(&input);

    println!("{} increments!!", increments_counter);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aoc_01b() {
        let input = std::fs::read_to_string("input/test.txt").unwrap();
        assert_eq!(aoc_01b(&input), 5)
    }
}