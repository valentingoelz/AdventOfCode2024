mod y2024;

use std::os::unix::prelude::OsStrExt;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use std::{fmt, fs, io};

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(required = true)]
    files: Vec<PathBuf>,
}

macro_rules! days {
    ( $( $day:ident : $name:expr => $path:path, )* ) => {
        enum Day { $( $day, )* }

        impl fmt::Display for Day {
            fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $( Self::$day => $name, )*
                }.fmt(f)
            }
        }

        impl FromStr for Day {
            type Err = ();
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok(match s {
                    $( $name => Self::$day, )*
                    _ => return Err(()),
                })
            }
        }

        impl Day {
            fn solve(self, input: String) {
                match self {
                    $( Self::$day => $path(input), )*
                }
            }
        }
    };
}

days! {
    Y2024D01: "01" => y2024::day01::solve,
    Y2024D02: "02" => y2024::day02::solve,
    Y2024D03: "03" => y2024::day03::solve,
    Y2024D04: "04" => y2024::day04::solve,
    Y2024D05: "05" => y2024::day05::solve,
    Y2024D06: "06" => y2024::day06::solve,
    Y2024D07: "07" => y2024::day07::solve,
    Y2024D08: "08" => y2024::day08::solve,
    Y2024D09: "09" => y2024::day09::solve,
    Y2024D10: "10" => y2024::day10::solve,
    Y2024D11: "11" => y2024::day11::solve,
    Y2024D19: "19" => y2024::day19::solve,
}

impl Day {
    fn from_path(path: &Path) -> Option<Self> {
        let file_name = path.file_stem()?.as_bytes();
        let file_name = std::str::from_utf8(file_name).ok()?;
        let day = file_name.strip_prefix("day")?;
        let day = day.parse().ok()?;
        Some(day)
    }
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    for file in args.files {
        let day = match Day::from_path(&file) {
            Some(day) => day,
            None => {
                eprintln!("Could not parse day from file {:?}", file);
                continue;
            }
        };

        eprintln!("Solving day {}", day);
        let input = fs::read_to_string(file)?;
        day.solve(input);
        eprintln!();
    }
    Ok(())
}
