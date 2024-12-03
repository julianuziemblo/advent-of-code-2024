extern crate regex;

#[allow(unused)]
const TEST_INPUT_1: &str =
    "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
#[allow(unused)]
const TEST_INPUT_2: &str =
    "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

#[allow(unused)]
const INPUT_PATH: &str = "input.txt";

use regex::Regex;

fn parse_input<'a, const N: usize>(input: &'a str, re: &'a Regex) -> impl Iterator<Item = &'a str> {
    re.captures_iter(input)
        .map(|c| c.extract::<N>())
        .map(|(matched, _)| matched)
}

fn execute_mul(mul: &str) -> u32 {
    mul[3..]
        .strip_prefix('(')
        .unwrap()
        .strip_suffix(')')
        .unwrap()
        .split(',')
        .map(|ch| ch.parse::<u32>().unwrap())
        .fold(1, |accum, n| accum * n)
}

fn part_1(input: &str) -> u32 {
    let re = Regex::new(r"mul\([0-9]{0,4},[0-9]{0,4}\)").unwrap();

    parse_input::<0>(input, &re).map(execute_mul).sum()
}

fn part_2(input: &str) -> u32 {
    let re = Regex::new(r"(mul\([0-9]{0,4},[0-9]{0,4}\))|(do\(\))|(don't\(\))").unwrap();

    parse_input::<1>(input, &re)
        .fold((true, 0), |(can_multiply, sum), op| match &op[0..3] {
            "do(" => (true, sum),
            "don" => (false, sum),
            "mul" => (
                can_multiply,
                sum + if can_multiply { execute_mul(op) } else { 0 },
            ),
            _ => panic!("Unreachable, op={}", op),
        })
        .1
}

fn main() {
    let input = &std::fs::read_to_string(INPUT_PATH).unwrap();

    let part_1_res = part_1(input);
    println!("Part 1: {part_1_res}");

    let part_2_res = part_2(input);
    println!("Part 2: {part_2_res}");
}
