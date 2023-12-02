use std::time::Instant;

use advent_of_code_2023::*;

#[cfg(feature = "io")]
macro_rules! input_str {
    ($d:expr) => {
        std::fs::read_to_string(concat!("input/2023/day", $d, ".txt")).unwrap()
    };
}

#[cfg(not(feature = "io"))]
macro_rules! input_str {
    ($d:expr) => {
        include_str!(concat!("../../input/2023/day", $d, ".txt"))
    };
}

macro_rules! run_parts {
    ($m:ident, $d:expr, $g:expr) => {
        let instant = Instant::now();
        let input = input_str!($d);
        let processed_input = $g(&input);
        println!(
            "day {0}-1: {1}\nday {0}-2: {2}",
            $d,
            $m::part_1(&processed_input),
            $m::part_2(&processed_input)
        );

        println!("{:?}\n", instant.elapsed());
    };
}

macro_rules! run_day_with_generator {
    ($m:ident, $d:expr) => {
        run_parts!($m, $d, |i| $m::input_generator(i));
    };
}

macro_rules! run_day {
    ($m:ident, $d:expr) => {
        run_parts!($m, $d, |i| i);
    };
}

pub fn main() {
    let instant = Instant::now();
    run_day!(day_01, "1");
    run_day_with_generator!(day_02, "2");

    println!("done in {:?}", instant.elapsed());
}
