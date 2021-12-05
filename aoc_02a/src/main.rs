use lazy_static::lazy_static;
use std::collections::HashMap;
use std::str::FromStr;

mod vectormath;
pub use crate::vectormath::Vector2;

lazy_static! {
    static ref NAVIGATION_COMMANDS: HashMap<&'static str, vectormath::Vector2> = {
        let mut m = HashMap::new();
        m.insert("forward", Vector2::new(1, 0));
        m.insert("up", Vector2::new(0, -1));
        m.insert("down", Vector2::new(0, 1));
        m
    };
}

fn aoc_02a(input: &str) -> i32 {
    let position_deltas_iter = input
    .lines()
    .filter(|line| !line.is_empty())
    .map(|line| {
        let mut line_iter = line.split_whitespace();
        let direction = NAVIGATION_COMMANDS[line_iter.next().unwrap()];
        let distance = i16::from_str(line_iter.next().unwrap()).unwrap();
        direction * distance
    });

    let mut current_position = Vector2::zero();

    for deltas in position_deltas_iter {
        current_position += deltas;
    }

    current_position.multiply()
}

fn main() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let result = aoc_02a(&input);

    println!("The result is {}!!", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aoc_02a() {
        let input = std::fs::read_to_string("input/test.txt").unwrap();
        assert_eq!(aoc_02a(&input), 150)
    }
}