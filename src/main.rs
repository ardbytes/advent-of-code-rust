use aoc::util::ansi::*;
use aoc::util::parse::*;
use aoc::*;
use std::env::args;
use std::fs::read_to_string;
use std::iter::empty;
use std::path::PathBuf;
use std::time::Duration;
use std::time::Instant;

fn main() {
    // Parse command line options
    let (year, day) = match args().nth(1) {
        Some(arg) => {
            let str = arg.as_str();
            let mut iter = str.iter_unsigned();
            (iter.next(), iter.next())
        }
        None => (None, None),
    };

    // Filter solutions
    let solutions: Vec<_> = empty()
        .chain(year2015())
        .chain(year2016())
        .chain(year2017())
        .chain(year2018())
        .chain(year2019())
        .chain(year2020())
        .chain(year2021())
        .chain(year2022())
        .chain(year2023())
        .filter(|solution| year == Some(solution.year) || year.is_none())
        .filter(|solution| day == Some(solution.day) || day.is_none())
        .collect();

    // Pretty print output and timing for each solution
    let mut elapsed = Duration::ZERO;

    for Solution { year, day, wrapper } in &solutions {
        let path: PathBuf =
            ["input", &format!("year{year}"), &format!("day{day:02}.txt")].iter().collect();

        if let Ok(data) = read_to_string(&path) {
            let time = Instant::now();
            let (answer1, answer2) = wrapper(&data);
            let duration = time.elapsed().as_micros();
            elapsed += time.elapsed();

            println!("{BOLD}{YELLOW}{year} Day {day:02}{RESET}");
            println!("    Part 1: {answer1}");
            println!("    Part 2: {answer2}");
            println!("    Duration: {duration} μs");
        } else {
            eprintln!("{BOLD}{RED}{year} Day {day:02}{RESET}");
            eprintln!("    Missing input!");
            eprintln!("    Place input file in {BOLD}{WHITE}{}{RESET}", path.display());
        }
    }

    // Print totals
    println!("{BOLD}{RED}Solutions: {}{RESET}", solutions.len());
    println!("{BOLD}{GREEN}Elapsed: {} ms{RESET}", elapsed.as_millis());
}

struct Solution {
    year: u32,
    day: u32,
    wrapper: fn(&str) -> (String, String),
}

macro_rules! solution {
    ($year:tt, $day:tt) => {
        Solution {
            year: (&stringify!($year)).unsigned(),
            day: (&stringify!($day)).unsigned(),
            wrapper: |data: &str| {
                use $year::$day::*;

                let input = parse(&data);
                let part1 = part1(&input).to_string();
                let part2 = part2(&input).to_string();

                (part1, part2)
            },
        }
    };
}

fn year2015() -> Vec<Solution> {
    vec![
        solution!(year2015, day01),
        solution!(year2015, day02),
        solution!(year2015, day03),
        solution!(year2015, day04),
        solution!(year2015, day05),
        solution!(year2015, day06),
        solution!(year2015, day07),
        solution!(year2015, day08),
        solution!(year2015, day09),
        solution!(year2015, day10),
        solution!(year2015, day11),
        solution!(year2015, day12),
        solution!(year2015, day13),
        solution!(year2015, day14),
        solution!(year2015, day15),
        solution!(year2015, day16),
        solution!(year2015, day17),
        solution!(year2015, day18),
        solution!(year2015, day19),
        solution!(year2015, day20),
        solution!(year2015, day21),
        solution!(year2015, day22),
        solution!(year2015, day23),
        solution!(year2015, day24),
        solution!(year2015, day25),
    ]
}

fn year2016() -> Vec<Solution> {
    vec![
        solution!(year2016, day01),
        solution!(year2016, day02),
        solution!(year2016, day03),
        solution!(year2016, day04),
        solution!(year2016, day05),
        solution!(year2016, day06),
        solution!(year2016, day07),
        solution!(year2016, day08),
        solution!(year2016, day09),
        solution!(year2016, day10),
        solution!(year2016, day11),
        solution!(year2016, day12),
        solution!(year2016, day13),
        solution!(year2016, day14),
        solution!(year2016, day15),
        solution!(year2016, day16),
        solution!(year2016, day17),
        solution!(year2016, day18),
        solution!(year2016, day19),
        solution!(year2016, day20),
        solution!(year2016, day21),
        solution!(year2016, day22),
        solution!(year2016, day23),
        solution!(year2016, day24),
        solution!(year2016, day25),
    ]
}

fn year2017() -> Vec<Solution> {
    vec![
        solution!(year2017, day01),
        solution!(year2017, day02),
        solution!(year2017, day03),
        solution!(year2017, day04),
        solution!(year2017, day05),
        solution!(year2017, day06),
        solution!(year2017, day07),
        solution!(year2017, day08),
        solution!(year2017, day09),
        solution!(year2017, day10),
        solution!(year2017, day11),
        solution!(year2017, day12),
        solution!(year2017, day13),
        solution!(year2017, day14),
        solution!(year2017, day15),
        solution!(year2017, day16),
        solution!(year2017, day17),
        solution!(year2017, day18),
        solution!(year2017, day19),
        solution!(year2017, day20),
        solution!(year2017, day21),
        solution!(year2017, day22),
        solution!(year2017, day23),
        solution!(year2017, day24),
        solution!(year2017, day25),
    ]
}

