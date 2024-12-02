#[allow(unused)]
const TEST_INPUT: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

#[allow(unused)]
const INPUT_PATH: &str = "input.txt";

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.trim()
                .split_whitespace()
                .map(|num| num.trim().parse().unwrap())
                .collect()
        })
        .collect()
}

fn is_good(seq: &[i32]) -> bool {
    let diff = seq[1].abs_diff(seq[0]);
    if diff < 1 || diff > 3 {
        return false;
    }

    let is_increasing = (seq[1] - seq[0]) > 0;
    for tuple in seq.windows(2) {
        let (left, right) = (tuple[0], tuple[1]);
        let diff = right.abs_diff(left);
        if diff < 1 || diff > 3 || ((right - left) > 0) != is_increasing {
            return false;
        }
    }
    
    true
}

fn part_1(input: &str) -> usize {
    let lists = parse_input(input);

    lists
        .iter()
        .filter(|&list| is_good(list))
        .count()
}

fn part_2(input: &str) -> usize {
    let lists = parse_input(input);

    let mut cnt = 0;

    for list in lists.iter() {
        if is_good(list) {
            cnt += 1;
            continue;
        }

        for i in 0..list.len() {
            let mut new_list = list.clone();
            new_list.remove(i);
            if is_good(&new_list) {
                cnt += 1;
                break;
            }
        }
    }

    cnt
}

fn main() {
    let input = std::fs::read_to_string(INPUT_PATH).unwrap_or_else(|err| {
        panic!(
            "File {INPUT_PATH} could not be oppened because of an error: {:?}",
            err
        )
    });

    let part_1_res = part_1(&input);
    println!("Part 1: {part_1_res}");

    let part_2_res = part_2(&input);
    println!("Part 2: {part_2_res}");
}
