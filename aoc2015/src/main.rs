#[macro_use] extern crate lazy_static;
extern crate md5;
extern crate regex;
extern crate serde_json;

mod prob1;
mod prob2;
mod prob3;
mod prob4;
mod prob5;
mod prob6;
mod prob7;
mod prob8;
mod prob9;
mod prob10;
mod prob11;
mod prob12;
mod prob13;
mod prob14;
mod prob15;
mod prob16;
mod prob17;
mod prob18;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() == 0 {
        panic!("Need to specify a problem number");
    }

    let problem_num: u32 = args[1].parse::<u32>().unwrap();

    match problem_num {
        1 => {
            prob1::solve_part_1();
            prob1::solve_part_2();
        }
        2 => {
            prob2::solve_part_1();
            prob2::solve_part_2();
        }
        3 => {
            prob3::solve_part_1();
            prob3::solve_part_2();
        }
        4 => {
            prob4::solve_part_1();
            prob4::solve_part_2();
        }
        5 => {
            prob5::solve_part_1();
            prob5::solve_part_2();
        }
        6 => {
            prob6::solve_part_1();
            prob6::solve_part_2();
        }
        7 => {
            prob7::solve_part_1();
            prob7::solve_part_2();
        }
        8 => {
            println!("Part 1: {}", prob8::solve_part_1());
            println!("Part 2: {}", prob8::solve_part_2());
        }
        9 => {
            println!("Part 1: {}", prob9::solve_part_1());
            println!("Part 2: {}", prob9::solve_part_2());
        }
        10 => {
            println!("Part 1: {}", prob10::solve_part_1());
            println!("Part 2: {}", prob10::solve_part_2());
        }
        11 => {
            println!("Part 1: {}", prob11::solve_part_1().as_str());
            println!("Part 2: {}", prob11::solve_part_2().as_str());
        }
        12 => {
            println!("Part 1: {}", prob12::solve_part_1());
            println!("Part 2: {}", prob12::solve_part_2());
        }
        13 => {
            println!("Part 1: {}", prob13::solve_part_1());
            println!("Part 2: {}", prob13::solve_part_2());
        }
        14 => {
            println!("Part 1: {}", prob14::solve_part_1());
            println!("Part 2: {}", prob14::solve_part_2());
        }
        15 => {
            println!("Part 1: {}", prob15::solve_part_1());
            println!("Part 2: {}", prob15::solve_part_2());
        }
        16 => {
            println!("Part 1: {}", prob16::solve_part_1());
            println!("Part 2: {}", prob16::solve_part_2());
        }
        17 => {
            println!("Part 1: {}", prob17::solve_part_1());
            println!("Part 2: {}", prob17::solve_part_2());
        }
        18 => {
            println!("Part 1: {}", prob18::solve_part_1());
            println!("Part 2: {}", prob18::solve_part_2());
        }
        _ => panic!("Unknown problem number"),
    }
}
