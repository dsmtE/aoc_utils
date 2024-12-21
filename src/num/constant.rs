use paste::paste;

pub trait Zero {
    const ZERO: Self;
}
pub trait One {
    const ONE: Self;
}
pub trait Two {
    const TWO: Self;
}
pub trait Ten {
    const TEN: Self;
}

macro_rules! const_trait {
    ($value:expr, $trait_name:ident for $($t:ty)*) => ($(
        paste!{
            impl $trait_name for $t {
                const [<$trait_name:upper>]: $t = $value;
            }
        }
    )*)
}

const_trait!(0, Zero for u8 u16 u32 u64 u128 usize i16 i32 i64 i128);
const_trait!(1, One for u8 u16 u32 u64 u128 usize i16 i32 i64 i128);
const_trait!(2, Two for u8 u16 u32 u64 u128 usize i16 i32 i64 i128);
const_trait!(10, Ten for u8 u16 u32 u64 u128 usize i16 i32 i64 i128);

const_trait!(0., Zero for f32 f64);
const_trait!(1., One for f32 f64);
const_trait!(2., Two for f32 f64);
const_trait!(10., Ten for f32 f64);