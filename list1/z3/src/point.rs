use std::ops::*;

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

macro_rules! arithmetic_impl {
    ($trait:ident, $method:ident, $op:tt) => {
        impl $trait for Point {
            type Output = Point;

            fn $method(self, rhs: Self) -> Self::Output {
                Self::Output {
                    x: self.x $op rhs.x,
                    y: self.y $op rhs.y,
                }
            }
        }
    };

    ($trait:ident, $method:ident, assign $op:tt) => {
        impl $trait for Point {
            fn $method(&mut self, rhs: Self) {
                self.x $op rhs.x;
                self.y $op rhs.y;
            }
        }
    };
}

arithmetic_impl!(Add, add, +);
arithmetic_impl!(Sub, sub, -);
arithmetic_impl!(Mul, mul, *);
arithmetic_impl!(Div, div, /);

arithmetic_impl!(AddAssign, add_assign, assign +=);
arithmetic_impl!(SubAssign, sub_assign, assign -=);
arithmetic_impl!(MulAssign, mul_assign, assign *=);
arithmetic_impl!(DivAssign, div_assign, assign /=);
