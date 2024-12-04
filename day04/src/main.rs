#[allow(unused)]
const TEST_INPUT: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

type WordSearch = Vec<Vec<char>>;

fn parse_input(input: &str) -> WordSearch {
    input.lines().map(|line| line.chars().collect()).collect()
}

fn part_1(input: &str) -> usize {
    let word_search = parse_input(input);

    todo!("Return number of times the word 'XMAS' appears in WordSearch")
}

fn main() {
    let input = TEST_INPUT;

    let part_1_res = part_1(input);
    println!("Part 1: {part_1_res}");
}
