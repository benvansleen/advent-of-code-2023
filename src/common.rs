use std::fmt::Debug;
use std::iter::{Product, Sum};
use std::ops::{Add, Sub, Mul, Div};
use std::str::FromStr;

pub trait BoundedInt:
    Copy
    + Ord
    + FromStr
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Debug
    + ilog::IntLog
    + Default
    + Product
    + Sum
    + Send
    + Sync
    + 'static
{
    fn min_value() -> Self;
    fn max_value() -> Self;
}

impl BoundedInt for u8 {
    fn min_value() -> Self {
        0
    }

    fn max_value() -> Self {
        std::u8::MAX
    }
}

impl BoundedInt for u16 {
    fn min_value() -> Self {
        0
    }

    fn max_value() -> Self {
        std::u16::MAX
    }
}

impl BoundedInt for u32 {
    fn min_value() -> Self {
        0
    }

    fn max_value() -> Self {
        std::u32::MAX
    }
}

impl BoundedInt for u64 {
    fn min_value() -> Self {
        0
    }

    fn max_value() -> Self {
        std::u64::MAX
    }
}

impl BoundedInt for u128 {
    fn min_value() -> Self {
        0
    }

    fn max_value() -> Self {
        std::u128::MAX
    }
}

impl BoundedInt for usize {
    fn min_value() -> Self {
        0
    }

    fn max_value() -> Self {
        std::usize::MAX
    }
}

impl BoundedInt for i8 {
    fn min_value() -> Self {
        std::i8::MIN
    }

    fn max_value() -> Self {
        std::i8::MAX
    }
}

impl BoundedInt for i16 {
    fn min_value() -> Self {
        std::i16::MIN
    }

    fn max_value() -> Self {
        std::i16::MAX
    }
}

impl BoundedInt for i32 {
    fn min_value() -> Self {
        std::i32::MIN
    }

    fn max_value() -> Self {
        std::i32::MAX
    }
}

impl BoundedInt for i64 {
    fn min_value() -> Self {
        std::i64::MIN
    }

    fn max_value() -> Self {
        std::i64::MAX
    }
}

impl BoundedInt for i128 {
    fn min_value() -> Self {
        std::i128::MIN
    }

    fn max_value() -> Self {
        std::i128::MAX
    }
}
