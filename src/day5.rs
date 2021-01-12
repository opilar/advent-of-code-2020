#[aoc_generator(day5)]
fn input_generator(input: &str) -> Vec<Seat> {
    input.lines().map(Seat::parse).collect()
}

#[aoc(day5, part1)]
fn part1(seats: &[Seat]) -> usize {
    seats.iter().map(|s| s.id()).max().unwrap()
}

#[aoc(day5, part2)]
fn part2(seats: &[Seat]) -> usize {
    let first = seats.iter().map(|s| s.id()).min().unwrap();
    let last = seats.iter().map(|s| s.id()).max().unwrap();
    let number_of_seats = last - first + 1;
    let sum_of_tickets = number_of_seats * (first + last) / 2;
    seats
        .iter()
        .map(|s| s.id())
        .fold(sum_of_tickets, |sum, s| sum - s)
}

#[derive(Debug, PartialEq)]
struct Seat {
    row: usize,
    col: usize,
}

impl Seat {
    fn id(&self) -> usize {
        self.row * COLS + self.col
    }

    fn parse(s: &str) -> Self {
        let row = Seat::parse_row(&s[0..7]);
        let col = Seat::parse_col(&s[7..]);
        Seat { row, col }
    }

    fn parse_row(s: &str) -> usize {
        if s.len() != 7 {
            panic!("incorrect length of row string: {:?}", s);
        }

        let mut n = 0;
        for c in s.chars() {
            n <<= 1;
            if c == 'B' {
                n += 1;
            }
        }
        n
    }

    fn parse_col(s: &str) -> usize {
        if s.len() != 3 {
            panic!("incorrect length of col string: {:?}", s);
        }

        let mut n = 0;
        for c in s.chars() {
            n <<= 1;
            if c == 'R' {
                n += 1;
            }
        }
        n
    }
}

const COLS: usize = 8;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_seat_id() {
        assert_eq!(Seat::parse("FBFBBFFRLR"), Seat { row: 44, col: 5 });
        assert_eq!(Seat::parse("BFFFBBFRRR"), Seat { row: 70, col: 7 });
        assert_eq!(Seat::parse("FFFBBBFRRR"), Seat { row: 14, col: 7 });
        assert_eq!(Seat::parse("BBFFBBFRLL"), Seat { row: 102, col: 4 });
    }
}
