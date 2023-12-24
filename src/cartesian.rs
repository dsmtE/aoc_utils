use std::{
    array::from_fn,
    fmt::Debug,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign, Neg, Deref, DerefMut},
};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Point<const N: usize>(pub [i64; N]);

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Vector<const N: usize>(pub [i64; N]);

pub type Point2 = Point<2>;
pub type Vector2 = Vector<2>;

// TODO: extend to 3 dimensions

// Could also be done with derive_more:{Deref, DerefMut}
// But done manually here to avoid additional dependencies
impl<const N: usize> Deref for Point<N> {
    type Target = [i64; N];
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<const N: usize> DerefMut for Point<N> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

impl<const N: usize> Deref for Vector<N> {
    type Target = [i64; N];
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<const N: usize> DerefMut for Vector<N> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

pub trait Zero: Sized {
    fn zero() -> Self;
}

impl<const N: usize> Zero for Point<N> { fn zero() -> Self { Self([0; N]) } }
impl<const N: usize> Default for Point<N> { fn default() -> Self {  Self([0; N]) } }

impl<const N: usize> Zero for Vector<N> { fn zero() -> Self { Self([0; N]) } }
impl<const N: usize> Default for Vector<N> { fn default() -> Self {  Self([0; N]) } }

impl<const N: usize> Debug for Point<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "p({})", self.0.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "))
    }
}

impl<const N: usize> Debug for Vector<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "v({})", self.0.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", "))
    }
}

pub fn p2(x: i64, y: i64) -> Point<2> { Point([x, y]) }
pub fn v2(x: i64, y: i64) -> Vector<2> { Vector([x, y]) }

impl<const N: usize> Point<N> {
    pub fn origin() -> Self { Self::default() }
    pub fn vector(&self) -> Vector<N> { Vector(**self) }
}

impl<const N: usize> Vector<N> {
    pub fn origin() -> Self { Self::default() }
    pub fn point(&self) -> Point<N> { Point(**self)}

    pub fn normalize(self) -> Self {
        assert!(self.is_aligned());
        self / self.manhattan_dist()
    }

    /// vector is a multiple of a basis vector (single axis)
    pub fn is_aligned(self) -> bool { self.iter().any(|&x| x == 0) }
    pub fn square_magnitude(self) -> i64 { (*self).into_iter().map(|x| x*x).sum() }
}

impl Vector<2> {
    pub const UP: Vector2 = Vector([0, 1]);
    pub const DOWN: Vector2 = Vector([0, -1]);
    pub const LEFT: Vector2 = Vector([-1, 0]);
    pub const RIGHT: Vector2 = Vector([1, 0]);

    pub fn cross(&self, other: &Self) -> i64 {
        let Vector([a, b]) = self;
        let Vector([x, y]) = other;
        a * y - b * x
    }
}

impl From<char> for Vector2 {
    fn from(c: char) -> Self {
        Direction::from(c).vector()
    }
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '^' | 'U' | 'u' => Direction::Up,
            'v' | 'D' | 'd' => Direction::Down,
            '<' | 'L' | 'l' => Direction::Left,
            '>' | 'R' | 'r' => Direction::Right,
            _ => panic!("{c} is not a valid direction"),
        }
    }
}

impl<const N: usize> Sub for Point<N> {
    type Output = Vector<N>;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector(from_fn(|i| self.0[i] - rhs.0[i]))
    }
}

impl<const N: usize> Add for Vector<N> {
    type Output = Vector<N>;
    fn add(self, rhs: Self) -> Self::Output {
        Self(from_fn(|i| self.0[i] + rhs.0[i]))
    }
}

impl<const N: usize> Sub for Vector<N> {
    type Output = Vector<N>;
    fn sub(self, rhs: Self) -> Self::Output {
        Vector(from_fn(|i| self.0[i] - rhs.0[i]))
    }
}

impl<const N: usize> AddAssign for Vector<N> {
    fn add_assign(&mut self, rhs: Self) { *self = *self + rhs; }
}

impl<const N: usize> SubAssign for Vector<N> {
    fn sub_assign(&mut self, rhs: Self) { *self = *self - rhs; }
}

impl<const N: usize> Neg for Vector<N> {
    type Output = Self;
    fn neg(self) -> Self::Output { self * -1 }
}

impl<const N: usize> Sub<Vector<N>> for Point<N> {
    type Output = Self;

    fn sub(self, rhs: Vector<N>) -> Self::Output {
        Self(from_fn(|i| self.0[i] - rhs.0[i]))
    }
}

impl<const N: usize> Add<Vector<N>> for Point<N> {
    type Output = Self;

