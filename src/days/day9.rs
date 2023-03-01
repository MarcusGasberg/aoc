use std::{
    fs::read_to_string,
    ops::{Add, AddAssign, Sub, SubAssign}, collections::HashSet,
};

use itertools::Itertools;

pub fn main() {
    let Ok(input) = read_to_string("src/days/day9_input.txt") else {return;};

    let mut rope = Rope {
        head: RopePart {
            coordinate: Coordinate::from_tuple((0, 0)),
            visited: HashSet::new(),

        },
        parts: vec![RopePart {
            coordinate: Coordinate::from_tuple((0, 0)),
            visited: HashSet::new(),
        }; 9],
    };

    let moves = input.lines().map(Direction::parse);

    moves.for_each(|x| x.move_rope(&mut rope));
    let tail = rope.parts.last().unwrap();
    println!("{:?}", tail.visited.len());
    // println!("{:?}", moves.1);
}

#[derive(Debug)]
enum Direction {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

impl Direction {
    fn parse(i: &str) -> Self {
        match i.split(" ").collect_tuple().unwrap() {
            ("U", x) => Direction::Up(x.parse::<i32>().unwrap()),
            ("D", x) => Direction::Down(x.parse::<i32>().unwrap()),
            ("L", x) => Direction::Left(x.parse::<i32>().unwrap()),
            ("R", x) => Direction::Right(x.parse::<i32>().unwrap()),
            _ => panic!("SHIT"),
        }
    }

    fn get_direction_coordinate(&self) -> Coordinate {
        match self {
            Direction::Up(_) => Coordinate::from_tuple((0, 1)),
            Direction::Down(_) => Coordinate::from_tuple((0, -1)),
            Direction::Left(_) => Coordinate::from_tuple((-1, 0)),
            Direction::Right(_) => Coordinate::from_tuple((1, 0)),
        }
    }

    fn get_value(&self) -> i32 {
        match self {
            Direction::Up(x) => *x,
            Direction::Down(x) => *x,
            Direction::Left(x) => *x,
            Direction::Right(x) => *x,
        }
    }

    fn move_rope(&self, rope: &mut Rope) {
        let range_to = self.get_value();
        for _i in 0..range_to {
            rope.head.coordinate += self.get_direction_coordinate();
            rope.parts
                .iter_mut()
                .fold(&mut rope.head, |acc, rope_part| {
                    rope_part.follow(acc);

                    rope_part
                });
        }
    }
}

#[derive(Clone, Copy, Eq, Hash, Debug)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Coordinate {
    fn from_tuple(i: (i32, i32)) -> Self {
        Coordinate { x: i.0, y: i.1 }
    }

    fn to_tuple(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

pub(crate) struct Rope {
    head: RopePart,
    parts: Vec<RopePart>,
}

#[derive(Clone)]
pub(crate) struct RopePart {
    coordinate: Coordinate,
    visited: HashSet<Coordinate>,
}

impl RopePart {
    fn follow(&mut self, next: &RopePart) {
        let diff_coord = next.coordinate - self.coordinate;

        println!("==================");
        println!("NEXT: {:?}", next.coordinate);
        println!("TAIL: {:?}", self.coordinate);
        let move_direction_coord = match diff_coord.to_tuple() {
            // connected, stay
            (0, 1)
            | (1, 0)
            | (-1, 0)
            | (0, -1)
            | (-1, 1)
            | (1, -1)
            | (1, 1)
            | (-1, -1)
            | (0, 0)
             => (0, 0),
            // left
            (-2, 0) => (-1, 0),
            // right
            (2, 0) => (1, 0),
            // up
            (0, 2) => (0, 1),
            // down
            (0, -2) => (0, -1),
            // two down and left
            (-1, -2) | (-2, -1) | (-2, -2) => (-1, -1),
            // two down and right
            (1, -2) | (2, -1) | (2, -2) => (1, -1),
            // two up and left
            (-1, 2) | (-2, 1) | (-2, 2) => (-1, 1),
            // two up and right
            (1, 2) | (2, 1) | (2, 2) => (1, 1),
            x => panic!("Unsupported {:?}", x),
        };

        println!("Moving {:?}: {:?}", self.coordinate, move_direction_coord);

        self.coordinate += Coordinate::from_tuple(move_direction_coord);
        self.visited.insert(self.coordinate);
    }
}

impl Add for Coordinate {
    fn add(self, rhs: Self) -> Self::Output {
        Coordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }

    type Output = Self;
}

impl AddAssign for Coordinate {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Coordinate {
    fn sub(self, rhs: Self) -> Self::Output {
        Coordinate {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }

    type Output = Self;
}

impl SubAssign for Coordinate {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}
