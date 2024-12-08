use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Sub},
};

use itertools::Itertools;

#[allow(unused)]
const TEST_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

#[allow(unused)]
const INPUT_PATH: &str = "input.txt";

type Board = Vec<Vec<char>>;
type Point = (usize, usize);

fn parse_input(input: &str) -> Board {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Vec2 {
    i: i32,
    j: i32,
}

impl Vec2 {
    fn from_point(p: Point) -> Self {
        Self {
            i: p.0 as i32,
            j: p.1 as i32,
        }
    }

    fn is_in_bounds(self, board: &Board) -> bool {
        self.i >= 0 && self.i < board.len() as i32 && self.j >= 0 && self.j < board[0].len() as i32
    }

    fn as_tuple(self) -> (usize, usize) {
        (self.i as usize, self.j as usize)
    }

    fn normalize(self) -> Self {
        let div = gcd(self.i.abs(), self.j.abs());
        Self {
            i: self.i / div,
            j: self.j / div,
        }
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            i: rhs.i + self.i,
            j: rhs.j + self.j,
        }
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            i: self.i - rhs.i,
            j: self.j - rhs.j,
        }
    }
}

fn gcd(a: i32, b: i32) -> i32 {
    if a == 0 {
        return b;
    }

    gcd(b % a, a)
}

fn get_antinodes_1(
    board: &Board,
    (i1, j1): Point,
    (i2, j2): Point,
) -> (Option<Point>, Option<Point>) {
    let ((new_i1, new_j1), (new_i2, new_j2)) = (
        (2 * i1 as i32 - i2 as i32, 2 * j1 as i32 - j2 as i32),
        (2 * i2 as i32 - i1 as i32, 2 * j2 as i32 - j1 as i32),
    );

    let an1 = if new_i1 < 0
        || new_i1 >= board.len() as i32
        || new_j1 < 0
        || new_j1 >= board[0].len() as i32
    {
        None
    } else {
        Some((new_i1 as usize, new_j1 as usize))
    };

    let an2 = if new_i2 < 0
        || new_i2 >= board.len() as i32
        || new_j2 < 0
        || new_j2 >= board[0].len() as i32
    {
        None
    } else {
        Some((new_i2 as usize, new_j2 as usize))
    };

    (an1, an2)
}

fn get_antinodes_2(board: &Board, p1: Point, p2: Point) -> Vec<Point> {
    let (p1, p2) = (Vec2::from_point(p1), Vec2::from_point(p2));
    let step = (p2 - p1).normalize();
    let mut antinodes = HashSet::from([p1, p2]);

    let mut point = p1;
    while point.is_in_bounds(board) {
        antinodes.insert(point);
        point = point - step;
    }

    let mut point = p1;
    while point.is_in_bounds(board) {
        antinodes.insert(point);
        point = point + step;
    }

    antinodes
        .iter()
        .map(|antinode| antinode.as_tuple())
        .collect()
}

fn get_board_and_antennas(input: &str) -> (Board, HashMap<char, Vec<Point>>) {
    let board = parse_input(input);
    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();

    for (i, row) in board.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == '.' {
                continue;
            }
            if let Some(antenna) = antennas.get_mut(&cell) {
                antenna.push((i, j));
            } else {
                antennas.insert(cell, vec![(i, j)]);
            }
        }
    }
    (board, antennas)
}

fn part_1(input: &str) -> usize {
    let (board, antennas) = get_board_and_antennas(input);
    let mut antinodes = vec![vec![false; board[0].len()]; board.len()];

    for (_, coords) in antennas.iter() {
        for combo in coords.iter().combinations(2) {
            let (an1, an2) = get_antinodes_1(&board, *combo[0], *combo[1]);
            if let Some(an1) = an1 {
                antinodes[an1.0][an1.1] = true;
            }
            if let Some(an2) = an2 {
                antinodes[an2.0][an2.1] = true;
            }
        }
    }

    antinodes.into_iter().flatten().filter(|&elem| elem).count()
}

fn part_2(input: &str) -> usize {
    let (board, antennas) = get_board_and_antennas(input);
    let mut antinodes = vec![vec![false; board[0].len()]; board.len()];

    for (_, coords) in antennas.iter() {
        for combo in coords.iter().combinations(2) {
            let anodes = get_antinodes_2(&board, *combo[0], *combo[1]);
            for &(i, j) in anodes.iter() {
                antinodes[i][j] = true;
            }
        }
    }

    antinodes.into_iter().flatten().filter(|&elem| elem).count()
}

fn main() {
    let input = &std::fs::read_to_string(INPUT_PATH).unwrap();

    let part_1_res = part_1(input);
    println!("Part 1: {part_1_res}");

    let part_2_res = part_2(input);
    println!("Part 2: {part_2_res}");
}
