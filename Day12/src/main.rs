use std::collections::HashSet;
use std::fs;
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn origin() -> Self {
        Self::new(0, 0)
    }

    pub fn up() -> Self {
        Self::new(0, -1)
    }

    pub fn down() -> Self {
        Self::new(0, 1)
    }

    pub fn left() -> Self {
        Self::new(-1, 0)
    }

    pub fn right() -> Self {
        Self::new(1, 0)
    }

    pub fn diagonal() -> impl Iterator<Item = Self> {
        (-1..=1).flat_map(|x| {
            (-1..=1)
                .filter(move |&y| x != 0 && y != 0)
                .map(move |y| Self::new(x, y))
        })
    }

    pub fn moore() -> impl Iterator<Item = Self> {
        (-1..=1).flat_map(|x| {
            (-1..=1)
                .filter(move |&y| x != 0 || y != 0)
                .map(move |y| Self::new(x, y))
        })
    }

    pub fn von_neumann() -> impl Iterator<Item = Self> {
        (-1..=1).flat_map(|x| {
            (-1..=1)
                .filter(move |&y| (x == 0) ^ (y == 0))
                .map(move |y| Self::new(x, y))
        })
    }

    pub fn manhattan_distance(&self, other: &Point) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    pub fn opposite(&self) -> Self {
        Self::new(-self.x, -self.y)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Mul for Point {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl MulAssign for Point {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl Mul<usize> for Point {
    type Output = Self;

    fn mul(self, rhs: usize) -> Self::Output {
        Self::new(self.x * rhs as i32, self.y * rhs as i32)
    }
}

impl MulAssign<usize> for Point {
    fn mul_assign(&mut self, rhs: usize) {
        self.x *= rhs as i32;
        self.y *= rhs as i32;
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl From<u8> for Point {
    fn from(value: u8) -> Self {
        match value {
            b'^' | b'U' => Self::up(),
            b'v' | b'D' => Self::down(),
            b'<' | b'L' => Self::left(),
            b'>' | b'R' => Self::right(),
            _ => unreachable!(),
        }
    }
}

impl From<Point> for u8 {
    fn from(value: Point) -> Self {
        match value {
            Point { x: 0, y: -1 } => b'^',
            Point { x: 0, y: 1 } => b'v',
            Point { x: -1, y: 0 } => b'<',
            Point { x: 1, y: 0 } => b'>',
            _ => unreachable!(),
        }
    }
}

pub fn part1(input: &str) -> u32 {
    get_regions(input)
        .iter()
        .map(|region| region.area * region.perimeter)
        .sum()
}

pub fn part2(input: &str) -> u32 {
    get_regions(input)
        .iter()
        .map(|region| region.area * region.sides())
        .sum()
}

fn parse(input: &str) -> Vec<&[u8]> {
    input.lines().map(|line| line.as_bytes()).collect()
}

fn get_regions(input: &str) -> Vec<Region> {
    let grid = parse(input);
    let mut visited = HashSet::new();  
    let mut regions = Vec::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, plant) in row.iter().enumerate() {
            let point = Point::new(x as i32, y as i32);

            if !visited.contains(&point) {
                let mut region = Region::default();
                find_region(&grid, &mut visited, &mut region, point, *plant);
                regions.push(region);
            }
        }
    }

    regions
}

fn find_region(
    grid: &Vec<&[u8]>,
    visited: &mut HashSet<Point>,  // ใช้ HashSet แทน FxHashSet
    region: &mut Region,
    point: Point,
    plant: u8,
) -> bool {
    if let Some(row) = grid.get(point.y as usize) {
        if let Some(&char) = row.get(point.x as usize) {
            if char == plant {
                if visited.contains(&point) {
                    return true;
                }

                visited.insert(point);
                region.area += 1;

                for neighbor in Point::von_neumann() {
                    let new_point = point + neighbor;
                    if !find_region(grid, visited, region, new_point, plant) {
                        region.perimeter += 1;
                        region.edges.insert(Edge(point, neighbor.into()));
                    }
                }

                return true;
            }
        }
    }

    false
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Edge(Point, u8);

#[derive(Default)]
struct Region {
    area: u32,
    perimeter: u32,
    edges: HashSet<Edge>,
}

impl Region {
    fn sides(&self) -> u32 {
        let mut to_remove = HashSet::new();
        let mut sorted: Vec<&Edge> = self.edges.iter().collect();
        sorted.sort();

        for edge in sorted {
            let sides = match edge.1 {
                b'^' | b'v' => [Point::left(), Point::right()],
                _ => [Point::up(), Point::down()],
            }
            .map(|point| Edge(edge.0 + point, edge.1));

            if sides
                .iter()
                .any(|fence| self.edges.contains(fence) && !to_remove.contains(fence))
            {
                to_remove.insert(*edge);
            }
        }

        self.edges.difference(&to_remove).count() as u32
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read the file");

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
