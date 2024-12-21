use std::ops::*;

use crate::empty_trait;

use super::{constant::{One, Ten, Two, Zero}, opts::{ArithAssignOps, ArithOps, BitOps}};

pub trait Integer:
    Sized 
    + From<u8>
    + PartialOrd + Ord + Eq
    + ArithOps + BitOps + ArithAssignOps
    + Zero + One + Two + Ten
{}

empty_trait!(Integer for u8 u16 u32 u64 u128 usize i16 i32 i64 i128);

pub trait Unsigned: Integer {}
pub trait Signed: Integer + Neg {}

empty_trait!(Unsigned for u8 u16 u32 u64 u128 usize);
empty_trait!(Signed for i16 i32 i64 i128);

pub fn digits_count<T>(number: &T) -> usize 
where T: Integer + Copy {
    let mut count = 0;
    let mut number = *number;
    while number > T::ZERO {
        number /= T::TEN;
        count += 1;
    }
    count
}