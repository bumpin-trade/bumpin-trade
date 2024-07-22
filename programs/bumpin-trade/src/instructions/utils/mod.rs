pub mod calculator;
pub mod swap;

pub use calculator::*;

pub enum Either<A, B> {
    Left(A),
    Right(B),
}
