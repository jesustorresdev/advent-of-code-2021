use std::collections::HashMap;

fn prepare_aoc_10a_input(filename: &str) -> Result<Vec<String>, std::io::Error> {
    let content = std::fs::read_to_string(filename)?;
    Ok(content.lines().map(|line| line.to_owned()).collect())
}

fn get_delimiter_pairs() -> HashMap<char, char> {
    HashMap::from([
        ('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')
    ])
}

fn get_illegal_char_scores() -> HashMap<char, usize> {
    HashMap::from([
        (')', 3), (']', 57), ('}', 1197), ('>', 25137)
    ])
}

fn solve_aoc_10a(input: &[String]) -> usize {
    let delimiters = get_delimiter_pairs();
    let delimiters_scores = get_illegal_char_scores();
    let mut score: usize = 0;

    for sequence in input {
        let mut chunks_found: Vec<char> = Vec::new();

        for c in sequence.chars() {
            if delimiters.contains_key(&c) {
                chunks_found.push(c);
            }
            else if chunks_found.last().is_some() && delimiters[chunks_found.last().unwrap()] == c {
                chunks_found.pop();
            }
            else {
                score += delimiters_scores.get(&c).unwrap();
                break;
            }
        }
    }

    score
}

fn main() -> Result<(), std::io::Error> {
    let input = prepare_aoc_10a_input("input/input.txt")?;
    let result = solve_aoc_10a(&input);

    println!("The result is {}!!", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aoc_10a() {
        let input = prepare_aoc_10a_input("input/test.txt").unwrap();
        let result = solve_aoc_10a(&input);

        assert_eq!(result, 26397)
    }
}