const TEST_INPUT: &str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;
const INPUT_PATH: &str = "input.txt";

fn parse_input(input: &str) -> (Vec<u32>, Vec<u32>) {
    input
        .split('\n')
        .fold((vec![], vec![]), |(mut map1, mut map2), row| {
            let (cell1, cell2) = row.trim().split_once("   ").unwrap();
            map1.push(cell1.trim().parse().unwrap());
            map2.push(cell2.trim().parse().unwrap());
            (map1, map2)
        })
}

fn part_1(input: &str) -> u32 {
    let (mut map1, mut map2) = parse_input(input);
    map1.sort();
    map2.sort();

    map1.iter()
        .zip(&map2)
        .map(|(&e1, &e2)| e1.abs_diff(e2))
        .sum()
}

fn part_2(input: &str) -> u32 {
    let (map1, mut map2) = parse_input(input);
    map2.sort();

    map1.iter()
        .map(|&elem| {
            let mut cnt = 0;
            let mut i = 0;

            while i < map2.len() && map2[i] < elem {
                i += 1;
            }

            while i < map2.len() && map2[i] == elem {
                cnt += 1;
                i += 1;
            }

            elem * cnt
        })
        .sum()
}

fn main() {
    let input = &std::fs::read_to_string(INPUT_PATH).unwrap_or_else(|err| {
        panic!("File {INPUT_PATH} could not be oppened because of an error: {err}")
    });

    let part_1_res = part_1(input);
    println!("Part 1: {part_1_res}");

    let part_2_res = part_2(input);
    println!("Part 2: {part_2_res}");
}
