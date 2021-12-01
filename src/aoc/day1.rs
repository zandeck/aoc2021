use itertools::Itertools;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::str::FromStr;

fn read_data() -> io::Result<Vec<i32>> {
    let file = File::open("resources/day1.txt")?;
    let reader = BufReader::new(file);

    reader
        .lines()
        .map_ok(|v| i32::from_str(&v).unwrap())
        .collect()
}

pub fn consecutive_increasing_depth(depths: &[i32]) -> usize {
    depths.iter().tuple_windows().filter(|(a, b)| b > a).count()
}

pub fn rolling_depth_3(depths: &[i32]) -> Vec<i32> {
    depths
        .iter()
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .collect()
}

pub fn part1() -> io::Result<()> {
    let data = read_data()?;
    println!("{}", consecutive_increasing_depth(&data));
    Ok(())
}

pub fn part2() -> io::Result<()> {
    let data = read_data()?;
    let rolling_window = rolling_depth_3(&data);
    println!("{}", consecutive_increasing_depth(&rolling_window));
    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn part1() {
        let depths = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(super::consecutive_increasing_depth(&depths), 7);
    }

    #[test]
    fn part2() {
        let depths = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let rolling = super::rolling_depth_3(&depths);
        let result = vec![607, 618, 618, 617, 647, 716, 769, 792];
        assert_eq!(rolling, result);
        assert_eq!(super::consecutive_increasing_depth(&rolling), 5);
    }
}
