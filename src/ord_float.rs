use std::{
    fmt::Display,
    iter::Sum,
    ops::{Add, Deref, DerefMut, Div, Mul, Neg, Sub},
};

macro_rules! create_ord_float {
    ($wrapper:ident, $inner:ty, $inner_bits:ty) => {
        #[derive(Debug, Clone, Copy)]
        pub struct $wrapper(pub $inner);

        impl Deref for $wrapper {
            type Target = $inner;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl DerefMut for $wrapper {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }

        impl Display for $wrapper {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<$inner> for $wrapper {
            fn from(value: $inner) -> Self {
                Self(value)
            }
        }

        impl From<$wrapper> for $inner {
            fn from(value: $wrapper) -> Self {
                value.0
            }
        }

        impl $wrapper {
            fn bits(&self) -> $inner_bits {
                self.0.to_bits() as $inner_bits
            }
        }

        impl PartialEq for $wrapper {
            fn eq(&self, other: &Self) -> bool {
                self.bits() == other.bits()
            }
        }

        impl Eq for $wrapper {}

        impl PartialOrd for $wrapper {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }

        impl Ord for $wrapper {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.bits().cmp(&other.bits())
            }
        }

        impl Sum for $wrapper {
            fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
                Self(iter.map(|x| x.0).sum())
            }
        }

        impl Add for $wrapper {
            type Output = Self;

            fn add(self, other: Self) -> Self {
                Self(self.0 + other.0)
            }
        }

        impl Sub for $wrapper {
            type Output = Self;

            fn sub(self, other: Self) -> Self {
                Self(self.0 - other.0)
            }
        }

        impl Mul for $wrapper {
            type Output = Self;

            fn mul(self, other: Self) -> Self {
                Self(self.0 * other.0)
            }
        }

        impl Div for $wrapper {
            type Output = Self;

            fn div(self, other: Self) -> Self {
                Self(self.0 / other.0)
            }
        }

        impl Neg for $wrapper {
            type Output = Self;

            fn neg(self) -> Self {
                Self(-self.0)
            }
        }
    };
}

create_ord_float!(OrdF32, f32, i32);
create_ord_float!(OrdF64, f64, i64);
