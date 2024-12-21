pub mod integer;
pub mod constant;
pub mod opts;

#[macro_export]
macro_rules! empty_trait {
    ($name:ident for $($t:ty)*) => ($(
        impl $name for $t {}
    )*)
}