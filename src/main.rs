use std::str::FromStr;

use structopt::StructOpt;

mod aoc;

#[derive(Debug)]
enum Day {
    Day1,
}

type ParseError = &'static str;

impl FromStr for Day {
    type Err = ParseError;
    fn from_str(day: &str) -> Result<Self, Self::Err> {
        match day {
            "1" => Ok(Day::Day1),
            _ => Err("Could not parse a day"),
        }
    }
}

#[derive(Debug)]
enum Part {
    Part1 = 1,
    Part2 = 2,
}

impl FromStr for Part {
    type Err = ParseError;
    fn from_str(day: &str) -> Result<Self, Self::Err> {
        match day {
            "1" => Ok(Part::Part1),
            "2" => Ok(Part::Part2),
            _ => Err("Could not parse which part"),
        }
    }
}

#[derive(Debug, StructOpt)]
#[structopt(name = "Advent of Code 2021", about = "Let's crush it this year")]
struct Opt {
    #[structopt(short)]
    day: Day,
    #[structopt(short)]
    part: Part,
}

impl Opt {
    pub fn run_challenge(&self) -> std::io::Result<()> {
        use aoc::day1;

        println!("Running challenge for {:?} {:?}", &self.day, &self.part);

        let _ = match (&self.day, &self.part) {
            (Day::Day1, Part::Part1) => day1::part1(),
            (Day::Day1, Part::Part2) => day1::part2(),
        }?;

        Ok(())
    }
}

fn main() {
    let opt = Opt::from_args();

    println!("{:?}", opt.run_challenge());
}
