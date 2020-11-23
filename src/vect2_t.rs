use std::{
    fmt::{self, Display},
    ops,
};

pub type UCoord2 = Vect2<usize>;
pub type UDelta2 = Vect2<i32>;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vect2<T> {
    pub x: T,
    pub y: T,
}

impl<T> fmt::Display for Vect2<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl<T> Vect2<T>
where
    T: Copy
        + PartialEq
        + ops::Add<Output = T>
        + ops::AddAssign
        + ops::Sub
        + ops::SubAssign
        + ops::Mul<Output = T>
        + ops::MulAssign,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    pub fn is(&self, x: T, y: T) -> bool {
        self.x == x && self.y == y
    }

    pub fn add(mut self, rhs: Self) -> Self {
        self.x += rhs.x;
        self.y += rhs.y;
        self
    }

    pub fn sub(mut self, rhs: Self) -> Self {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self
    }

    pub fn length2(&self) -> T {
        self.x * self.x + self.y * self.y
    }
}

// macro_rules! int_vect2_idx {
//     ($t:ty) => {
//         impl Vect2<$t> {
//             pub fn x_idx(&self) -> usize {
//                 if self.x < 0 {
//                     panic!()
//                 }
//                 self.x as usize
//             }

//             pub fn y_idx(&self) -> usize {
//                 if self.y < 0 {
//                     panic!()
//                 }
//                 self.y as usize
//             }

//             pub fn idx(&self) -> UCoord2 {
//                 UCoord2::new(self.x_idx(), self.y_idx())
//             }

//             pub fn length(&self) -> f64 {
//                 // ((self.x * self.x + self.y * self.y) as f64).sqrt()
//                 (self.x as f64).hypot(self.y as f64)
//             }
//         }
//     };
// }

// int_vect2_idx!(i32);
// coord2_int_idx!(usize);

impl Vect2<f64> {
    fn length_optim(&self) -> f64 {
        self.x.hypot(self.y)
    }

    pub fn x_usize(&self) -> usize {
        if self.x.is_sign_negative() {
            panic!()
        }
        self.x as usize
    }

    pub fn y_usize(&self) -> usize {
        if self.y.is_sign_negative() {
            panic!()
        }
        self.y as usize
    }

    pub fn xy_usize(&self) -> UCoord2 {
        UCoord2::new(self.x_usize(), self.y_usize())
    }
}

// impl<T> From<(T, T)> for Vect2<T> {
//     fn from(t: (T, T)) -> Self {
//         Self { x: t.0, y: t.1 }
//     }
// }

macro_rules! vect2_from {
    ($t:ty, $from:ty) => {
        impl From<($from, $from)> for Vect2<$t> {
            fn from(t: ($from, $from)) -> Self {
                Self {
                    x: t.0 as $t,
                    y: t.1 as $t,
                }
            }
        }

        impl From<&($from, $from)> for Vect2<$t> {
            fn from(t: &($from, $from)) -> Self {
                Self {
                    x: t.0 as $t,
                    y: t.1 as $t,
                }
            }
        }
    };
}

// vect2_from!(f64, u8);
// vect2_from!(f64, i8);
// vect2_from!(f64, u16);
// vect2_from!(f64, i16);
// vect2_from!(f64, u32);
// vect2_from!(f64, i32);
// vect2_from!(f64, usize);
// vect2_from!(f64, isize);

vect2_from!(i32, u32);
vect2_from!(i32, i32);
vect2_from!(i32, usize);
vect2_from!(i32, isize);

vect2_from!(usize, u32);
vect2_from!(usize, i32);
vect2_from!(usize, usize);
vect2_from!(usize, isize);

// vect2_from!(isize, u32);
// vect2_from!(isize, i32);
// vect2_from!(isize, usize);
// vect2_from!(isize, isize);


impl ops::Add<UDelta2> for UCoord2
{
    type Output = Self;

    fn add(self, rhs: UDelta2) -> Self::Output {
        Self {
            x: (self.x as i32 + rhs.x) as usize,
            y: (self.y as i32 + rhs.y) as usize,
        }
    }
}

impl ops::Sub<UCoord2> for UCoord2
{
    type Output = UDelta2;

    fn sub(self, rhs: UCoord2) -> Self::Output {
        Vect2 {
            x: self.x as i32 - rhs.x as i32,
            y: self.y as i32 - rhs.y as i32,
        }
    }
}

impl ops::Sub<UDelta2> for UDelta2
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T> ops::Add for Vect2<T>
where
    T: ops::Add<Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> ops::AddAssign for Vect2<T>
where
    T: ops::AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

// impl<T> ops::Sub for Vect2<T>
// where
//     T: ops::Sub<Output = T>,
// {
//     type Output = Self;

//     fn sub(self, rhs: Self) -> Self::Output {
//         Self {
//             x: self.x - rhs.x,
//             y: self.y - rhs.y,
//         }
//     }
// }

impl<T> ops::SubAssign for Vect2<T>
where
    T: ops::SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T> ops::Neg for Vect2<T>
where
    T: ops::Neg<Output = T>,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

// pub fn test() {
//     let a: Vect2<f64> = (1.2, 2.5).into();
//     let b: Vect2<f64> = (3., 6.).into();

//     let c = a + b;

//     // let d = c.scale(3.);

//     println!("{:?}", c.xy_usize());
// }