    fn add(self, rhs: Vector<N>) -> Self::Output {
        Self(from_fn(|i| self.0[i] + rhs.0[i]))
    }
}

impl<const N: usize> Add<Point<N>> for Vector<N> {
    type Output = Point<N>;
    fn add(self, rhs: Point<N>) -> Self::Output { Add::add(rhs, self) }
}

impl<const N: usize> Mul<i64> for Vector<N> {
    type Output = Self;
    fn mul(self, rhs: i64) -> Self::Output { Self(self.0.map(|x| x * rhs)) }
}

impl<const N: usize> MulAssign<i64> for Vector<N> {
    fn mul_assign(&mut self, rhs: i64) { *self = Mul::mul(*self, rhs) }
}

impl<const N: usize> Mul<Vector<N>> for i64 {
    type Output = Vector<N>;
    fn mul(self, rhs: Vector<N>) -> Self::Output { Mul::mul(rhs, self) }
}

impl<const N: usize> Div<i64> for Vector<N> {
    type Output = Self;
    fn div(self, rhs: i64) -> Self::Output { Self(self.0.map(|x| x / rhs)) }
}

// TODO: macro assignOperator from operator
impl<const N: usize> DivAssign<i64> for Vector<N> {
    fn div_assign(&mut self, rhs: i64) { *self = Div::div(*self, rhs) }
}

impl<const N: usize> Add<i64> for Vector<N> {
    type Output = Self;
    fn add(self, rhs: i64) -> Self::Output { Self(self.0.map(|x| x + rhs)) }
}

impl<const N: usize> AddAssign<i64> for Vector<N> {
    fn add_assign(&mut self, rhs: i64) { *self = *self + rhs; }
}

// TODO: macro commutative Ops from Ops
impl<const N: usize> Add<Vector<N>> for i64 {
    type Output = Vector<N>;
    fn add(self, rhs: Vector<N>) -> Self::Output { Add::add(rhs, self) }
}

impl<const N: usize> Sub<i64> for Vector<N> {
    type Output = Self;
    fn sub(self, rhs: i64) -> Self::Output { Self(self.0.map(|x| x - rhs)) }
}

// Vector <-> Point
impl<const N: usize> AddAssign<Vector<N>> for Point<N> {
    fn add_assign(&mut self, rhs: Vector<N>) { *self = *self + rhs; }
}

impl<const N: usize> SubAssign<Vector<N>> for Point<N> {
    fn sub_assign(&mut self, rhs: Vector<N>) { *self = *self - rhs; }
}

pub trait Cartesian<const N: usize>:
    Sized + Default + Clone + Copy + Deref<Target = [i64; N]> + DerefMut<Target = [i64; N]> + Eq
{
    fn new(x: [i64; N]) -> Self;

    fn manhattan_dist(&self) -> i64 { (*self).into_iter().map(|x| x.abs()).sum() }
}

pub trait Cartesian2: Cartesian<2> {
    #[inline]
    fn x(&self) -> i64 { self[0] }
    #[inline]
    fn y(&self) -> i64 { self[1] }

    #[inline]
    fn x_ref(&self) -> &i64 { &self[0] }
    #[inline]
    fn y_ref(&self) -> &i64 { &self[1] }

    #[inline]
    fn x_ref_mut(&mut self) -> &mut i64 { &mut self[0] }
    #[inline]
    fn y_ref_mut(&mut self) -> &mut i64 { &mut self[1] }

    fn up(&self, n: i64) -> Self { Self::new([self.x(), self.y() + n]) }
    fn down(&self, n: i64) -> Self { Self::new([self.x(), self.y() - n]) }
    fn left(&self, n: i64) -> Self { Self::new([self.x() - n, self.y()]) }
    fn right(&self, n: i64) -> Self { Self::new([self.x() + n, self.y()]) }

    fn rotate_right(&self) -> Self { Self::new([self.y(), -self.x()]) }
    fn rotate_left(&self) -> Self { Self::new([-self.y(), self.x()]) }
}

// Automatically implement Cartesian2 for any type that implements Cartesian<2>
impl<T: Cartesian<2>> Cartesian2 for T {}

impl<const N: usize> Cartesian<N> for Point<N> {
    fn new(x: [i64; N]) -> Self { Self(x) }
}

impl<const N: usize> Cartesian<N> for Vector<N> {
    fn new(x: [i64; N]) -> Self { Self(x) }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn vector(&self) -> Vector2 {
        match self {
            Direction::Up => v2(0, -1),
            Direction::Down => v2(0, 1),
            Direction::Left => v2(-1, 0),
            Direction::Right => v2(1, 0),
        }
    }
}

