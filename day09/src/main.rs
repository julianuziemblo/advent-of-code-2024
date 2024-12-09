#[allow(unused)]
const TEST_INPUT: &str = "2333133121414131402";
#[allow(unused)]
const INPUT_PATH: &str = "input.txt";

trait BlockUtil {
    fn fits(&self, other: &Self) -> bool;
    fn is_file(&self) -> bool;
}

impl<T: Clone> BlockUtil for &[Option<T>] {
    fn fits(&self, other: &Self) -> bool {
        self.len() >= other.len()
    }

    fn is_file(&self) -> bool {
        self.iter().all(|elem| elem.is_some())
    }
}

fn swap_blocks_least(disk: &mut [Option<usize>], mut b1_start: usize, mut b2_start: usize) {
    let b1_val = disk[b1_start];
    let b2_val = disk[b2_start];
    while b1_start < disk.len()
        && disk[b1_start] == b1_val
        && b2_start < disk.len()
        && disk[b2_start] == b2_val
    {
        disk.swap(b1_start, b2_start);
        b1_start += 1;
        b2_start += 1;
    }
}

fn parse_input_1(input: &str) -> Vec<Option<usize>> {
    input.chars().enumerate().fold(vec![], |mut list, (i, ch)| {
        let len = ch.to_digit(10).unwrap() as usize;
        list.extend(if i % 2 == 0 {
            vec![Some(i / 2); len]
        } else {
            vec![None; len]
        });
        list
    })
}

fn next_block<T: PartialEq>(disk: &[T], start: usize, step: i32) -> Option<(&[T], usize)> {
    let mut p = start;
    while p < disk.len() && disk[p] == disk[start] {
        p = (p as i32 + step) as usize;
    }

    let res;
    if start <= p {
        res = (&disk[start..p], p)
    } else {
        res = (&disk[p + 1..=start], p)
    }

    if res.0.is_empty() {
        None
    } else {
        Some(res)
    }
}

fn next_free<T: Clone + PartialEq>(disk: &[Option<T>], start: usize) -> Option<(&[Option<T>], usize)> {
    let mut p = start;
    while let Some((block, p1)) = next_block(&disk, p, -1) {
        if block.is_file() {
            return Some((block, p));
        }
        p = p1;
    }
    None
}

fn checksum(disk: &[Option<usize>]) -> usize {
    disk.iter().enumerate().fold(0, |sum, (i, &space)| {
        if let Some(file_id) = space {
            sum + (file_id * i)
        } else {
            sum
        }
    })
}

fn part_1(input: &str) -> usize {
    let mut disk = parse_input_1(input);
    let mut p = 0;
    let mut q = disk.len() - 1;

    while p < q {
        while p < disk.len() && disk[p].is_some() {
            p += 1;
        }
        while disk[q].is_none() {
            q -= 1;
        }

        if q >= p {
            disk.swap(p, q);
        }
    }

    checksum(&disk)
}

fn part_2(input: &str) -> usize {
    let mut disk = parse_input_1(input);
    let mut q = disk.len() - 1;

    while let Some((block, q1)) = next_block(&disk, q, -1) {
        let mut p = 0;
        while let Some((free, p1)) = next_free(&disk, p) {
            
            if p1 > q1 {
                break;
            }

            if free.fits(&block) {
                swap_blocks_least(&mut disk, p, q);
                break;
            }

            p = p1;
        }
        q = q1;
    }

    println!("disk={disk:?}");
    checksum(&disk)
}

fn main() {
    let input = &std::fs::read_to_string(INPUT_PATH).unwrap();

    let part_1_res = part_1(input);
    println!("Part 1: {part_1_res}");

    let part_2_res = part_2(TEST_INPUT);
    println!("Part 2: {part_2_res}");
}
