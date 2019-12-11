mod intcode;
mod prob1;
mod prob10;
mod prob2;
mod prob3;
mod prob4;
mod prob5;
mod prob6;
mod prob7;
mod prob8;
mod prob9;

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
            prob4::solve_for_part_1();
            prob4::solve_for_part_2();
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
            prob8::solve_part_1();
            prob8::solve_part_2();
        }
        9 => {
            prob9::solve_part_1();
            prob9::solve_part_2();
        }
        10 => {
            prob10::solve_part_1();
            prob10::solve_part_2();
        }
        _ => panic!("Unknown problem number"),
    }
}
