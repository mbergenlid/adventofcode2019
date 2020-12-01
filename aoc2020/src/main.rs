mod prob1;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() == 0 {
        panic!("Need to specify a problem number");
    }

    let problem_num: u32 = args[1].parse::<u32>().unwrap();

    match problem_num {
        1 => {
            println!("Part 1: {}", prob1::solve_part_1());
            println!("Part 2: {}", prob1::solve_part_2());
        }
        _ => panic!("Unknown problem number"),
    }
}
