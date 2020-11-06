use std::iter::Skip;

pub fn solve_part_1() {
    let signal = phases(
        vec![
            5, 9, 7, 1, 7, 2, 3, 8, 1, 6, 8, 5, 8, 0, 0, 1, 0, 5, 9, 9, 0, 1, 2, 5, 2, 7, 5, 1, 0,
            9, 4, 3, 1, 4, 9, 3, 4, 7, 9, 3, 0, 7, 4, 2, 8, 2, 2, 8, 9, 9, 6, 3, 8, 2, 4, 7, 0, 8,
            3, 0, 0, 5, 8, 5, 5, 4, 8, 3, 8, 6, 7, 4, 8, 4, 3, 5, 6, 0, 5, 5, 4, 8, 9, 4, 1, 9, 9,
            1, 3, 5, 1, 2, 7, 2, 1, 0, 9, 5, 5, 6, 1, 6, 5, 5, 2, 6, 5, 1, 0, 7, 7, 4, 5, 9, 7, 2,
            7, 3, 9, 4, 6, 4, 2, 6, 8, 8, 4, 6, 3, 7, 4, 7, 2, 8, 3, 9, 3, 5, 0, 7, 5, 0, 9, 8, 4,
            0, 8, 5, 4, 1, 0, 9, 8, 0, 3, 7, 1, 8, 8, 0, 2, 7, 8, 0, 5, 4, 3, 2, 9, 8, 1, 4, 1, 3,
            9, 8, 6, 4, 4, 9, 5, 5, 5, 0, 6, 1, 4, 9, 9, 1, 4, 7, 9, 6, 7, 7, 5, 8, 8, 5, 2, 4, 6,
            6, 0, 2, 1, 2, 3, 7, 4, 6, 8, 6, 6, 2, 2, 3, 5, 2, 8, 3, 5, 6, 4, 9, 3, 0, 1, 2, 1, 3,
            6, 1, 5, 2, 9, 7, 4, 2, 1, 8, 7, 2, 0, 5, 4, 2, 2, 9, 7, 2, 7, 5, 1, 4, 5, 4, 6, 5, 1,
            8, 8, 1, 5, 3, 7, 5, 2, 8, 6, 5, 0, 6, 1, 8, 2, 2, 1, 9, 1, 5, 3, 0, 1, 2, 9, 4, 2, 0,
            8, 6, 6, 1, 9, 8, 9, 5, 2, 5, 5, 3, 1, 0, 1, 9, 7, 9, 4, 6, 3, 0, 2, 6, 2, 7, 8, 7, 8,
            8, 7, 3, 5, 7, 2, 6, 6, 5, 2, 2, 9, 7, 8, 5, 7, 8, 8, 3, 2, 7, 8, 5, 2, 4, 5, 6, 5, 7,
            5, 1, 9, 9, 9, 4, 5, 8, 9, 0, 2, 5, 5, 0, 2, 0, 3, 6, 6, 6, 3, 5, 8, 0, 4, 3, 3, 5, 5,
            8, 1, 6, 1, 6, 2, 7, 8, 8, 1, 3, 5, 4, 8, 8, 9, 1, 5, 7, 2, 2, 9, 8, 9, 5, 6, 0, 1, 6,
            3, 4, 5, 6, 0, 5, 7, 5, 5, 1, 2, 6, 8, 3, 0, 6, 3, 1, 8, 0, 8, 5, 0, 2, 0, 9, 4, 8, 5,
            4, 4, 4, 7, 4, 1, 0, 8, 3, 4, 0, 9, 6, 9, 8, 7, 4, 9, 4, 3, 6, 5, 9, 7, 8, 8, 0, 7, 6,
            3, 3, 3, 9, 3, 4, 4, 1, 9, 7, 2, 9, 8, 3, 1, 8, 9, 6, 0, 8, 1, 4, 3, 1, 8, 8, 6, 6, 2,
            1, 9, 9, 6, 6, 1, 0, 1, 4, 3, 7, 8, 5, 6, 2, 4, 1, 6, 6, 7, 8, 9, 7, 7, 2, 0, 1, 3, 7,
            0, 7, 1, 7, 7, 9, 4, 0, 1, 5, 0, 2, 3, 0, 0, 4, 2, 5, 6, 3, 0, 4, 1, 9, 1, 5, 6, 2, 4,
            5, 2, 5, 9, 0, 0, 8, 2, 6, 0, 9, 7, 7, 3, 0, 7, 9, 0, 5, 6, 2, 5, 4, 3, 3, 5, 2, 6, 9,
            0, 0, 9, 1, 6, 5, 3, 0, 4, 1, 8, 3, 9, 7, 7, 1, 1, 2, 5, 1, 1, 9, 1, 6, 2, 1, 5, 4, 6,
            2, 5, 4, 5, 9, 6, 5, 4, 8, 6, 1, 9, 2, 2, 9, 8, 9, 1, 8, 6, 7, 8, 4, 4, 1, 4, 4, 5, 5,
            4, 5, 3, 1, 3, 2, 0, 1, 1, 4, 9, 8,
        ],
        100,
    );

    println!(
        "Part 1: {:?}",
        signal.into_iter().take(8).collect::<Vec<_>>()
    );
}
pub fn solve_part_2() {
    let signal = phases_part_2(
        vec![
            5, 9, 7, 1, 7, 2, 3, 8, 1, 6, 8, 5, 8, 0, 0, 1, 0, 5, 9, 9, 0, 1, 2, 5, 2, 7, 5, 1, 0,
            9, 4, 3, 1, 4, 9, 3, 4, 7, 9, 3, 0, 7, 4, 2, 8, 2, 2, 8, 9, 9, 6, 3, 8, 2, 4, 7, 0, 8,
            3, 0, 0, 5, 8, 5, 5, 4, 8, 3, 8, 6, 7, 4, 8, 4, 3, 5, 6, 0, 5, 5, 4, 8, 9, 4, 1, 9, 9,
            1, 3, 5, 1, 2, 7, 2, 1, 0, 9, 5, 5, 6, 1, 6, 5, 5, 2, 6, 5, 1, 0, 7, 7, 4, 5, 9, 7, 2,
            7, 3, 9, 4, 6, 4, 2, 6, 8, 8, 4, 6, 3, 7, 4, 7, 2, 8, 3, 9, 3, 5, 0, 7, 5, 0, 9, 8, 4,
            0, 8, 5, 4, 1, 0, 9, 8, 0, 3, 7, 1, 8, 8, 0, 2, 7, 8, 0, 5, 4, 3, 2, 9, 8, 1, 4, 1, 3,
            9, 8, 6, 4, 4, 9, 5, 5, 5, 0, 6, 1, 4, 9, 9, 1, 4, 7, 9, 6, 7, 7, 5, 8, 8, 5, 2, 4, 6,
            6, 0, 2, 1, 2, 3, 7, 4, 6, 8, 6, 6, 2, 2, 3, 5, 2, 8, 3, 5, 6, 4, 9, 3, 0, 1, 2, 1, 3,
            6, 1, 5, 2, 9, 7, 4, 2, 1, 8, 7, 2, 0, 5, 4, 2, 2, 9, 7, 2, 7, 5, 1, 4, 5, 4, 6, 5, 1,
            8, 8, 1, 5, 3, 7, 5, 2, 8, 6, 5, 0, 6, 1, 8, 2, 2, 1, 9, 1, 5, 3, 0, 1, 2, 9, 4, 2, 0,
            8, 6, 6, 1, 9, 8, 9, 5, 2, 5, 5, 3, 1, 0, 1, 9, 7, 9, 4, 6, 3, 0, 2, 6, 2, 7, 8, 7, 8,
            8, 7, 3, 5, 7, 2, 6, 6, 5, 2, 2, 9, 7, 8, 5, 7, 8, 8, 3, 2, 7, 8, 5, 2, 4, 5, 6, 5, 7,
            5, 1, 9, 9, 9, 4, 5, 8, 9, 0, 2, 5, 5, 0, 2, 0, 3, 6, 6, 6, 3, 5, 8, 0, 4, 3, 3, 5, 5,
            8, 1, 6, 1, 6, 2, 7, 8, 8, 1, 3, 5, 4, 8, 8, 9, 1, 5, 7, 2, 2, 9, 8, 9, 5, 6, 0, 1, 6,
            3, 4, 5, 6, 0, 5, 7, 5, 5, 1, 2, 6, 8, 3, 0, 6, 3, 1, 8, 0, 8, 5, 0, 2, 0, 9, 4, 8, 5,
            4, 4, 4, 7, 4, 1, 0, 8, 3, 4, 0, 9, 6, 9, 8, 7, 4, 9, 4, 3, 6, 5, 9, 7, 8, 8, 0, 7, 6,
            3, 3, 3, 9, 3, 4, 4, 1, 9, 7, 2, 9, 8, 3, 1, 8, 9, 6, 0, 8, 1, 4, 3, 1, 8, 8, 6, 6, 2,
            1, 9, 9, 6, 6, 1, 0, 1, 4, 3, 7, 8, 5, 6, 2, 4, 1, 6, 6, 7, 8, 9, 7, 7, 2, 0, 1, 3, 7,
            0, 7, 1, 7, 7, 9, 4, 0, 1, 5, 0, 2, 3, 0, 0, 4, 2, 5, 6, 3, 0, 4, 1, 9, 1, 5, 6, 2, 4,
            5, 2, 5, 9, 0, 0, 8, 2, 6, 0, 9, 7, 7, 3, 0, 7, 9, 0, 5, 6, 2, 5, 4, 3, 3, 5, 2, 6, 9,
            0, 0, 9, 1, 6, 5, 3, 0, 4, 1, 8, 3, 9, 7, 7, 1, 1, 2, 5, 1, 1, 9, 1, 6, 2, 1, 5, 4, 6,
            2, 5, 4, 5, 9, 6, 5, 4, 8, 6, 1, 9, 2, 2, 9, 8, 9, 1, 8, 6, 7, 8, 4, 4, 1, 4, 4, 5, 5,
            4, 5, 3, 1, 3, 2, 0, 1, 1, 4, 9, 8,
        ],
        100,
    );
    println!(
        "Part 2: {:?}",
        signal.into_iter().take(8).collect::<Vec<_>>()
    );
}
fn phases(mut signal: Vec<i32>, phase: u32) -> Vec<i32> {
    for i in 0..phase {
        process(&mut signal, 0);
        println!("Phase {}", i);
    }
    signal
}

