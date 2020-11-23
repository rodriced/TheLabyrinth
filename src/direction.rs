use crate::vect2_t::{UCoord2, UDelta2};

use std::{
    fmt, iter
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label())
    }
}

impl Direction {
    fn id(&self) -> usize {
        match self {
            Self::Right => 0,
            Self::Down => 1,
            Self::Left => 2,
            Self::Up => 3,
        }
    }

    fn array() -> [Self; 4] {
        [Self::Right, Self::Down, Self::Left, Self::Up]
    }

    pub fn label(&self) -> &str {
        match self {
            Self::Right => "RIGHT",
            Self::Down => "DOWN",
            Self::Left => "LEFT",
            Self::Up => "UP",
        }
    }

    fn next(&self) -> Self {
        match self {
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Up => Self::Right,
        }
    }
    fn reverse(&self) -> Self {
        match self {
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Up => Self::Down,
        }
    }
    fn to_delta(&self) -> UDelta2 {
        match self {
            Self::Right => (1, 0).into(),
            Self::Down => (0, 1).into(),
            Self::Left => (-1, 0).into(),
            Self::Up => (0, -1).into(),
        }
    }
    fn from_delta(vect: &UDelta2) -> Direction {
        match (vect.x, vect.y) {
            (1, 0) => Self::Right,
            (0, 1) => Self::Down,
            (-1, 0) => Self::Left,
            (0, -1) => Self::Up,
            _ => panic!(),
        }
    }
    fn apply_to_coord(&self, start: UCoord2) -> UCoord2 {
        start + self.to_delta()
    }

    fn iter_from(start: Direction) -> impl Iterator<Item = Direction> {
        iter::successors(Some(start), |dir| Some(dir.next()))
    }

    fn successors(self) -> impl Iterator<Item = Direction> {
        iter::successors(Some(self), |&dir| Some(dir.next()))
    }
}

impl From<(UCoord2, UCoord2)> for Direction {
    fn from((src, dest): (UCoord2, UCoord2)) -> Self {
        (dest - src).into()
    }
}

impl From<UDelta2> for Direction {
    fn from(v: UDelta2) -> Self {
        let UDelta2 { x, y } = v;
        if y >= -x && y < x {
            Direction::Right
        } else if x <= y && x > -y {
            Direction::Down
        } else if y <= -x && y > x {
            Direction::Left
        } else if x >= y && x < -y {
            Direction::Up
        } else {
            panic!()
        }
    }
}

#[test]
fn test_vect_to_dir() {
    macro_rules! assert_vect_into_dir {
        ($x:expr, $y:expr, $dir:ident) => {
            let v: UDelta2 = UDelta2::new($x, $y);
            eprintln!("{:?} -> {}", v, Direction::$dir);
            assert_eq!(Direction::$dir, v.into());
        };
    }

    assert_vect_into_dir!(2, -2, Right);
    assert_vect_into_dir!(2, -1, Right);
    assert_vect_into_dir!(2, 1, Right);
    assert_vect_into_dir!(2, 2, Down);
    assert_vect_into_dir!(1, 2, Down);
    assert_vect_into_dir!(-1, 2, Down);
    assert_vect_into_dir!(-2, 2, Left);
    assert_vect_into_dir!(-2, 1, Left);
    assert_vect_into_dir!(-2, -1, Left);
    assert_vect_into_dir!(-2, -2, Up);
    assert_vect_into_dir!(-1, -2, Up);
    assert_vect_into_dir!(1, -2, Up);
}

impl UCoord2 {
    pub fn get_neighbour(&self, dir: &Direction) -> UCoord2 {
        *self + dir.to_delta()
    }

    pub fn neighbours_iter(
        self,
        first_dir: Direction,
    ) -> impl Iterator<Item = UCoord2> {
        first_dir
            .successors()
            .take(4)
            .map(move |dir| self.get_neighbour(&dir))
    }
}
