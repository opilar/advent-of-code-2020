use std::collections::HashSet;

#[aoc_generator(day1)]
fn input_generator(input: &str) -> Vec<i64> {
    input
        .lines()
        .map(|s| s.parse().unwrap())
        .collect()
}

const TARGET_SUM: i64 = 2020;

#[aoc(day1, part1)]
fn part1(input: &[i64]) -> i64 {
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


#[aoc(day1, part2)]
fn part2(input: &[i64]) -> i64 {
    if input.len() < 3 {
        panic!("not enough input numbers");
    }

    for (i, first_num) in input[0..input.len()-2].iter().enumerate() {
        let remaining_sum = TARGET_SUM - first_num;

        let mut previous_numbers = HashSet::new();
        for second_num in input[i+1..].iter() {
            let third_num = remaining_sum - second_num;
            if previous_numbers.contains(&third_num) {
                return first_num * second_num * third_num;
            }
            previous_numbers.insert(second_num);
        }
   
    }
    panic!("no entries");
}
