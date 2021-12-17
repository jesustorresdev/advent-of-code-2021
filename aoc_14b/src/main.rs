use regex::Regex;
use std::collections::HashMap;

type Pair = (u8, u8);

fn prepare_aoc_14b_input(filename: &str) -> Result<(Vec<u8>, HashMap<Pair, u8>), std::io::Error> {
    let content = std::fs::read_to_string(filename)?;
    let mut lines = content.lines();

    let sequence = lines.next().unwrap().bytes().collect();
    lines.next();

    let line_regex = Regex::new(r"(.)(.) -> (.)").unwrap();
    let mut rules: HashMap<Pair, u8> = HashMap::new();
    for line in lines.filter(|line| !line.is_empty()) {
        let captures = line_regex.captures(line).unwrap();
        let pair_first: u8 = captures.get(1).unwrap().as_str().as_bytes()[0];
        let pair_second: u8 = captures.get(2).unwrap().as_str().as_bytes()[0];
        let element = captures.get(3).unwrap().as_str().as_bytes()[0];
        rules.insert((pair_first, pair_second), element);
    }

    Ok((sequence, rules))
}

fn solve_aoc_14b(sequence: &[u8], rules: &HashMap<Pair, u8>) -> usize {
    println!("{:?}\n{:?}", sequence, rules);

    let mut pairs_histogram: HashMap<Pair, isize> = HashMap::new();
    let mut histogram: [usize; 26] = [0; 26];

    for i in 0..sequence.len() - 1 {
        histogram[(sequence[i] - 65) as usize] += 1;

        // Init pairs histogram
        let pair = (sequence[i], sequence[i + 1]);
        let freq = pairs_histogram.get(&pair).unwrap_or(&0).clone();
        pairs_histogram.insert(pair, freq + 1);
    }
    histogram[(sequence[sequence.len() - 1] - 65) as usize] += 1;

    for _ in 0..40 {
        println!("{:?}", pairs_histogram);

        let mut new_pairs_histogram = pairs_histogram.clone();

        for pair in pairs_histogram.keys() {
            let freq = pairs_histogram[&pair];
            if rules.contains_key(&pair) {
                let prev_freq = new_pairs_histogram.get(&pair).unwrap_or(&0).clone();
                new_pairs_histogram.insert(*pair, prev_freq - freq);

                let pair_left = (pair.0, rules[&pair]);
                let freq_left = new_pairs_histogram.get(&pair_left).unwrap_or(&0).clone();
                new_pairs_histogram.insert(pair_left, freq_left + freq);
                
                let pair_right = (rules[&pair], pair.1);
                let freq_right = new_pairs_histogram.get(&pair_right).unwrap_or(&0).clone();
                new_pairs_histogram.insert(pair_right, freq_right + freq);

                histogram[(pair_left.1 - 65) as usize] += freq as usize;
            }
        }

        pairs_histogram = new_pairs_histogram;
    }

    //println!("{:?}", pairs_histogram);
    //println!("{:?}", histogram);

    let mut min: (u8, usize) = (0, usize::MAX);
    let mut max: (u8, usize) = (0, 0);

    for (i, value) in histogram.into_iter().enumerate() {
        max = if value > max.1 { (i as u8, value) } else { max };
        min = if value != 0 && value < min.1 { (i as u8, value) } else { min };
    }

    //println!("{:?} {:?}", min, max);

    max.1 - min.1
}

fn main() -> Result<(), std::io::Error> {
    let (sequence, rules) = prepare_aoc_14b_input("input/input.txt")?;
    let result = solve_aoc_14b(&sequence, &rules);

    println!("The result is {}!!", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aoc_14b() {
        let (sequence, rules) = prepare_aoc_14b_input("input/test.txt").unwrap();
        let result = solve_aoc_14b(&sequence, &rules);

        //assert_eq!(result, 1588)
        assert_eq!(result, 2188189693529)
    }
}