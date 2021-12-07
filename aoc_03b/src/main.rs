fn prepare_aoc_03b_input(filename: &str) -> Result<Vec<u32>, std::io::Error> {
    let content = std::fs::read_to_string(filename)?;
    Ok(content
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| u32::from_str_radix(line, 2).unwrap()).collect())
}

fn filter_input(input: &mut Vec<u32>, mask: u32, more_bits: bool) {
    let mut ones_counter = 0;

    for value in input.iter() {
        ones_counter += if ((*value as u32) & mask) != 0 { 1 } else { -1 };
    }

    if (more_bits && ones_counter >= 0) || (!more_bits && ones_counter < 0) {
        input.retain(|value| value & mask != 0);
    }
    else {
        input.retain(|value| value & mask == 0);
    }
}

fn solve_aoc_03b(input: &[u32], width: u32) -> u32 {
    let mut oxigen_filtered_input = input.to_vec();
    let mut co2_filtered_input = input.to_vec();

    let mut mask: u32 = u32::pow(2, width - 1);

    while mask != 0 {
        if oxigen_filtered_input.len() > 1 {
            filter_input(&mut oxigen_filtered_input, mask, true);
        }
        
        if co2_filtered_input.len() > 1 {
            filter_input(&mut co2_filtered_input, mask, false);
        }

        mask >>= 1;
    }
    
    oxigen_filtered_input.first().unwrap() * co2_filtered_input.first().unwrap()
}

fn main() -> Result<(), std::io::Error> {
    let input = prepare_aoc_03b_input("input/input.txt")?;
    let result = solve_aoc_03b(&input, 12);

    println!("The result is {}!!", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aoc_03b() {
        let input = prepare_aoc_03b_input("input/test.txt").unwrap();
        let result = solve_aoc_03b(&input, 5);

        assert_eq!(result, 230)
    }
}