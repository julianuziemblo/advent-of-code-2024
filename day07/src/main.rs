use indicatif::ProgressIterator;
use std::ops::{Add, Mul};

#[allow(unused)]
const TEST_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

#[allow(unused)]
const INPUT_PATH: &str = "input.txt";

trait ConcatBase10: Copy {
    fn concat_base_10(self, other: Self) -> Self;
}

impl ConcatBase10 for usize {
    fn concat_base_10(self, other: Self) -> Self {
        format!("{self}{other}").parse().unwrap()
    }
}

fn parse_input(input: &str) -> Vec<(usize, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let (result, operands) = line.split_once(":").unwrap();
            (
                result.parse().unwrap(),
                operands
                    .split_whitespace()
                    .map(|op| op.parse().unwrap())
                    .collect(),
            )
        })
        .collect()
}

// for N arbitrary ops!
fn solve_for_ops<const N: usize>(
    input: &[(usize, Vec<usize>)],
    ops: [fn(usize, usize) -> usize; N],
) -> usize {
    input
        .iter()
        .progress()
        .filter(|(result, operands)| {
            for possibility in 0..N.pow(operands.len() as u32 - 1) {
                let curr_res = operands.iter().skip(1).enumerate().fold(
                    operands[0],
                    |accum, (i, &operand)| {
                        let opcode = (possibility / N.pow(i as u32)) % N;
                        ops[opcode](accum, operand)
                    },
                );

                if curr_res == *result {
                    return true;
                }
            }

            false
        })
        .map(|(elem, _)| *elem)
        .sum()
}

fn part_1(input: &str) -> usize {
    solve_for_ops(&parse_input(input), [Add::add, Mul::mul])
}

fn part_2(input: &str) -> usize {
    solve_for_ops(
        &parse_input(input),
        [Add::add, Mul::mul, ConcatBase10::concat_base_10],
    )
}

fn main() {
    let input = &std::fs::read_to_string(INPUT_PATH).unwrap();

    let part_1_res = part_1(input);
    println!("Part 1: {part_1_res}");

    let part_2_res = part_2(input);
    println!("Part 2: {part_2_res}");
}
