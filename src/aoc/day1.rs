use itertools::Itertools;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::str::FromStr;

fn read_data() -> io::Result<impl Iterator<Item = i32>> {
    let file = File::open("resources/day1.txt")?;
    let reader = BufReader::new(file);

    Ok(reader.lines().map(|v| match v {
        Ok(s) => i32::from_str(&s).unwrap(),
        Err(_) => panic!("Error reading lines from file"),
    }))
}

pub fn consecutive_increasing_depth(depths: impl Iterator<Item = i32>) -> usize {
    depths.tuple_windows().filter(|(a, b)| b > a).count()
}

pub fn rolling_depth_3(depths: impl Iterator<Item = i32>) -> impl Iterator<Item = i32> {
    depths.tuple_windows().map(|(a, b, c)| a + b + c)
}

pub fn part1() -> io::Result<()> {
    let data = read_data()?;
    println!("{}", consecutive_increasing_depth(data));
    Ok(())
}

pub fn part2() -> io::Result<()> {
    let data = read_data()?;
    let rolling_window = rolling_depth_3(data);
    println!("{}", consecutive_increasing_depth(rolling_window));
    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn part1() {
        let depths = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(super::consecutive_increasing_depth(depths.into_iter()), 7);
    }

    #[test]
    fn part2() {
        let depths = [199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        let rolling = super::rolling_depth_3(depths.into_iter());
        // let result = vec![607, 618, 618, 617, 647, 716, 769, 792];
        assert_eq!(super::consecutive_increasing_depth(rolling), 5);
    }
}
