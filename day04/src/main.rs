#[allow(unused)]
const TEST_INPUT_1: &str = "MMMSXXMASM
                            MSAMXMSMSA
                            AMXSXMAAMM
                            MSAMASMSMX
                            XMASAMXAMM
                            XXAMMXXAMA
                            SMSMSASXSS
                            SAXAMASAAA
                            MAMMMXMMMM
                            MXMXAXMASX";
#[allow(unused)]
const TEST_INPUT_2: &str = ".M.S......
                            ..A..MSMS.
                            .M.S.MAA..
                            ..A.ASMSM.
                            .M.S.M....
                            ..........
                            S.S.S.S.S.
                            .A.A.A.A..
                            M.M.M.M.M.
                            ..........";

#[allow(unused)]
const TEST_INPUT_SIMPLER: &str = "1234h
                                  5678i
                                  9abcj
                                  defgk
                                  lmnop";

#[allow(unused)]
const INPUT_PATH: &str = "input.txt";

#[derive(PartialEq)]
enum Diagonal {
    Ascending,
    Descending,
}

enum SplitKind {
    Rows,
    Columns,
    Diagonal(Diagonal, usize),
}

fn get_diagonals(board: &[Vec<char>], diag: Diagonal, size: usize) -> Vec<Vec<char>> {
    let mut diagonals = vec![];
    for (i, row) in board.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            let mut curr_diagonal = vec![];
            let mut did_break = false;
            for next in 0..size {
                match diag {
                    Diagonal::Descending => {
                        if i + next < board.len() && j + next < board[0].len() {
                            let curr_point = board[i + next][j + next];
                            curr_diagonal.push(curr_point);
                        } else {
                            did_break = true;
                            break;
                        }
                    }
                    Diagonal::Ascending => {
                        if i + next < board.len() && j as i32 - next as i32 >= 0 {
                            let curr_point = board[i + next][j - next];
                            curr_diagonal.push(curr_point);
                        } else {
                            did_break = true;
                            break;
                        }
                    }
                };
            }

            if let Some(diagonal) = if did_break { None } else { Some(curr_diagonal) } {
                diagonals.push(diagonal);
            }
        }
    }

    diagonals
}

fn split_into(board: &[Vec<char>], split_kind: SplitKind) -> Vec<Vec<char>> {
    match split_kind {
        SplitKind::Rows => board.to_vec(),
        SplitKind::Columns => {
            let mut columns = vec![vec![]; board[0].len()];
            for row in board.iter() {
                for (j, &cell) in row.iter().enumerate() {
                    columns[j].push(cell);
                }
            }
            columns
        }
        SplitKind::Diagonal(diagonal, size) => get_diagonals(board, diagonal, size),
    }
}

fn get_blocks(board: &[Vec<char>], width: usize, height: usize) -> Vec<Vec<Vec<char>>> {
    let mut blocks = vec![];
    for (i, row) in board.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            let mut curr_block = vec![vec![]; width];
            let mut did_break = false;
            for y in 0..height {
                for x in 0..width {
                    if i + y < board.len() && j + x < board[0].len() {
                        curr_block[y].push(board[i + y][j + x]);
                    } else {
                        did_break = true;
                        break;
                    }
                }
            }
            if let Some(block) = if did_break { None } else { Some(curr_block) } {
                blocks.push(block);
            }
        }
    }
    blocks
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

fn part_1(input: &str) -> usize {
    let board = parse_input(input);

    let rows = split_into(&board, SplitKind::Rows);
    let columns = split_into(&board, SplitKind::Columns);
    let diagonals_ascending = split_into(&board, SplitKind::Diagonal(Diagonal::Ascending, 4));
    let diagonals_descending = split_into(&board, SplitKind::Diagonal(Diagonal::Descending, 4));

    let mut cnt = 0;

    for row in rows.iter() {
        for window in row.windows(4) {
            match window {
                ['X', 'M', 'A', 'S'] | ['S', 'A', 'M', 'X'] => cnt += 1,
                _ => continue,
            };
        }
    }

    for col in columns.iter() {
        for window in col.windows(4) {
            match window {
                ['X', 'M', 'A', 'S'] | ['S', 'A', 'M', 'X'] => cnt += 1,
                _ => continue,
            };
        }
    }

    for diag_asc in diagonals_ascending.iter() {
        let sl = diag_asc.as_slice();
        match sl {
            ['X', 'M', 'A', 'S'] | ['S', 'A', 'M', 'X'] => cnt += 1,
            _ => continue,
        };
    }

    for diag_dsc in diagonals_descending.iter() {
        let sl = diag_dsc.as_slice();
        match sl {
            ['X', 'M', 'A', 'S'] | ['S', 'A', 'M', 'X'] => cnt += 1,
            _ => continue,
        };
    }

    cnt
}

fn part_2(input: &str) -> usize {
    const LETTER_SEQUENCES: [[char; 4]; 4] = [
        ['M', 'M', 'S', 'S'],
        ['S', 'M', 'M', 'S'],
        ['S', 'S', 'M', 'M'],
        ['M', 'S', 'S', 'M'],
    ];

    let board = parse_input(input);
    let blocks = get_blocks(&board, 3, 3);
    let mut cnt = 0;

    for block in blocks.into_iter() {
        for seq in LETTER_SEQUENCES {
            // clockwise winding order
            if block[1][1] == 'A'
                && block[0][0] == seq[0]
                && block[0][2] == seq[1]
                && block[2][2] == seq[2]
                && block[2][0] == seq[3]
            {
                cnt += 1;
            }
        }
    }

    cnt
}

fn main() {
    let input = &std::fs::read_to_string(INPUT_PATH).unwrap();

    let part_1_res = part_1(input);
    println!("Part 1: {part_1_res}");

    let part_2_res = part_2(input);
    println!("Part 2: {part_2_res}")
}
