use std::collections::HashSet;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|s| s.parse().unwrap())
        .collect()
}

#[aoc(day1, part1)]
fn part1(input: &[i64]) -> i64 {
    const TARGET_SUM: i64 = 2020;
    let mut previous_numbers = HashSet::new();
    for num in input.iter() {
        let pair_num = TARGET_SUM - num;
        if previous_numbers.contains(&pair_num) {
            return num * pair_num;
        }
        previous_numbers.insert(num);
    }
    panic!("no pair");
}
