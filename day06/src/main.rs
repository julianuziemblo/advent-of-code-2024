use std::collections::HashSet;

use indicatif::ProgressIterator;

#[allow(unused)]
const TEST_INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

#[allow(unused)]
const INPUT_PATH: &str = "input.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn turn_right_90_deg(self) -> Self {
        match self {
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Guard(Direction),
}

impl Tile {
    fn from_char(ch: char) -> Self {
        match ch {
            '.' => Tile::Empty,
            '#' => Tile::Wall,
            'v' => Tile::Guard(Direction::Down),
            '>' => Tile::Guard(Direction::Right),
            '<' => Tile::Guard(Direction::Left),
            '^' => Tile::Guard(Direction::Up),
            _ => panic!("Unknown character: {}", ch),
        }
    }

    #[allow(unused)]
    fn as_char(self) -> char {
        match self {
            Tile::Empty => '.',
            Tile::Wall => '#',
            Tile::Guard(direction) => match direction {
                Direction::Down => 'v',
                Direction::Right => '>',
                Direction::Left => '<',
                Direction::Up => '^',
            },
        }
    }
}

type Board = Vec<Vec<Tile>>;

#[cfg(debug_assertions)]
#[allow(unused)]
trait PrintTiles {
    fn print(&self);
    fn print_with_visited(&self, visited: &HashSet<(usize, usize)>);
}

#[cfg(debug_assertions)]
#[allow(unused)]
impl PrintTiles for Board {
    fn print(&self) {
        for row in self.iter() {
            for &cell in row.iter() {
                print!("{}", cell.as_char());
            }
            println!()
        }
    }

    fn print_with_visited(&self, visited: &HashSet<(usize, usize)>) {
        for (i, row) in self.iter().enumerate() {
            for (j, &cell) in row.iter().enumerate() {
                if visited.contains(&(i, j)) {
                    print!("@");
                } else {
                    print!("{}", cell.as_char());
                }
            }
            println!()
        }
    }
}

enum NextMove {
    OutOfBound,
    Turn,
    GoTo(usize, usize),
}

fn parse_input(input: &str) -> Board {
    input
        .lines()
        .map(|line| line.chars().map(Tile::from_char).collect())
        .collect()
}

fn find_start(board: &Board) -> ((usize, usize), Direction) {
    for (i, row) in board.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if let Tile::Guard(dir) = cell {
                return ((i, j), *dir);
            }
        }
    }
    panic!("Couldn't find start!");
}

fn next_move(board: &Board, (i, j): (usize, usize), direction: Direction) -> NextMove {
    let (next_i, next_j) = match direction {
        Direction::Left => (i as i32, j as i32 - 1),
        Direction::Up => (i as i32 - 1, j as i32),
        Direction::Right => (i as i32, j as i32 + 1),
        Direction::Down => (i as i32 + 1, j as i32),
    };

    if next_i < 0 || next_i >= board.len() as i32 || next_j < 0 || next_j >= board[0].len() as i32 {
        NextMove::OutOfBound
    } else if board[next_i as usize][next_j as usize] == Tile::Wall {
        NextMove::Turn
    } else {
        NextMove::GoTo(next_i as usize, next_j as usize)
    }
}

fn part_1(input: &str) -> usize {
    let board = parse_input(input);
    let (mut curr_pos, mut curr_dir) = find_start(&board);
    let mut visited = HashSet::new();

    loop {
        visited.insert(curr_pos);
        match next_move(&board, curr_pos, curr_dir) {
            NextMove::OutOfBound => break,
            NextMove::Turn => curr_dir = curr_dir.turn_right_90_deg(),
            NextMove::GoTo(new_i, new_j) => curr_pos = (new_i, new_j),
        }
    }

    #[cfg(debug_assertions)]
    board.print_with_visited(&visited);
    #[cfg(debug_assertions)]
    println!();

    visited.len()
}

fn part_2(input: &str) -> usize {
    let board = parse_input(input);
    let (start_pos, start_dir) = find_start(&board);
    let mut cnt = 0;

    for (i, row) in board.iter().enumerate().progress() {
        for (j, &cell) in row.iter().enumerate() {
            if (i, j) == start_pos || cell == Tile::Wall {
                continue;
            }

            let (mut curr_pos, mut curr_dir) = (start_pos, start_dir);
            let mut new_board = board.clone();
            let mut moved = false;
            new_board[i][j] = Tile::Wall;

            let mut visited = HashSet::new();

            loop {
                match next_move(&new_board, curr_pos, curr_dir) {
                    NextMove::OutOfBound => break,
                    NextMove::Turn => curr_dir = curr_dir.turn_right_90_deg(),
                    NextMove::GoTo(new_i, new_j) => {
                        curr_pos = (new_i, new_j);
                        moved = true;
                    }
                };

                if moved && visited.contains(&(curr_pos, curr_dir)) {
                    cnt += 1;
                    break;
                }

                visited.insert((curr_pos, curr_dir));
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
    println!("Part 2: {part_2_res}");
}
