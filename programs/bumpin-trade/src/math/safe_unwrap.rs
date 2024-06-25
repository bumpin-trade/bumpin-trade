use std::panic::Location;

use crate::errors::BumpErrorCode::FailedUnwrap;
use anchor_lang::prelude::*;

use crate::errors::BumpResult;

pub trait SafeUnwrap {
    type Item;

    fn safe_unwrap(self) -> BumpResult<Self::Item>;
}

impl<T> SafeUnwrap for Option<T> {
    type Item = T;

    #[track_caller]
    #[inline(always)]
    fn safe_unwrap(self) -> BumpResult<T> {
        match self {
            Some(v) => Ok(v),
            None => {
                let caller = Location::caller();
                msg!("Unwrap error thrown at {}:{}", caller.file(), caller.line());
                Err(FailedUnwrap)
            },
        }
    }
}

impl<T, U> SafeUnwrap for std::result::Result<T, U> {
    type Item = T;

    #[track_caller]
    #[inline(always)]
    fn safe_unwrap(self) -> BumpResult<T> {
        match self {
            Ok(v) => Ok(v),
            Err(_) => {
                let caller = Location::caller();
                msg!("Unwrap error thrown at {}:{}", caller.file(), caller.line());
                Err(FailedUnwrap)
            },
        }
    }
}
