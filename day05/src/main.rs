use std::collections::{HashMap, HashSet};

#[allow(unused)]
const TEST_INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

#[allow(unused)]
const INPUT_PATH: &str = "input.txt";

type Page = u8;
type Rules = HashMap<Page, HashSet<Page>>;
type Update = Vec<Page>;

fn parse_input(input: &str) -> (Rules, Vec<Update>) {
    #[cfg(windows)]
    const DOUBLE_LINE_ENDING: &str = "\r\n\r\n";
    #[cfg(not(windows))]
    const DOUBLE_LINE_ENDING: &'static str = "\n\n";
    let (rules_str, updates_str) = input.split_once(DOUBLE_LINE_ENDING).unwrap();

    let mut rules = Rules::new();
    let updates: Vec<Update> = updates_str
        .lines()
        .map(|line| {
            line.split(',')
                .map(|value| value.parse().unwrap())
                .collect()
        })
        .collect();

    for (k, v) in rules_str.lines().map(|rule| {
        let (pre, post) = rule.trim().split_once('|').unwrap();
        (pre.trim().parse().unwrap(), post.trim().parse().unwrap())
    }) {
        if let std::collections::hash_map::Entry::Vacant(e) = rules.entry(k) {
            let mut rule_set = HashSet::new();
            rule_set.insert(v);
            e.insert(rule_set);
        } else {
            rules.get_mut(&k).unwrap().insert(v);
        }
    }

    (rules, updates)
}

fn is_correct(update: &[Page], rules: &Rules) -> bool {
    for i in 0..update.len() {
        if let Some(rule_set) = rules.get(&update[i]) {
            for elem in &update[..i] {
                if rule_set.contains(elem) {
                    return false;
                }
            }
        }
    }
    true
}

fn fix(update: &mut [Page], rules: &Rules) {
    for i in 0..update.len() {
        if let Some(rule_set) = rules.get(&update[i]) {
            for j in 0..i {
                if rule_set.contains(&update[j]) {
                    update.swap(i, j);
                }
            }
        }
    }
}

fn part_1(input: &str) -> u32 {
    let (rules, updates) = parse_input(input);
    let mut middle_sum = 0;

    for update in updates.iter() {
        if is_correct(update, &rules) {
            middle_sum += update[update.len() / 2] as u32;
        }
    }

    middle_sum
}

fn part_2(input: &str) -> u32 {
    let (rules, updates) = parse_input(input);
    let mut middle_sum = 0;

    for update in updates.iter() {
        if !is_correct(update, &rules) {
            let mut fixed_update = update.clone();
            fix(&mut fixed_update, &rules);
            middle_sum += fixed_update[fixed_update.len() / 2] as u32;
        }
    }

    middle_sum
}

fn main() {
    let input = &std::fs::read_to_string(INPUT_PATH).unwrap();

    let part_1_res = part_1(input);
    println!("Part 1: {part_1_res}");

    let part_2_res = part_2(input);
    println!("Part 2: {part_2_res}");
}
