pub mod cal_utils;
pub mod swap;

pub use cal_utils::*;

pub enum Either<A, B> {
    Left(A),
    Right(B),
}
