use std::ops::Index;
use std::ops::IndexMut;

pub struct FishesVec {
    array: [u64; 9],
    first_index: usize
}

impl FishesVec {
    pub fn new(times: &[u64]) -> Self{
        let mut fishes = Self {
            array: [0; 9],
            first_index: 0
        };

        for time in times {
            fishes.array[*time as usize] += 1;
        }

        fishes
    }
    
    pub fn shift(&mut self, amount: usize) {
        self.first_index = (self.first_index + amount) % self.array.len()
    }

    pub fn total(&self) -> usize {
        self.array.iter().fold(0, |acc, x| acc + *x as usize)
    }
}

impl Default for FishesVec {
    fn default() -> Self {
        Self {
            array: [0; 9],
            first_index: 0
        }
    }
}

impl Index<usize> for FishesVec {
    type Output = u64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.array[(self.first_index + index) % self.array.len()]
    }
}

impl IndexMut<usize> for FishesVec {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { 
        &mut self.array[(self.first_index + index) % self.array.len()]
    }
}

fn prepare_aoc_06b_input(filename: &str) -> Result<Vec<u64>, std::io::Error> {
    let content = std::fs::read_to_string(filename)?;
    Ok(content.split(",").map(|value| value.parse::<u64>().unwrap()).collect())
}

fn solve_aoc_06b(input: &[u64]) -> usize {
    let mut fishes = FishesVec::new(input);
    //println!("{:?}", fishes.array);

    for _ in 1..256 {
        //println!("{:?}", fishes.array);
        fishes.shift(1);

        fishes[7] += fishes[0];

        // This happens automatically in the next shift
        // fishes[9] = fishes[0];
        // fishes[0] = 0;
    }

    fishes.total()
}

fn main() -> Result<(), std::io::Error> {
    let input = prepare_aoc_06b_input("input/input.txt")?;
    let result = solve_aoc_06b(&input);

    println!("The result is {}!!", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aoc_06b() {
        let input = prepare_aoc_06b_input("input/test.txt").unwrap();
        let result = solve_aoc_06b(&input);

        assert_eq!(result, 26984457539)
    }
}