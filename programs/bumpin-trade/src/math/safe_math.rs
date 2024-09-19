use crate::errors::{BumpErrorCode, BumpResult};
use crate::math::bn::{U192, U256};
use crate::math::casting::Cast;
use crate::math::ceil_div::CheckedCeilDiv;
use crate::math::constants::{PER_TOKEN_PRECISION, RATE_PRECISION, SMALL_RATE_PRECISION};
use crate::math::floor_div::CheckedFloorDiv;
use anchor_lang::prelude::*;
use std::panic::Location;

pub trait SafeMath: Sized {
    fn safe_add(self, rhs: Self) -> BumpResult<Self>;
    fn safe_sub(self, rhs: Self) -> BumpResult<Self>;
    fn safe_mul(self, rhs: Self) -> BumpResult<Self>;
    fn safe_mul_rate(self, rhs: Self) -> BumpResult<Self>;
    fn safe_mul_small_rate(self, rhs: Self) -> BumpResult<Self>;
    fn safe_mul_per_rate(self, rhs: Self) -> BumpResult<Self>;
    fn safe_div(self, rhs: Self) -> BumpResult<Self>;
    fn safe_div_rate(self, rhs: Self) -> BumpResult<Self>;
    fn safe_div_small_rate(self, rhs: Self) -> BumpResult<Self>;
    fn safe_div_ceil(self, rhs: Self) -> BumpResult<Self>;
}

macro_rules! checked_impl {
    ($t:ty) => {
        impl SafeMath for $t {
            #[track_caller]
            #[inline(always)]
            fn safe_add(self, v: $t) -> BumpResult<$t> {
                match self.checked_add(v) {
                    Some(result) => Ok(result),
                    None => {
                        let caller = Location::caller();
                        msg!("Math error thrown at {}:{}", caller.file(), caller.line());
                        Err(BumpErrorCode::MathError)
                    },
                }
            }

            #[track_caller]
            #[inline(always)]
            fn safe_sub(self, v: $t) -> BumpResult<$t> {
                match self.checked_sub(v) {
                    Some(result) => Ok(result),
                    None => {
                        let caller = Location::caller();
                        msg!("Math error thrown at {}:{}", caller.file(), caller.line());
                        Err(BumpErrorCode::MathError)
                    },
                }
            }

            #[track_caller]
            #[inline(always)]
            fn safe_mul(self, v: $t) -> BumpResult<$t> {
                match self.checked_mul(v) {
                    Some(result) => Ok(result),
                    None => {
                        let caller = Location::caller();
                        msg!("Math error thrown at {}:{}", caller.file(), caller.line());
                        Err(BumpErrorCode::MathError)
                    },
                }
            }

            #[track_caller]
            #[inline(always)]
            fn safe_mul_rate(self, v: $t) -> BumpResult<$t> {
                self.safe_mul(v)?.safe_div(RATE_PRECISION.cast()?)
            }

            #[track_caller]
            #[inline(always)]
            fn safe_mul_small_rate(self, v: $t) -> BumpResult<$t> {
                self.safe_mul(v)?.safe_div(SMALL_RATE_PRECISION.cast()?)
            }

            #[track_caller]
            #[inline(always)]
            fn safe_mul_per_rate(self, v: $t) -> BumpResult<$t> {
                self.safe_mul(v)?.safe_div(PER_TOKEN_PRECISION.cast()?)
            }

            #[track_caller]
            #[inline(always)]
            fn safe_div(self, v: $t) -> BumpResult<$t> {
                match self.checked_div(v) {
                    Some(result) => Ok(result),
                    None => {
                        let caller = Location::caller();
                        msg!("Math error thrown at {}:{}", caller.file(), caller.line());
                        Err(BumpErrorCode::MathError)
                    },
                }
            }

            #[track_caller]
            #[inline(always)]
            fn safe_div_rate(self, v: $t) -> BumpResult<$t> {
                self.safe_mul(RATE_PRECISION.cast()?)?.safe_div(v)
            }

            #[track_caller]
            #[inline(always)]
            fn safe_div_small_rate(self, v: $t) -> BumpResult<$t> {
                self.safe_mul(SMALL_RATE_PRECISION.cast()?)?.safe_div(v)
            }

            #[track_caller]
            #[inline(always)]
            fn safe_div_ceil(self, v: $t) -> BumpResult<$t> {
                match self.checked_ceil_div(v) {
                    Some(result) => Ok(result),
                    None => {
                        let caller = Location::caller();
                        msg!("Math error thrown at {}:{}", caller.file(), caller.line());
                        Err(BumpErrorCode::MathError)
                    },
                }
            }
        }
    };
}

checked_impl!(U256);
checked_impl!(U192);
checked_impl!(u128);
checked_impl!(u64);
checked_impl!(u32);
checked_impl!(u16);
checked_impl!(u8);
checked_impl!(i128);
checked_impl!(i64);
checked_impl!(i32);
checked_impl!(i16);
checked_impl!(i8);

pub trait SafeDivFloor: Sized {
    /// Perform floor division
    fn safe_div_floor(self, rhs: Self) -> BumpResult<Self>;
}

macro_rules! div_floor_impl {
    ($t:ty) => {
        impl SafeDivFloor for $t {
            #[track_caller]
            #[inline(always)]
            fn safe_div_floor(self, v: $t) -> BumpResult<$t> {
                match self.checked_floor_div(v) {
                    Some(result) => Ok(result),
                    None => {
                        let caller = Location::caller();
                        msg!("Math error thrown at {}:{}", caller.file(), caller.line());
                        Err(BumpErrorCode::MathError)
                    },
                }
            }
        }
    };
}

div_floor_impl!(i128);
div_floor_impl!(i64);
div_floor_impl!(i32);
div_floor_impl!(i16);
div_floor_impl!(i8);
