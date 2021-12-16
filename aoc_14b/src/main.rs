use regex::Regex;

const FIRST_ALPHA: u8 = 65;
const NUM_OF_CHARS: usize = 26;
const NUM_OF_PAIRS: usize = NUM_OF_CHARS * NUM_OF_CHARS;

fn prepare_aoc_14b_input(filename: &str) -> Result<(Vec<u8>, Vec<u8>), std::io::Error> {
    let content = std::fs::read_to_string(filename)?;
    let mut lines = content.lines();

    let sequence = lines.next().unwrap().bytes().map(|c| alpha_to_id(c)).collect();
    lines.next();

    let line_regex = Regex::new(r"(.)(.) -> (.)").unwrap();
    let mut rules: Vec<u8> = vec![255; NUM_OF_PAIRS];
    for line in lines.filter(|line| !line.is_empty()) {
        let captures = line_regex.captures(line).unwrap();
        let pair_first: u8 = alpha_to_id(captures.get(1).unwrap().as_str().as_bytes()[0]);
        let pair_second: u8 = alpha_to_id(captures.get(2).unwrap().as_str().as_bytes()[0]);
        let element = alpha_to_id(captures.get(3).unwrap().as_str().as_bytes()[0]);
        rules[pair_to_id((pair_first, pair_second))] = element;
    }

    Ok((sequence, rules))
}

fn alpha_to_id(c: u8) -> u8 {
    c - FIRST_ALPHA
}

fn pair_to_id(pair: (u8, u8)) -> usize {
    pair.0 as usize * NUM_OF_CHARS + pair.1 as usize
}

fn update_histogram(start_pair: (u8, u8), rules: &[u8], histogram: &mut [usize; 26], step: usize) {
    let mut next_pairs: Vec<((u8, u8), usize)> = Vec::with_capacity(step * 2);
    next_pairs.push((start_pair, step));

    while next_pairs.len() > 0 {
        let (next_pair, step) = next_pairs.pop().unwrap();
        let new_item = rules[pair_to_id(next_pair)];
        if new_item != 255 {
            histogram[new_item as usize] += 1;
            if step > 1 {
                next_pairs.push(((next_pair.0, new_item), step - 1));
                next_pairs.push(((new_item, next_pair.1), step - 1));
            }
        }
    }
}

fn solve_aoc_14b(sequence: &[u8], rules: &[u8]) -> usize {
    //println!("{:?}\n{:?}", sequence, rules);

    let mut histogram: [usize; NUM_OF_CHARS] = [0; NUM_OF_CHARS];
    
    histogram[sequence[0] as usize] += 1;
    for i in 0..sequence.len() - 1 {
        let pair: (u8, u8) = (sequence[i], sequence[i+1]);
        histogram[pair.1 as usize] += 1;
        update_histogram(pair, &rules, &mut histogram, 40);
        //update_histogram(pair, &rules, &mut histogram, 40);
    }
    
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

        assert_eq!(result, 1588)
        //assert_eq!(result, 2188189693529)
    }
}
