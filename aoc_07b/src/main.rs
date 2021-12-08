fn triangle_number(number: f32) -> f32 {
    (number * (number + 1.0)) / 2.0
}

fn average(numbers: &[f32]) -> f32 {
    let numerator = numbers.iter().fold(0.0, |acc, n| acc + n);
    numerator / (numbers.len() as f32)
}

fn fuel_wasted(positions: &[f32], final_place: f32) -> f32 {
    positions.iter().fold(0.0, |acc, pos| acc + triangle_number((pos - final_place).abs()))
}

fn calculate_best(positions: &[f32]) -> f32 {
    let num_of_positions = positions.len() as f32;
    let median = positions[positions.len() / 2];
    let mean = average(&positions);

    let mut current_place: f32 = median;
    let mut prev_place: f32 = 0.0;
    let mut prev_epsilon: f32 = 0.0;
    
    loop {
        let num_of_greater = positions.iter().filter(|&value| *value > current_place).count() as f32;
        let num_of_less = positions.iter().filter(|&value| *value < current_place).count() as f32;

        let current_epsilon = current_place - mean + (num_of_less - num_of_greater) / (num_of_positions * 2.0);
        println!("{:?}", current_place);
        println!("{:?}", current_epsilon);

        if current_epsilon < 0.0 {
            if prev_epsilon > 0.0 {
                return if current_epsilon.abs() > prev_epsilon { prev_place } else { current_place };
            }

            prev_place = current_place;
            current_place += 1.0;
            prev_epsilon = current_epsilon;
        }
        else if current_epsilon > 0.0 {
            if prev_epsilon < 0.0 {
                return if current_epsilon > prev_epsilon.abs() { prev_place } else { current_place };
            }

            prev_place = current_place;
            current_place -= 1.0;
            prev_epsilon = current_epsilon;
        }
        else  {
            return current_place;
        }
    }
}

fn prepare_aoc_07b_input(filename: &str) -> Result<Vec<u32>, std::io::Error> {
    let content = std::fs::read_to_string(filename)?;
    Ok(content.split(",").map(|value| value.parse::<u32>().unwrap()).collect())
}

fn solve_aoc_07b(input: &[u32]) -> f32 {
    //println!("{:?}", input);
    let mut positions: Vec<f32> = input.iter().map(|value| *value as f32).collect();
    positions.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let final_place = calculate_best(&positions);
    fuel_wasted(&positions, final_place)
}

fn main() -> Result<(), std::io::Error> {
    let input = prepare_aoc_07b_input("input/input.txt")?;
    let result = solve_aoc_07b(&input);

    println!("The result is {}!!", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aoc_07b() {
        let input = prepare_aoc_07b_input("input/test.txt").unwrap();
        let result = solve_aoc_07b(&input);

        assert_eq!(result, 168.0)
    }
}