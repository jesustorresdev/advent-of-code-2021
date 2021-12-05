mod vectormath;
pub use crate::vectormath::Vector2;

fn aoc_02b(input: &str) -> i32 {
    let commands_iter = input
    .lines()
    .filter(|line| !line.is_empty())
    .map(|line| {
        let mut line_iter = line.split_whitespace();
        (
            line_iter.next().unwrap(),
            line_iter.next().unwrap().parse::<i32>().unwrap()
        )
    });

    let mut current_position = Vector2::zero();
    let mut current_aim: i32 = 0;

    for command in commands_iter {
        match command.0 {
            "up" => current_aim -= command.1,
            "down" => current_aim += command.1,
            "forward" => current_position += Vector2::new(command.1, current_aim * command.1),
            _ => println!("Unknown command!!")
        }
    }

    current_position.multiply()
}

fn main() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let result = aoc_02b(&input);

    println!("The result is {}!!", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aoc_02a() {
        let input = std::fs::read_to_string("input/test.txt").unwrap();
        assert_eq!(aoc_02b(&input), 900)
    }
}