use core::panic;
use itertools::{iproduct, Itertools};
use regex::Regex;
use std::io::BufRead;

#[derive(Debug, Clone, Copy)]
enum N<T> {
    Marked(T),
    UnMarked(T),
}

#[derive(Debug)]
struct Board<T> {
    data: Vec<Vec<N<T>>>,
}

impl Board<i64> {
    pub fn winning_line(&self, i: usize) -> bool {
        self.data
            .get(i)
            .unwrap()
            .iter()
            .filter(|n| match n {
                N::Marked(_) => true,
                _ => false,
            })
            .count()
            == 5
    }

    pub fn winning_column(&self, i: usize) -> bool {
        (0..5)
            .map(|l| self.data.get(l).unwrap().get(i).unwrap())
            .filter(|n| match n {
                N::Marked(_) => true,
                _ => false,
            })
            .count()
            == 5
    }

    pub fn won(&self) -> bool {
        (0..5)
            .map(|i| self.winning_column(i) || self.winning_line(i))
            .any(|x| x)
    }

    pub fn sum_unmarked(&self) -> i64 {
        self.data
            .iter()
            .flatten()
            .map(|n| match n {
                N::UnMarked(i) => *i,
                _ => 0,
            })
            .sum()
    }

    pub fn mark(&mut self, v: i64) -> bool {
        for (i, j) in iproduct!(0..5, 0..5) {
            match self.data[i][j] {
                N::UnMarked(n) if n == v => {
                    self.data[i][j] = N::Marked(v);
                    return true;
                }
                _ => (),
            }
        }
        false
    }
}

fn read_data() -> std::io::Result<(Vec<i64>, Vec<Board<i64>>)> {
    let file = std::fs::File::open("resources/day4.txt")?;
    let reader = std::io::BufReader::new(file);
    let re = Regex::new(r"[0-9]+").unwrap();
    let mut lines = reader.lines();

    let draws = lines
        .next()
        .expect("Missing draws")?
        .split(',')
        .map(|d| d.parse().unwrap())
        .collect();

    let mut boards = Vec::new();
    let mut buffer: Vec<Vec<N<i64>>> = Vec::new();

    while let Some(line) = lines.next() {
        match line {
            Ok(l) => {
                let nums: Vec<N<i64>> = re
                    .find_iter(&l)
                    .map(|m| N::UnMarked(m.as_str().parse().unwrap()))
                    .collect();
                if nums.len() > 0 {
                    buffer.push(nums);
                } else if buffer.len() > 0 {
                    boards.push(Board { data: buffer });
                    buffer = Vec::new();
                }
            }
            _ => panic!("Problem when parsing the file"),
        }
    }

    Ok((draws, boards))
}

pub fn part1() -> std::io::Result<()> {
    let (draws, mut boards) = read_data()?;

    'main: for draw in draws {
        for board in boards.iter_mut() {
            println!("{:?}", board);
            if board.mark(draw) {
                if board.won() {
                    println!("{:?}", board.sum_unmarked() * draw);
                    break 'main;
                }
            }
        }
    }

    Ok(())
}

pub fn part2() -> std::io::Result<()> {
    let (draws, mut boards) = read_data()?;
    let mut draw = draws.iter();

    while boards.len() > 1 {
        dbg!(boards.len());
        let e = draw.next().unwrap();

        let mut buff = Vec::new();
        for mut board in boards {
            if board.mark(*e) {
                if !board.won() {
                    buff.push(board)
                }
            } else {
                buff.push(board);
            }
        }
        boards = buff;
    }

    assert_eq!(boards.len(), 1);
    let mut last_board = boards.pop().unwrap();
    while !last_board.won() {
        let e = draw.next().unwrap();
        if last_board.mark(*e) {
            if last_board.won() {
                println!("{:?}", last_board.sum_unmarked() * e);
                break;
            }
        }
    }

    Ok(())
}
