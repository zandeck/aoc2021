use crate::aoc::utils::read_data;
use std::io;

pub fn aggregate<T: AsRef<str>>(data: &[T]) -> Vec<i64> {
    let mut res = vec![0; data[0].as_ref().len()];
    data.iter().for_each(|s| {
        for (i, c) in s.as_ref().chars().enumerate() {
            res[i] += if c == '1' { 1 } else { 0 };
        }
    });
    res
}

pub fn occur_most(bits: &[i64], n: i64) -> Vec<i64> {
    bits.iter()
        .map(move |i| if *i >= (n - *i) { 1 } else { 0 })
        .collect()
}

pub fn get_gamma_rate(bits: &[i64], n: i64) -> i64 {
    let l = bits.len();
    occur_most(bits, n)
        .iter()
        .enumerate()
        .fold(0, |acc, (i, x)| acc + x * 2_i64.pow((l - i - 1) as u32))
}

pub fn get_occurence_most(data: &Vec<String>) -> String {
    let mut res = data.clone();
    let mut i = 0;

    while res.len() != 1 {
        let letters = res.iter().map(|s| s.chars().nth(i).unwrap());
        let (zeros, ones) = letters.fold((0, 0), |acc, c| {
            if c == '1' {
                (acc.0, acc.1 + 1)
            } else {
                (acc.0 + 1, acc.1)
            }
        });
        res = if ones >= zeros {
            res.into_iter()
                .filter(|s| s.chars().nth(i) == Some('1'))
                .collect()
        } else {
            res.into_iter()
                .filter(|s| s.chars().nth(i) == Some('0'))
                .collect()
        };
        i += 1;
    }
    res.get(0).unwrap().clone()
}

pub fn get_occurence_least(data: &Vec<String>) -> String {
    let mut res = data.clone();
    let mut i = 0;

    while res.len() != 1 {
        let letters = res.iter().map(|s| s.chars().nth(i).unwrap());
        let (zeros, ones) = letters.fold((0, 0), |acc, c| {
            if c == '1' {
                (acc.0, acc.1 + 1)
            } else {
                (acc.0 + 1, acc.1)
            }
        });
        res = if ones < zeros {
            res.into_iter()
                .filter(|s| s.chars().nth(i) == Some('1'))
                .collect()
        } else {
            res.into_iter()
                .filter(|s| s.chars().nth(i) == Some('0'))
                .collect()
        };
        i += 1;
    }
    res.get(0).unwrap().clone()
}

pub fn str_to_i64(s: &str) -> i64 {
    let n = s.len();
    s.chars().enumerate().fold(0, |acc, (i, x)| {
        acc + if x == '1' {
            2_i64.pow((n - i - 1) as u32)
        } else {
            0
        }
    })
}

pub fn get_max_nb(n: i64) -> i64 {
    (0..n).fold(0, |acc, i| acc + 2_i64.pow(i as u32))
}

pub fn part1() -> io::Result<()> {
    let data = read_data("resources/day3.txt")?.collect::<Vec<String>>();
    let aggregated = aggregate(&data[..]);
    let gamma_rate = get_gamma_rate(&aggregated[..], data.len() as i64);
    let epsilon_rate = get_max_nb(aggregated.len() as i64) - gamma_rate;
    println!("Power: {:?}", gamma_rate * epsilon_rate);
    Ok(())
}
pub fn part2() -> io::Result<()> {
    let data = read_data("resources/day3.txt")?.collect::<Vec<String>>();
    let (most, least) = (get_occurence_most(&data), get_occurence_least(&data));
    println!("Oxygen Generator rating: {:?}", str_to_i64(&most));
    println!("CO2 Scrubber rating: {:?}", str_to_i64(&least));
    println!(
        "Life Support rating: {:?}",
        str_to_i64(&most) * str_to_i64(&least)
    );

    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn test_aggregate() {
        let data = [
            "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000",
            "11001", "00010", "01010",
        ];
        assert_eq!(super::aggregate(&data), [7, 5, 8, 7, 5]);
    }

    #[test]
    fn test_gamma_rate() {
        let data = [7, 5, 8, 7, 5];
        assert_eq!(super::get_gamma_rate(&data, 12), 22);
    }

    #[test]
    fn test_get_max_nb() {
        assert_eq!(super::get_max_nb(5), 31);
    }

    #[test]
    fn test_power() {
        assert_eq!(super::get_max_nb(5), 31);
    }

    #[test]
    fn test_str_to_i64() {
        assert_eq!(super::str_to_i64("01010"), 10);
        assert_eq!(super::str_to_i64("10111"), 23);
    }

    #[test]
    fn test_example() {
        use super::*;
        let data = vec![
            "00100".into(),
            "11110".into(),
            "10110".into(),
            "10111".into(),
            "10101".into(),
            "01111".into(),
            "00111".into(),
            "11100".into(),
            "10000".into(),
            "11001".into(),
            "00010".into(),
            "01010".into(),
        ];
        let (most, least) = (get_occurence_most(&data), get_occurence_least(&data));
        assert_eq!(most, "10111");
        assert_eq!(least, "01010");
        assert_eq!(str_to_i64(&most) * str_to_i64(&least), 230);
    }
}
