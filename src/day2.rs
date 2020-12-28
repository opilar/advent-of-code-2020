use std::str::FromStr;

struct Input {
    policy: Policy,
    password: String,
}

impl Input {
    fn parse(s: &str) -> Result<Input, String> {
        let components: Vec<&str> = s.split(": ").collect();
        if components.len() != 2 {
            return Err(format!("wrong input: {:?}", s));
        }
        return Ok(Input{
            policy: Policy::parse(components[0])?,
            password: components[1].to_owned(),
        })
    }

    fn validate_part1(&self) -> bool {
        self.policy.is_valid_part1(&self.password)
    }

    fn validate_part2(&self) -> bool {
        self.policy.is_valid_part2(&self.password)
    }
}

struct Policy {
    lowest: usize,
    highest: usize,
    letter: char,
}

impl Policy {
    fn parse(s: &str) -> Result<Policy, String> {
        let components: Vec<&str> = s.split('-').collect();
        if components.len() != 2 {
            return Err(format!("wrong input: {:?}", s));
        }
        let lowest = usize::from_str(components[0]).unwrap();

        let components: Vec<&str> = components[1].split(' ').collect();
        let highest = usize::from_str(components[0]).unwrap();
        let letter = components[1].chars().nth(0).unwrap();

        Ok(Policy{
            lowest,
            highest,
            letter,
        })
    }

    fn is_valid_part1(&self, s: &str) -> bool {
        let letter_count = s.chars().filter(|c| *c == self.letter).count();
        letter_count >= self.lowest && letter_count <= self.highest
    }

    fn is_valid_part2(&self, s: &str) -> bool {
        let lowest_char = s.chars().nth(self.lowest-1).unwrap();
        let highest_char = s.chars().nth(self.highest-1).unwrap();
        (lowest_char == self.letter) ^ (highest_char == self.letter)
    }
}

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<Input> {
    input
        .lines()
        .map(|s| Input::parse(s).unwrap())
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &[Input]) -> usize {
    input.iter().filter(|i| i.validate_part1()).count()
}

#[aoc(day2, part2)]
fn part2(input: &[Input]) -> usize {
    input.iter().filter(|i| i.validate_part2()).count()
}
