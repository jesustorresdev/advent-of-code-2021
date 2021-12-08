use std::collections::HashMap;

type Signals = Vec<String>;
type Segments = Vec<String>; 

fn prepare_aoc_08b_input(filename: &str) -> Result<Vec<(Signals, Segments)>, std::io::Error> {
    let content = std::fs::read_to_string(filename)?;
    let output: Vec<(Signals, Segments)> = content.lines().map(|line| {
        let parts = line.split_once("|").unwrap();
        (
            parts.0.split_whitespace().map(|word| {
                let mut v: Vec<char> = word.chars().collect();
                v.sort_unstable();
                v.into_iter().collect()
            }).collect::<Vec<_>>(),
            parts.1.split_whitespace().map(|word| {
                let mut v: Vec<char> = word.chars().collect();
                v.sort_unstable();
                v.into_iter().collect()
            }).collect::<Vec<_>>()
        )
    }).collect();
    //println!("{:?}", output);
    Ok(output)
}

fn matching_segments(sequence: &str, segments: &str) -> usize {
    let mut counter = 0;
    for c in sequence.chars() {
        if segments.contains(c) {
            counter += 1;
        }
    }
    counter
}

fn solve_aoc_08b(input: &[(Signals, Segments)]) -> u32 {
    let mut displayed_number: u32 = 0;

    for (signals, segments) in input {
        // println!("sig: {:?}", signals);
        // println!("seg: {:?}", segments);
        let mut dict: HashMap<&str, u32> = HashMap::new();

        let digit1_signals = signals.iter().find(|word| word.len() == 2).unwrap();
        dict.insert(digit1_signals, 1);

        let digit7_signals = signals.iter().find(|word| word.len() == 3).unwrap();
        dict.insert(digit7_signals, 7);

        let digit4_signals = signals.iter().find(|word| word.len() == 4).unwrap();
        dict.insert(digit4_signals, 4);

        let digit8_signals = signals.iter().find(|word| word.len() == 7).unwrap();
        dict.insert(digit8_signals, 8);

        let digit3_signals = signals.iter().find(|word| word.len() == 5 &&
             matching_segments(word, digit7_signals) == 3).unwrap();
        dict.insert(digit3_signals, 3);

        let digit9_signals = signals.iter().find(|word| word.len() == 6 &&
        matching_segments(word, digit3_signals) == 5).unwrap();
        dict.insert(digit9_signals, 9);

        let digit0_signals = signals.iter().find(|word| word.len() == 6 && *word != digit9_signals &&
            matching_segments(word, digit7_signals) == 3).unwrap();
        dict.insert(digit0_signals, 0);

        let digit6_signals = signals.iter().find(|word| word.len() == 6 && *word != digit0_signals &&
            *word != digit9_signals).unwrap();
        dict.insert(digit6_signals, 6);

        let digit5_signals = signals.iter().find(|word| word.len() == 5 &&
            matching_segments(word, digit6_signals) == 5).unwrap();
        dict.insert(digit5_signals, 5);

        let digit2_signals = signals.iter().find(|word| word.len() == 5 &&
            *word != digit5_signals && *word != digit3_signals).unwrap();
        dict.insert(digit2_signals, 2);

        //println!("dict: {:?}", dict);
        
        for (i, magnitude) in [1000, 100, 10, 1].iter().enumerate() {
            //println!("word: {:?}", segments[i]);
            displayed_number += dict.get(segments[i].as_str()).unwrap() * magnitude;
        }
    }
    displayed_number
}

fn main() -> Result<(), std::io::Error> {
    let input = prepare_aoc_08b_input("input/input.txt")?;
    let result = solve_aoc_08b(&input);

    println!("The result is {}!!", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aoc_08b() {
        let input = prepare_aoc_08b_input("input/test.txt").unwrap();
        let result = solve_aoc_08b(&input);

        assert_eq!(result, 61229)
    }
}