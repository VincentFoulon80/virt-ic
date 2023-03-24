pub mod nes6502;

pub use nes6502::Nes6502;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
#[repr(transparent)]
pub struct Reg<T: Copy> {
    inner: T,
}

macro_rules! default_impl {
    ( $( $name:ident ),*) => {
        $(impl $crate::chip::cpu::Reg<$name> {
            pub fn inc(&mut self) {
                self.inner = self.inner.wrapping_add(1)
            }
            pub fn dec(&mut self) {
                self.inner = self.inner.wrapping_sub(1)
            }
        }
        impl ::std::convert::From<$name> for $crate::chip::cpu::Reg<$name> {
            fn from(value: $name) -> Self {
                Self { inner: value }
            }
        }

        impl ::std::ops::Deref for $crate::chip::cpu::Reg<$name> {
            type Target = $name;

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl ::std::ops::Add for $crate::chip::cpu::Reg<$name> {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                self.inner.wrapping_add(rhs.inner).into()
            }
        }

        impl ::std::ops::Add<$name> for $crate::chip::cpu::Reg<$name> {
            type Output = Self;

            fn add(self, rhs: $name) -> Self::Output {
                self.inner.wrapping_add(rhs).into()
            }
        }

        impl ::std::ops::AddAssign<$name> for $crate::chip::cpu::Reg<$name> {
            fn add_assign(&mut self, other: $name) {
                self.inner = self.inner.wrapping_add(other)
            }
        }

        impl ::std::ops::Sub for $crate::chip::cpu::Reg<$name> {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                self.inner.wrapping_sub(rhs.inner).into()
            }
        }

        impl ::std::ops::Sub<$name> for $crate::chip::cpu::Reg<$name> {
            type Output = Self;

            fn sub(self, rhs: $name) -> Self::Output {
                self.inner.wrapping_sub(rhs).into()
            }
        }

        impl ::std::ops::SubAssign<$name> for $crate::chip::cpu::Reg<$name> {
            fn sub_assign(&mut self, other: $name) {
                self.inner = self.inner.wrapping_sub(other)
            }
        }

        impl ::std::ops::BitAnd for $crate::chip::cpu::Reg<$name> {
            type Output = Self;

            fn bitand(mut self, rhs: Self) -> Self::Output {
                self.inner &= rhs.inner;
                self
            }
        }

        impl ::std::ops::BitAnd<$name> for $crate::chip::cpu::Reg<$name> {
            type Output = Self;

            fn bitand(mut self, rhs: $name) -> Self::Output {
                self.inner &= rhs;
                self
            }
        }

        impl ::std::ops::BitAndAssign<$name> for $crate::chip::cpu::Reg<$name> {
            fn bitand_assign(&mut self, other: $name) {
                self.inner = self.inner & other
            }
        }

        impl ::std::ops::BitOr for $crate::chip::cpu::Reg<$name> {
            type Output = Self;

            fn bitor(mut self, rhs: Self) -> Self::Output {
                self.inner |= rhs.inner;
                self
            }
        }

        impl ::std::ops::BitOr<$name> for $crate::chip::cpu::Reg<$name> {
            type Output = Self;

            fn bitor(mut self, rhs: $name) -> Self::Output {
                self.inner |= rhs;
                self
            }
        }

        impl ::std::ops::BitOrAssign<$name> for $crate::chip::cpu::Reg<$name> {
            fn bitor_assign(&mut self, other: $name) {
                self.inner = self.inner | other
            }
        }

        impl ::std::ops::BitXor for $crate::chip::cpu::Reg<$name> {
            type Output = Self;

            fn bitxor(mut self, rhs: Self) -> Self::Output {
                self.inner ^= rhs.inner;
                self
            }
        }

        impl ::std::ops::BitXor<$name> for $crate::chip::cpu::Reg<$name> {
            type Output = Self;

            fn bitxor(mut self, rhs: $name) -> Self::Output {
                self.inner ^= rhs;
                self
            }
        }

        impl ::std::ops::BitXorAssign<$name> for $crate::chip::cpu::Reg<$name> {
            fn bitxor_assign(&mut self, other: $name) {
                self.inner = self.inner ^ other
            }
        }

        impl ::std::ops::Shl for $crate::chip::cpu::Reg<$name> {
            type Output = Self;

            fn shl(mut self, rhs: Self) -> Self::Output {
                self.inner <<= rhs.inner;
                self
            }
        }

        impl ::std::ops::Shl<$name> for $crate::chip::cpu::Reg<$name> {
            type Output = Self;

            fn shl(mut self, rhs: $name) -> Self::Output {
                self.inner <<= rhs;
                self
            }
        }

        impl ::std::ops::ShlAssign<$name> for $crate::chip::cpu::Reg<$name> {
            fn shl_assign(&mut self, other: $name) {
                self.inner = self.inner << other
            }
        }

        impl ::std::ops::Shr for $crate::chip::cpu::Reg<$name> {
            type Output = Self;

            fn shr(mut self, rhs: Self) -> Self::Output {
                self.inner >>= rhs.inner;
                self
            }
        }

        impl ::std::ops::Shr<$name> for $crate::chip::cpu::Reg<$name> {
            type Output = Self;

            fn shr(mut self, rhs: $name) -> Self::Output {
                self.inner >>= rhs;
                self
            }
        }

        impl ::std::ops::ShrAssign<$name> for $crate::chip::cpu::Reg<$name> {
            fn shr_assign(&mut self, other: $name) {
                self.inner = self.inner >> other
            }
        }

        impl ::std::cmp::PartialEq<$name> for $crate::chip::cpu::Reg<$name> {
            fn eq(&self, other: &$name) -> bool {
                self.inner.eq(other)
            }
        }

        impl ::std::cmp::PartialOrd<$name> for $crate::chip::cpu::Reg<$name> {
            fn partial_cmp(&self, other: &$name) -> ::std::option::Option<::std::cmp::Ordering> {
                self.inner.partial_cmp(other)
            }
        }
    )*
    };
}

default_impl!(u8, u16);
