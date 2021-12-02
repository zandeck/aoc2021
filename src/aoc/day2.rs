use crate::aoc::utils::read_data;
use itertools::Itertools;
use std::io;
use std::str::FromStr;

#[derive(Debug)]
pub enum Move {
    Forward { by: i32 },
    Up { by: i32 },
    Down { by: i32 },
}

impl FromStr for Move {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Move::*;
        match s.split(' ').collect_tuple() {
            Some(("forward", m)) => Ok(Forward {
                by: m.parse().unwrap(),
            }),
            Some(("up", m)) => Ok(Up {
                by: m.parse().unwrap(),
            }),
            Some(("down", m)) => Ok(Down {
                by: m.parse().unwrap(),
            }),
            _ => panic!("unexpected move parsed"),
        }
    }
}

pub fn aggregate_moves(moves: impl Iterator<Item = Move>) -> (i32, i32) {
    moves.fold((0, 0), |(horizontal, depth), m| {
        use Move::*;
        match m {
            Forward { by } => (horizontal + by, depth),
            Up { by } => (horizontal, depth - by),
            Down { by } => (horizontal, depth + by),
        }
    })
}

pub fn aggregate_moves_with_aim(moves: impl Iterator<Item = Move>) -> (i32, i32, i32) {
    moves.fold((0, 0, 0), |(horizontal, depth, aim), m| {
        use Move::*;
        match m {
            Forward { by } => (horizontal + by, depth + aim * by, aim),
            Up { by } => (horizontal, depth, aim - by),
            Down { by } => (horizontal, depth, aim + by),
        }
    })
}

pub fn part1() -> io::Result<()> {
    let data = read_data("resources/day2.txt")?;
    let aggregates = aggregate_moves(data);
    println!("{:?} => {}", aggregates, aggregates.0 * aggregates.1);
    Ok(())
}
pub fn part2() -> io::Result<()> {
    let data = read_data("resources/day2.txt")?;
    let aggregates = aggregate_moves_with_aim(data);
    println!("{:?} => {}", aggregates, aggregates.0 * aggregates.1);
    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn test_part_1() {
        use super::Move::*;
        let moves = [
            Forward { by: 5 },
            Down { by: 5 },
            Forward { by: 8 },
            Up { by: 3 },
            Down { by: 8 },
            Forward { by: 2 },
        ];

        assert_eq!(super::aggregate_moves(moves.into_iter()), (15, 10));
    }
    #[test]
    fn test_part_2() {
        use super::Move::*;
        let moves = [
            Forward { by: 5 },
            Down { by: 5 },
            Forward { by: 8 },
            Up { by: 3 },
            Down { by: 8 },
            Forward { by: 2 },
        ];

        assert_eq!(
            super::aggregate_moves_with_aim(moves.into_iter()),
            (15, 60, 10)
        );
    }
}
