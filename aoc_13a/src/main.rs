use ndarray::{Array, s, Ix2};

#[derive(Debug)] 
enum Axis {
    Y,
    X,
}

#[derive(Debug)] 
struct Fold {
    axis: Axis,
    index: usize,
}

type Page = Array<bool, Ix2>;

fn prepare_aoc_13a_input(filename: &str) -> Result<(Page, Vec<Fold>), std::io::Error> {
    let content = std::fs::read_to_string(filename)?;
    let mut lines = content.lines();
    
    let mut points: Vec<(usize, usize)> = Vec::new();
    let mut max_row: usize = 0;
    let mut max_col: usize = 0;

    for line in &mut lines {
        if line.trim().len() == 0 {
            break;
        }

        let point = line.split_once(",").unwrap();
        let x = point.0.parse::<usize>().unwrap();
        let y = point.1.parse::<usize>().unwrap();
        max_col = if x > max_col { x } else { max_col };
        max_row = if y > max_row { y } else { max_row };
        points.push((x, y));
    }

    let mut page = Array::from_elem((max_row + 1, max_col + 1), false);

    for (x, y) in points {
        page[[y, x]] = true;
    }

    let mut folds: Vec<Fold> = Vec::new();
    
    for line in &mut lines {
        if line.trim().len() == 0 {
            break;
        }

        let part = line.split_once("=").unwrap();
        folds.push(Fold {
            axis: if part.0.chars().last().unwrap() == 'x' { Axis::X } else { Axis::Y },
            index: part.1.parse::<usize>().unwrap(),
        });
    }

    Ok((page, folds))
}

fn solve_aoc_13a(page: Page, folds: Vec<Fold>) -> usize {
    //println!("page: {:?}", page);
    //println!("fold: {:?}", folds);

    let mut folded_page = page;
    for fold in folds {
        match fold.axis {
            Axis::X => {
                let first_part = folded_page.slice(s![.., 0..fold.index]);
                let second_part = folded_page.slice(s![.., (fold.index + 1)..;-1]);
                folded_page = first_part.to_owned() | second_part;
                //println!("page: {:?}", folded_page);
            },
            Axis::Y => {
                let first_part = folded_page.slice(s![0..fold.index, ..]);
                let second_part = folded_page.slice(s![(fold.index + 1)..;-1, ..]);
                //println!("page1: {:?}", first_part);
                //println!("page2: {:?}", second_part);
                folded_page = first_part.to_owned() | second_part;
                //println!("page: {:?}", folded_page);
            },
        }
        break   // Only first fold
    }

    folded_page.iter().filter(|value| **value ).count()
}

fn main() -> Result<(), std::io::Error> {
    let (page, folds) = prepare_aoc_13a_input("input/input.txt")?;
    let result = solve_aoc_13a(page, folds);

    println!("The result is {}!!", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aoc_13a() {
        let (page, folds) = prepare_aoc_13a_input("input/test.txt").unwrap();
        let result = solve_aoc_13a(page, folds);

        assert_eq!(result, 17)
    }
}