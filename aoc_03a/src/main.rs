fn prepare_aoc_03a_input(filename: &str) -> Result<Vec<u32>, std::io::Error> {
    let content = std::fs::read_to_string(filename)?;
    Ok(content
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| u32::from_str_radix(line, 2).unwrap()).collect())
}

fn solve_aoc_03a(input: &[u32], width: u32) -> u32 {
    let mut gamma_rate: u32 = 0;
    let mut epsilon_rate: u32 = 0;
    let mut mask: u32 = u32::pow(2, width - 1);

    while mask != 0 {
        let mut ones_counter = 0;
        
        for value in input {
            ones_counter += if ((*value as u32) & mask) != 0 { 1 } else { -1 };
        }

        if ones_counter >= 0 {
            gamma_rate |= mask;
        }
        else {
            epsilon_rate |= mask;
        }

        mask >>= 1;
    }
    
    gamma_rate * epsilon_rate
}

fn main() -> Result<(), std::io::Error> {
    let input = prepare_aoc_03a_input("input/input.txt")?;
    let result = solve_aoc_03a(&input, 12);

    println!("The result is {}!!", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aoc_02a() {
        let input = prepare_aoc_03a_input("input/test.txt").unwrap();
        let result = solve_aoc_03a(&input, 5);

        assert_eq!(result, 198)
    }
}