use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const BOARD_COLS: usize = 5;
const BOARD_ROWS: usize = 5;

type RandomSequence = Vec<u8>;
type Board = Vec<(u8, bool)>;

fn read_line(reader: &mut dyn BufRead, buf: &mut String) -> Result<usize, std::io::Error> {
    buf.clear();
    let num_reads = reader.read_line(buf)?;
    
    if num_reads > 0 {
        // Remove the last 0x0A
        buf.pop();  
    }

    Ok(num_reads)
}

fn prepare_aoc_04b_input(filename: &str) -> Result<(RandomSequence, Vec<Board>), std::io::Error> {
    let input_file = File::open(filename)?;
    let mut reader = BufReader::new(input_file);

    let mut line = String::new();

    // Random sequence
    read_line(&mut reader, &mut line)?;
    let random_sequence: Vec<u8> = line.split(",").map(|value| value.parse::<u8>().unwrap()).collect();
    //println!("{:?}", random_sequence);

    let mut boards: Vec<Board> = Vec::new();

    let mut num_reads = read_line(&mut reader, &mut line)?;
    while num_reads !=0 && line.is_empty() {
        
        let mut board: Board = Board::new();

        loop {
            num_reads = read_line(&mut reader, &mut line)?;
            if line.is_empty() {
                break;
            }

            let numbers_iter = line.trim().split_whitespace().map(|value| (value.parse::<u8>().unwrap(), false));
            board.extend(numbers_iter);
        }

        boards.push(board);
    }

    //println!("{:?}", boards);

    Ok((random_sequence, boards))
}

fn solve_aoc_04b(random_sequence: &RandomSequence, boards: &Vec<Board>) -> usize {
    let mut mut_boards = boards.clone();

    let mut cols_status: Vec<[u8; BOARD_COLS]> = Vec::new();
    let mut rows_status: Vec<[u8; BOARD_ROWS]> = Vec::new();
    
    let mut completed_boards: Vec<(usize, u8)> = Vec::new();
    
    cols_status.resize(boards.len(), [BOARD_COLS as u8; BOARD_ROWS]);
    rows_status.resize(boards.len(), [BOARD_ROWS as u8; BOARD_COLS]);

    for number in random_sequence {
        for num_of_board in 0..mut_boards.len() {
            if completed_boards.iter().position(|&value| value.0 == num_of_board).is_some() {
                continue;
            }

            let board = &mut mut_boards[num_of_board];
            match board.iter().position(|&value| value.0 == *number) {
                Some(pos) => {
                    board[pos].1 = true;

                    let row = pos / BOARD_COLS;
                    let col = pos % BOARD_COLS;

                    cols_status[num_of_board][col] -= 1;
                    rows_status[num_of_board][row] -= 1;

                    if cols_status[num_of_board][col] == 0 || rows_status[num_of_board][row] == 0 {
                        completed_boards.push((num_of_board, *number)); 
                    }
                },
                None => continue
            }
        }
    }

    let last_completed_board = completed_boards.last().unwrap();

    return last_completed_board.1 as usize *
        mut_boards[last_completed_board.0].iter().filter(|value| !value.1).fold(0, |acc, value| acc + value.0 as usize);
}

fn main() -> Result<(), std::io::Error> {
    let (random_sequence, boards) = prepare_aoc_04b_input("input/input.txt")?;
    let result = solve_aoc_04b(&random_sequence, &boards);

    println!("The result is {}!!", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aoc_04b() {
        let (random_sequence, boards) = prepare_aoc_04b_input("input/test.txt").unwrap();
        let result = solve_aoc_04b(&random_sequence, &boards);

        assert_eq!(result, 1924)
    }
}