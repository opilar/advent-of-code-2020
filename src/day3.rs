use std::{
    ops::{Add, Mul},
    usize,
};

struct Row {
    cells: Vec<Cell>,
}

impl Row {
    fn parse(s: &str) -> Self {
        let cells = s.chars().map(Cell::parse).collect();
        Row { cells }
    }

    fn at(&self, col: usize) -> Cell {
        let col = col % self.cells.len();
        self.cells[col]
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Cell {
    Open,
    Tree,
}

impl Cell {
    fn parse(c: char) -> Cell {
        match c {
            '.' => Cell::Open,
            '#' => Cell::Tree,
            _ => panic!("unexpected cell char {:?}", c),
        }
    }
}

struct Map {
    rows: Vec<Row>,
}

impl Map {
    fn parse(s: &str) -> Self {
        let rows = s.lines().map(Row::parse).collect();
        Map { rows }
    }

    fn at(&self, pos: Position) -> Option<Cell> {
        if pos.row >= self.rows.len() {
            None
        } else {
            let row = &self.rows[pos.row];
            Some(row.at(pos.col))
        }
    }

    fn iter_with_slope(&self, slope: Position) -> SlopeIterator {
        SlopeIterator {
            position: Position { col: 0, row: 0 },
            map: &self,
            slope,
        }
    }
}

struct SlopeIterator<'a> {
    position: Position,
    map: &'a Map,
    slope: Position,
}

impl Iterator for SlopeIterator<'_> {
    type Item = Cell;

    fn next(&mut self) -> Option<Self::Item> {
        self.position = self.position + self.slope;
        self.map.at(self.position)
    }
}

#[derive(Copy, Clone)]
struct Position {
    row: usize,
    col: usize,
}

impl Add for Position {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let row = self.row + other.row;
        let col = self.col + other.col;
        Position { row, col }
    }
}

#[aoc_generator(day3)]
fn input_generator(input: &str) -> Map {
    Map::parse(input)
}

#[aoc(day3, part1)]
fn part1(map: &Map) -> usize {
    const SLOPE: Position = Position { row: 1, col: 3 };

    count_trees(map, SLOPE)
}

#[aoc(day3, part2)]
fn part2(map: &Map) -> usize {
    let slopes = vec![
        Position { row: 1, col: 1 },
        Position { row: 1, col: 3 },
        Position { row: 1, col: 5 },
        Position { row: 1, col: 7 },
        Position { row: 2, col: 1 },
    ];

    slopes
        .iter()
        .map(|slope| count_trees(map, *slope))
        .fold(1, usize::mul)
}

fn count_trees(map: &Map, slope: Position) -> usize {
    map.iter_with_slope(slope)
        .filter(|c| *c == Cell::Tree)
        .count()
}
