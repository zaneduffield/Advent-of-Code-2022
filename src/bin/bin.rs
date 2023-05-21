use std::time::Instant;

use advent_of_code_2022::*;

macro_rules! input_str {
    ($d:expr) => {
        include_str!(concat!("../../input/2022/day", $d, ".txt"))
    };
}

macro_rules! run_parts {
    ($m:ident, $d:expr, $i:expr, $suffix:expr) => {
        let instant = Instant::now();
        println!(
            "day {}\n  part 1: {}\n  part 2: {}",
            $d,
            $m::part_1($i),
            $m::part_2($i)
        );

        println!("parts completed in {:?}{}\n", instant.elapsed(), $suffix);
    };
}

macro_rules! run_day_with_generator {
    ($m:ident, $d:expr) => {
        let instant = Instant::now();
        let processed_input = $m::input_generator(input_str!($d));
        run_parts!(
            $m,
            $d,
            &processed_input,
            format!(" ({:?} including generator)", instant.elapsed())
        );
    };
}

macro_rules! run_day {
    ($m:ident, $d:expr) => {
        run_parts!($m, $d, input_str!($d), "");
    };
}

pub fn main() {
    let instant = Instant::now();
    run_day_with_generator!(day_01, "1");
    run_day_with_generator!(day_02, "2");
    run_day_with_generator!(day_03, "3");
    run_day_with_generator!(day_04, "4");
    run_day_with_generator!(day_05, "5");
    run_day!(day_06, "6");
    run_day_with_generator!(day_07, "7");
    run_day_with_generator!(day_08, "8");
    run_day!(day_09, "9");
    run_day_with_generator!(day_10, "10");
    run_day_with_generator!(day_11, "11");
    run_day_with_generator!(day_12, "12");
    run_day_with_generator!(day_13, "13");
    run_day_with_generator!(day_14, "14");
    run_day_with_generator!(day_15, "15");
    run_day_with_generator!(day_16, "16");
    run_day_with_generator!(day_17, "17");
    run_day_with_generator!(day_18, "18");
    run_day_with_generator!(day_19, "19");
    run_day_with_generator!(day_20, "20");
    run_day_with_generator!(day_21, "21");
    run_day_with_generator!(day_22, "22");
    run_day_with_generator!(day_23, "23");
    run_day_with_generator!(day_24, "24");

    println!("done in {:?}", instant.elapsed());
}
