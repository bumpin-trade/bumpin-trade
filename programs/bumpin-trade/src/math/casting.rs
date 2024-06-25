use std::convert::TryInto;
use std::panic::Location;

use anchor_lang::prelude::*;

use crate::errors::BumpErrorCode::CastingFailure;
use crate::errors::BumpResult;
use crate::math::bn::U192;

pub trait Cast: Sized {
    #[track_caller]
    #[inline(always)]
    fn cast<T: TryFrom<Self>>(self) -> BumpResult<T> {
        match self.try_into() {
            Ok(result) => Ok(result),
            Err(_) => {
                let caller = Location::caller();
                msg!("Casting error thrown at {}:{}", caller.file(), caller.line());
                Err(CastingFailure)
            },
        }
    }
}

impl Cast for U192 {}

impl Cast for u128 {}

impl Cast for u64 {}

impl Cast for u32 {}

impl Cast for u16 {}

impl Cast for u8 {}

impl Cast for i128 {}

impl Cast for i64 {}

impl Cast for i32 {}

impl Cast for i16 {}

impl Cast for i8 {}

impl Cast for bool {}
