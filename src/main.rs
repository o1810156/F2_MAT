use std::cmp::{Eq, PartialEq};
use std::fmt::Display;
use std::ops::*;

#[derive(Clone, Copy, PartialEq, Eq)]
enum F2Val {
    Z,
    O,
}

use F2Val::*;

impl Add for F2Val {
    type Output = F2Val;
    fn add(self, other: F2Val) -> F2Val {
        match (self, other) {
            (Z, Z) => Z,
            (Z, _) => O,
            (_, Z) => O,
            (_, _) => Z,
        }
    }
}

impl AddAssign for F2Val {
    fn add_assign(&mut self, other: F2Val) {
        *self = *self + other;
    }
}

impl Mul for F2Val {
    type Output = F2Val;
    fn mul(self, other: F2Val) -> F2Val {
        match (self, other) {
            (O, O) => O,
            _ => Z,
        }
    }
}

impl MulAssign for F2Val {
    fn mul_assign(&mut self, other: F2Val) {
        *self = *self * other;
    }
}

impl Display for F2Val {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Z => write!(f, "0"),
            O => write!(f, "1"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct F2Matrix<const S: usize>([[F2Val; S]; S]);

impl<const S: usize> Display for F2Matrix<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.0.iter() {
            for val in row.iter() {
                write!(f, "{}", val)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<const S: usize> F2Matrix<S> {
    fn zero() -> F2Matrix<S> {
        F2Matrix([[Z; S]; S])
    }

    fn e() -> F2Matrix<S> {
        let mut res = F2Matrix([[Z; S]; S]);
        for i in 0..S {
            res.0[i][i] = O;
        }
        res
    }
}

impl<const S: usize> Mul for F2Matrix<S> {
    type Output = F2Matrix<S>;
    fn mul(self, other: F2Matrix<S>) -> F2Matrix<S> {
        let mut result = F2Matrix::<S>::zero();
        for i in 0..S {
            for j in 0..S {
                for k in 0..S {
                    result.0[i][j] += self.0[i][k] * other.0[k][j];
                }
            }
        }
        result
    }
}

impl<const S: usize> MulAssign for F2Matrix<S> {
    fn mul_assign(&mut self, other: F2Matrix<S>) {
        *self = *self * other;
    }
}

fn main() {
    let x = F2Matrix::<3>([[Z, O, Z], [O, Z, Z], [O, O, O]]);

    let y = F2Matrix::<3>([[O, Z, Z], [Z, Z, O], [Z, O, O]]);

    let mut mx = x;
    let mut i = 2;
    let ord_x = loop {
        mx *= x;
        if mx == F2Matrix::<3>::e() {
            break i;
        }
        i += 1;
    };

    println!("ord(x) = {}", ord_x);

    let mut my = y;
    let mut i = 2;
    let ord_y = loop {
        my *= y;
        if my == F2Matrix::<3>::e() {
            break i;
        }
        i += 1;
    };
    println!("ord(y) = {}", ord_y);

    let xy = x * y;
    let mut mxy = xy;
    let mut i = 2;
    let ord_xy = loop {
        mxy *= xy;
        if mxy == F2Matrix::<3>::e() {
            break i;
        }
        i += 1;
    };
    println!("ord(xy) = {}", ord_xy);
}
