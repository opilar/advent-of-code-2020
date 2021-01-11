#[aoc_generator(day5)]
fn input_generator(input: &str) -> Vec<Seat> {
    input.lines().map(Seat::parse).collect()
}

#[aoc(day5, part1)]
fn part1(seats: &[Seat]) -> usize {
    seats.iter().map(|s| s.id()).max().unwrap()
}

struct Seat {
    row: usize,
    col: usize,
}

impl Seat {
    fn id(&self) -> usize {
        self.row * ROW_SIZE + self.col
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

const ROW_SIZE: usize = 8;
