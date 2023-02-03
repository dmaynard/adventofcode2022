use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
enum Direction {
    U,
    D,
    L,
    R,
}
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
}
#[derive(Debug)]
struct Board {
    head: Point,
    tail: Point,
}
impl Board {
    fn are_touching(&self) -> bool {
        ((self.head.x - self.tail.x).abs() < 2) && ((self.head.y - self.tail.y).abs() < 2)
    }
}

fn make_move(b: &mut Board, d: Direction) -> Point {
    match d {
        Direction::U => b.head.y = b.head.y + 1,
        Direction::D => b.head.y = b.head.y - 1,
        Direction::L => b.head.x = b.head.x - 1,
        Direction::R => b.head.x = b.head.x + 1,
    }
    let delta: (i32, i32) = match (b.tail.x - b.head.x, b.tail.y - b.head.y) {
        (0, 0) => (0, 0),
        (1, 0) => (0, 0),
        (0, 1) => (0, 0),
        (-1, 0) => (0, 0),
        (0, -1) => (0, 0),
        (1, 1) => (0, 0),
        (1, -1) => (0, 0),
        (-1, 1) => (0, 0),
        (-1, -1) => (0, 0),
        (2, 0) => (-1, 0),
        (-2, 0) => (1, 0),
        (0, 2) => (0, -1),
        (0, -2) => (0, 1),
        // diagonal cases right side
        (1, 2) => (-1, -1),
        (2, 1) => (-1, -1),
        (2, -1) => (-1, 1),
        (1, -2) => (-1, 1),
        // diagonal cases left side
        (-1, 2) => (1, -1),
        (-2, 1) => (1, -1),
        (-2, -1) => (1, 1),
        (-1, -2) => (1, 1),
        (_, _) => {
            println!("impossible board {:?} ", b);
            (0, 0)
        }
    };
    b.tail.x = b.tail.x + delta.0;
    b.tail.y = b.tail.y + delta.1;
    b.tail
}

pub fn process_part1(input: &str) -> String {
    let mut visited: HashSet<Point> = HashSet::new();
    let mut board = Board {
        head: Point { x: 0, y: 0 },
        tail: Point { x: 0, y: 0 },
    };
    visited.insert(board.tail);
    for line in input.lines() {
        let d = match &line[0..1] {
            "U" => Direction::U,
            "D" => Direction::D,
            "L" => Direction::L,
            "R" => Direction::R,
            _ => panic!("unknown direction"),
        };
        let times: u32 = line[2..line.len()].parse().unwrap();
        for _i in 1..=times {
            visited.insert(make_move(&mut board, d));
        }
    }

    visited.len().to_string()
}
pub fn process_part2(input: &str) -> String {
    "42".to_string()

    //vmax.unwrap().unwrap().to_string()
    //  max_view.to_string()
}
#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::SliceRandom;
    use rand::Rng;

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]

    fn it_works() {
        let board = Board {
            head: Point { x: 0, y: 0 },
            tail: Point { x: 0, y: 0 },
        };
        assert!(board.are_touching());
        let board2 = Board {
            head: Point { x: 0, y: 0 },
            tail: Point { x: 1, y: 1 },
        };
        assert!(board2.are_touching());
        let board3 = Board {
            head: Point { x: -1, y: -1 },
            tail: Point { x: 1, y: 1 },
        };

        assert!(!board3.are_touching());
        let result = process_part1(INPUT);

        let mut test_board = Board {
            head: Point { x: 0, y: 0 },
            tail: Point { x: 0, y: 0 },
        };

        let directions = [Direction::U, Direction::D, Direction::L, Direction::R];
        let mut rng = rand::thread_rng();
        for _i in 0..10 {
            let direction = directions.choose(&mut rng).unwrap();
            make_move(&mut test_board, *direction);
            assert!(test_board.are_touching());
        }
        assert_eq!(result, "13");
    }

    #[test]
    #[ignore]
    fn part2_works() {
        let result = process_part2(INPUT);
        assert_eq!(result, "8");
    }
}