fn year2018() -> Vec<Solution> {
    vec![
        solution!(year2018, day01),
        solution!(year2018, day02),
        solution!(year2018, day03),
        solution!(year2018, day04),
        solution!(year2018, day05),
    ]
}

fn year2019() -> Vec<Solution> {
    vec![
        solution!(year2019, day01),
        solution!(year2019, day02),
        solution!(year2019, day03),
        solution!(year2019, day04),
        solution!(year2019, day05),
        solution!(year2019, day06),
        solution!(year2019, day07),
        solution!(year2019, day08),
        solution!(year2019, day09),
        solution!(year2019, day10),
        solution!(year2019, day11),
        solution!(year2019, day12),
        solution!(year2019, day13),
        solution!(year2019, day14),
        solution!(year2019, day15),
        solution!(year2019, day16),
        solution!(year2019, day17),
        solution!(year2019, day18),
        solution!(year2019, day19),
        solution!(year2019, day20),
        solution!(year2019, day21),
        solution!(year2019, day22),
        solution!(year2019, day23),
        solution!(year2019, day24),
        solution!(year2019, day25),
    ]
}

fn year2020() -> Vec<Solution> {
    vec![
        solution!(year2020, day01),
        solution!(year2020, day02),
        solution!(year2020, day03),
        solution!(year2020, day04),
        solution!(year2020, day05),
        solution!(year2020, day06),
        solution!(year2020, day07),
        solution!(year2020, day08),
        solution!(year2020, day09),
        solution!(year2020, day10),
        solution!(year2020, day11),
        solution!(year2020, day12),
        solution!(year2020, day13),
        solution!(year2020, day14),
        solution!(year2020, day15),
        solution!(year2020, day16),
        solution!(year2020, day17),
        solution!(year2020, day18),
        solution!(year2020, day19),
        solution!(year2020, day20),
        solution!(year2020, day21),
        solution!(year2020, day22),
        solution!(year2020, day23),
        solution!(year2020, day24),
        solution!(year2020, day25),
    ]
}

fn year2021() -> Vec<Solution> {
    vec![
        solution!(year2021, day01),
        solution!(year2021, day02),
        solution!(year2021, day03),
        solution!(year2021, day04),
        solution!(year2021, day05),
        solution!(year2021, day06),
        solution!(year2021, day07),
        solution!(year2021, day08),
        solution!(year2021, day09),
        solution!(year2021, day10),
        solution!(year2021, day11),
        solution!(year2021, day12),
        solution!(year2021, day13),
        solution!(year2021, day14),
        solution!(year2021, day15),
        solution!(year2021, day16),
        solution!(year2021, day17),
        solution!(year2021, day18),
        solution!(year2021, day19),
        solution!(year2021, day20),
        solution!(year2021, day21),
        solution!(year2021, day22),
        solution!(year2021, day23),
        solution!(year2021, day24),
        solution!(year2021, day25),
    ]
}

fn year2022() -> Vec<Solution> {
    vec![
        solution!(year2022, day01),
        solution!(year2022, day02),
        solution!(year2022, day03),
        solution!(year2022, day04),
        solution!(year2022, day05),
        solution!(year2022, day06),
        solution!(year2022, day07),
        solution!(year2022, day08),
        solution!(year2022, day09),
        solution!(year2022, day10),
        solution!(year2022, day11),
        solution!(year2022, day12),
        solution!(year2022, day13),
        solution!(year2022, day14),
        solution!(year2022, day15),
        solution!(year2022, day16),
        solution!(year2022, day17),
        solution!(year2022, day18),
        solution!(year2022, day19),
        solution!(year2022, day20),
        solution!(year2022, day21),
        solution!(year2022, day22),
        solution!(year2022, day23),
        solution!(year2022, day24),
        solution!(year2022, day25),
    ]
}

fn year2023() -> Vec<Solution> {
    vec![
        solution!(year2023, day01),
        solution!(year2023, day02),
        solution!(year2023, day03),
        solution!(year2023, day04),
        solution!(year2023, day05),
        solution!(year2023, day06),
        solution!(year2023, day07),
        solution!(year2023, day08),
        solution!(year2023, day09),
        solution!(year2023, day10),
        solution!(year2023, day11),
        solution!(year2023, day12),
        solution!(year2023, day13),
        solution!(year2023, day14),
        solution!(year2023, day15),
        solution!(year2023, day16),
        solution!(year2023, day17),
        solution!(year2023, day18),
        solution!(year2023, day19),
        solution!(year2023, day20),
        solution!(year2023, day21),
        solution!(year2023, day22),
        solution!(year2023, day23),
        solution!(year2023, day24),
        solution!(year2023, day25),
    ]
}