fn phases_part_2(mut signal: Vec<i32>, phase: u32) -> Vec<i32> {
    let offset = signal.iter().take(7).fold(0, |acc, next| acc * 10 + next) as usize;
    let signal_length = signal.len();
    println!("Offset: {}", offset);
    let mut signal = signal
        .into_iter()
        .cycle()
        .skip(offset)
        .take(signal_length * 10_000 - offset)
        .collect();
    for i in 0..phase {
        process_part_2(&mut signal);
        println!("Phase {}", i);
    }
    signal
}

fn process_part_2(signal: &mut Vec<i32>) {
    for i in (0..signal.len() - 1).rev() {
        signal[i] = (signal[i + 1] + signal[i]).abs() % 10;
    }
}

fn process(signal: &mut Vec<i32>, offset: usize) {
    for i in offset..signal.len() {
        let mut value = 0;
        let mut base_pattern = BasePattern::new((i + 1) as u32).skip(i);
        for s in signal.iter().skip(i) {
            let p = base_pattern.next().unwrap();
            let v = s * p;
            value += v
        }
        signal[i] = value.abs() % 10;
        if i % 100 == 0 {
            println!("Row {}", i);
        }
    }
}

fn repeat(times: usize, signal: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::with_capacity(times * signal.len());
    for _ in 0..times {
        result.append(&mut signal.clone());
    }
    result
}

