use general::{get_args, read_trimmed_data_lines, reset_sigpipe};
use ndarray::{Array, Array2, ArrayView};
use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Write};

const BOARD_DIM: usize = 5;
const MATCH: u32 = u32::MAX;

fn winning_board(board: &Array2<u32>) -> bool {
    for row in 0..BOARD_DIM {
        if BOARD_DIM
            == (0..BOARD_DIM)
                .into_iter()
                .filter(|col| board[[row, *col]] == MATCH)
                .count()
        {
            return true;
        }
    }

    for col in 0..BOARD_DIM {
        if BOARD_DIM
            == (0..BOARD_DIM)
                .into_iter()
                .filter(|row| board[[*row, col]] == MATCH)
                .count()
        {
            return true;
        }
    }

    false
}

fn score_board(board: &Array2<u32>) -> u32 {
    board.iter().filter(|&n| *n != MATCH).sum::<u32>()
}

fn update_board(draw: u32, board: &mut Array2<u32>) {
    /*
    for row in 0..BOARD_DIM {
        for col in 0..BOARD_DIM {
            if board[[row, col]] == draw {
                board[[row, col]] = MATCH;
            }
        }
    }
    */
    for elem in board.iter_mut() {
        if *elem == draw {
            *elem = MATCH;
        }
    }
}

fn get_boards(data: &[String]) -> (Vec<u32>, Vec<Array2<u32>>) {
    // random draw is the first line
    let random_draw = data[0]
        .split(',')
        .map(|s| s.trim().parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    // read all the 5x5 boards into an array
    let mut boards = vec![];
    let mut board = Array::zeros((0, BOARD_DIM));
    for (i, line) in data[1..].iter().filter(|s| !s.is_empty()).enumerate() {
        if i % BOARD_DIM == 0 && !board.is_empty() {
            boards.push(board);
            board = Array::zeros((0, BOARD_DIM));
        }
        let row = line
            .split_whitespace()
            .map(|s| s.trim().parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        board.push_row(ArrayView::from(&row)).unwrap();
    }
    if !board.is_empty() {
        boards.push(board);
    }

    // validate all the boards are 5x5 (BOARD_DIM x BOARD_DIM)
    for b in &boards {
        assert_eq!(b.nrows(), BOARD_DIM, "invalid board rows = {}", b.nrows());
        assert_eq!(b.ncols(), BOARD_DIM, "invalid board columns = {}", b.ncols());
    }

    (random_draw, boards)
}

fn get_scores(data: &[String]) -> (Option<u32>, Option<u32>) {
    let (random_draw, mut boards) = get_boards(data);
    //println!("random_draw = {:?}", random_draw);

    for b in &boards {
        assert_eq!(b.nrows(), BOARD_DIM, "invalid board rows = {}", b.nrows());
        assert_eq!(b.ncols(), BOARD_DIM, "invalid board columns = {}", b.ncols());
    }

    let mut score1 = None;
    for draw in random_draw.iter() {
        if score1.is_none() {
            //println!("draw = {}", draw);
            for b in &mut boards {
                update_board(*draw, b);
            }
            for b in &boards {
                if winning_board(b) {
                    //println!("board = {:?}", b);
                    score1 = Some(score_board(b) * draw);
                }
            }
        }
    }

    let mut completed = HashSet::new();
    let mut score2 = None;
    for draw in random_draw.iter() {
        //println!("draw = {}", draw);
        let mut i = 0;
        for b in &mut boards {
            if !completed.contains(&i) {
                update_board(*draw, b);
            }
            i += 1;
        }

        i = 0;
        for b in &boards {
            if !completed.contains(&i) && winning_board(b) {
                //println!("board = {:?}", b);
                completed.insert(i);
                if completed.len() == boards.len() {
                    score2 = Some(score_board(b) * draw);
                }
            }
            i += 1;
        }
    }
    (score1, score2)
}

fn main() -> Result<(), Box<dyn Error>> {
    // behave like a typical unix utility
    reset_sigpipe()?;
    let mut stdout = io::stdout().lock();

    // parse command line arguments
    let args = get_args();

    // read puzzle data into a list of String
    let puzzle_lines = read_trimmed_data_lines(args.get_one::<std::path::PathBuf>("FILE"))?;

    // start a timer
    let timer = std::time::Instant::now();

    // ==============================================================

    let (score1, score2) = get_scores(&puzzle_lines);

    writeln!(stdout, "Answer Part 1 = {}", score1.ok_or("no winner")?)?;
    writeln!(stdout, "Answer Part 2 = {}", score2.ok_or("no winner")?)?;

    if args.get_flag("time") {
        writeln!(stdout, "Total Runtime: {:?}", timer.elapsed())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    fn get_data<T>(filename: &str) -> Result<Vec<T>, Box<dyn std::error::Error>>
    where
        T: FromStr,
        <T as FromStr>::Err: std::error::Error,
        <T as FromStr>::Err: 'static
    {
        let file = Some(std::path::PathBuf::from(filename));
        Ok(read_data_lines::<T>(file)?)
    }
    */

    fn part1(filename: &str) -> u32 {
        let data = read_trimmed_data_lines::<String>(Some(&std::path::PathBuf::from(filename))).unwrap();
        let (score1, _score2) = get_scores(&data);
        score1.unwrap()
    }

    fn part2(filename: &str) -> u32 {
        let data = read_trimmed_data_lines::<String>(Some(&std::path::PathBuf::from(filename))).unwrap();
        let (_score1, score2) = get_scores(&data);
        score2.unwrap()
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1("input-example"), 4512);
    }

    #[test]
    fn part1_actual() {
        assert_eq!(part1("input-actual"), 55770);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2("input-example"), 1924);
    }

    #[test]
    fn part2_actual() {
        assert_eq!(part2("input-actual"), 2980);
    }
}
