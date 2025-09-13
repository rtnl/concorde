use num_traits::Zero;
use std::ops::{Add, Div, Mul, Sub};

fn zero_div<T>(a: T, b: T) -> T
where
    T: Div<T, Output = T> + Zero + Copy,
{
    if b.is_zero() { T::zero() } else { a / b }
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct Vec2f {
    x: f64,
    y: f64,
}

impl Vec2f {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn of(value: f64) -> Self {
        Self { x: value, y: value }
    }

    pub fn zero() -> Self {
        Self::of(0.0)
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn flip(&self) -> Self {
        Self {
            x: self.y,
            y: self.x,
        }
    }

    pub fn flip_if(&self, condition: bool) -> Self {
        if condition { self.flip() } else { *self }
    }
}

impl Add for Vec2f {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Vec2f {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul for Vec2f {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl Div for Vec2f {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self::new(zero_div(self.x, rhs.x), zero_div(self.y, rhs.y))
    }
}
