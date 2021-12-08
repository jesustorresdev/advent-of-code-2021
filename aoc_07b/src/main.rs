use ordered_float::OrderedFloat;

fn triangle_number(number: f64) -> f64 {
    (number * (number + 1.0)) / 2.0
}

fn average(positions: &[u32]) -> f64 {
    let f32_positions: Vec<f64> = positions.iter().map(|x| *x as f64).collect();
    f32_positions.iter().fold(0.0, |acc, x| acc + x) / positions.len() as f64
}

fn fuel_wasted(positions: &[u32], place: f64) -> f64 {
    positions.iter().fold(0.0, |acc, pos| acc + triangle_number((*pos as f64 - place).abs()))
}

fn prepare_aoc_07b_input(filename: &str) -> Result<Vec<u32>, std::io::Error> {
    let content = std::fs::read_to_string(filename)?;
    Ok(content.split(",").map(|value| value.parse::<u32>().unwrap()).collect())
}

fn solve_aoc_07b(input: &[u32]) -> f64 {
    //println!("{:?}", input);
    let mut positions = input.to_vec();
    positions.sort_unstable();
    let min = positions[0];
    let max = positions[positions.len() - 1];

    let best = (min..=max).map(|pos| (pos, fuel_wasted(&positions, pos as f64)))
        .min_by_key(|value| OrderedFloat(value.1)).unwrap();

    println!("{:?}", average(&positions));

    best.1
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