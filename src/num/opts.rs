use std::ops::*;

pub trait ArithOps<Rhs = Self, Output = Self>:
    Add<Rhs, Output = Output>
    + Sub<Rhs, Output = Output>
    + Mul<Rhs, Output = Output>
    + Div<Rhs, Output = Output>
    + Rem<Rhs, Output = Output>
{}

impl<T, Rhs, Output> ArithOps<Rhs, Output> for T where
    T: Add<Rhs, Output = Output>
        + Sub<Rhs, Output = Output>
        + Mul<Rhs, Output = Output>
        + Div<Rhs, Output = Output>
        + Rem<Rhs, Output = Output>
{}

pub trait BitOps<Rhs = Self, Output = Self>:
    BitAnd<Rhs, Output = Output>
    + BitOr<Rhs, Output = Output>
    + BitXor<Rhs, Output = Output>
    + Shl<Rhs, Output = Output>
    + Shr<Rhs, Output = Output>
{}

impl<T, Rhs, Output> BitOps<Rhs, Output> for T where
    T: BitAnd<Rhs, Output = Output>
    + BitOr<Rhs, Output = Output>
    + BitXor<Rhs, Output = Output>
    + Shl<Rhs, Output = Output>
    + Shr<Rhs, Output = Output>
{}

pub trait ArithAssignOps<Rhs = Self>:
    AddAssign<Rhs> + SubAssign<Rhs> + MulAssign<Rhs> + DivAssign<Rhs> + RemAssign<Rhs>
{}

impl<T, Rhs> ArithAssignOps<Rhs> for T where
    T: AddAssign<Rhs> + SubAssign<Rhs> + MulAssign<Rhs> + DivAssign<Rhs> + RemAssign<Rhs>
{}