struct BasePattern {
    position: u32,
    current: usize,
    step: u32,
}
const BASE_PATTERN: [i32; 4] = [0, 1, 0, -1];

impl BasePattern {
    fn new(position: u32) -> BasePattern {
        BasePattern {
            position: position,
            current: 0,
            step: 1,
        }
    }

    fn from(position: u32) -> BasePattern {
        BasePattern {
            position: position,
            current: 1,
            step: 0,
        }
    }
}

impl Iterator for BasePattern {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        if (self.step as u32) < self.position {
            self.step += 1;
            Some(BASE_PATTERN[self.current])
        } else {
            self.step = 0;
            self.current = (self.current + 1) % 4;
            self.next()
        }
    }

    fn nth(&mut self, mut n: usize) -> Option<Self::Item> {
        if n == self.position as usize - 1 {
            self.current = 1;
            self.step = 0;
            self.next()
        } else {
            Iterator::nth(self, n)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::prob16::{repeat, BasePattern};

    #[test]
    fn test_base_pattern() {
        assert_eq!(
            super::BasePattern::new(1).take(8).collect::<Vec<_>>(),
            vec!(1, 0, -1, 0, 1, 0, -1, 0)
        );
        assert_eq!(
            super::BasePattern::new(2).take(8).collect::<Vec<_>>(),
            vec!(0, 1, 1, 0, 0, -1, -1, 0)
        );

        assert_eq!(
            super::BasePattern::new(2)
                .skip(1)
                .take(8)
                .collect::<Vec<_>>(),
            vec!(1, 1, 0, 0, -1, -1, 0, 0)
        );

        assert_eq!(
            super::BasePattern::new(5)
                .skip(4)
                .take(15)
                .collect::<Vec<_>>(),
            vec!(1, 1, 1, 1, 1, 0, 0, 0, 0, 0, -1, -1, -1, -1, -1)
        );
    }

    #[test]
    fn test_basic_sample() {
        assert_eq!(
            super::phases(vec!(1, 2, 3, 4, 5, 6, 7, 8), 4),
            vec!(0, 1, 0, 2, 9, 4, 9, 8)
        );
    }

    #[test]
    fn test_sample_2() {
        assert_eq!(
            super::phases(
                vec!(
                    8, 0, 8, 7, 1, 2, 2, 4, 5, 8, 5, 9, 1, 4, 5, 4, 6, 6, 1, 9, 0, 8, 3, 2, 1, 8,
                    6, 4, 5, 5, 9, 5
                ),
                100
            )
            .into_iter()
            .take(8)
            .collect::<Vec<_>>(),
            vec!(2, 4, 1, 7, 6, 1, 7, 6)
        );
    }

    #[test]
    fn test_sample_3() {
        assert_eq!(
            super::phases(
                vec!(
                    1, 9, 6, 1, 7, 8, 0, 4, 2, 0, 7, 2, 0, 2, 2, 0, 9, 1, 4, 4, 9, 1, 6, 0, 4, 4,
                    1, 8, 9, 9, 1, 7
                ),
                100
            )
            .into_iter()
            .take(8)
            .collect::<Vec<_>>(),
            vec!(7, 3, 7, 4, 5, 4, 1, 8)
        );
    }

    #[test]
    fn test_sample_4() {
        assert_eq!(
            super::phases(
                vec!(
                    6, 9, 3, 1, 7, 1, 6, 3, 4, 9, 2, 9, 4, 8, 6, 0, 6, 3, 3, 5, 9, 9, 5, 9, 2, 4,
                    3, 1, 9, 8, 7, 3,
                ),
                100
            )
            .into_iter()
            .take(8)
            .collect::<Vec<_>>(),
            vec!(5, 2, 4, 3, 2, 1, 3, 3)
        );
    }

    #[ignore]
    #[test]
    fn test_part_2() {
        let mut base_pattern = BasePattern::new(0303673 + 1);
        let mut i = 0;
        let mut count = 0303673;
        let signal = repeat(
            10_000,
            vec![
                0, 3, 0, 3, 6, 7, 3, 2, 5, 7, 7, 2, 1, 2, 9, 4, 4, 0, 6, 3, 4, 9, 1, 5, 6, 5, 4, 7,
                4, 6, 6, 4,
            ],
        );
        let mut value = 0;
        for s in signal.iter().skip(0303673) {
            let p = base_pattern.next().unwrap();
            value += s * p;
            count += 1;
            i = (count % signal.len());
        }
        println!("")
    }

    #[test]
    fn test_sample_1_part_2() {
        assert_eq!(
            super::phases_part_2(
                vec!(
                    0, 3, 0, 3, 6, 7, 3, 2, 5, 7, 7, 2, 1, 2, 9, 4, 4, 0, 6, 3, 4, 9, 1, 5, 6, 5,
                    4, 7, 4, 6, 6, 4
                ),
                100
            )
            .into_iter()
            // .skip(0303673)
            .take(8)
            .collect::<Vec<_>>(),
            vec!(8, 4, 4, 6, 2, 0, 2, 6)
        );
    }

    #[test]
    fn test_sample_2_part_2() {
        assert_eq!(
            super::phases_part_2(
                vec!(
                    0, 2, 9, 3, 5, 1, 0, 9, 6, 9, 9, 9, 4, 0, 8, 0, 7, 4, 0, 7, 5, 8, 5, 4, 4, 7,
                    0, 3, 4, 3, 2, 3
                ),
                100
            )
            .into_iter()
            // .skip(0303673)
            .take(8)
            .collect::<Vec<_>>(),
            vec!(7, 8, 7, 2, 5, 2, 7, 0)
        );
    }
}
