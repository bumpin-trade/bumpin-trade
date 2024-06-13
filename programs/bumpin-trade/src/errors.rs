use anchor_lang::prelude::*;

pub type BumpResult<T = ()> = std::result::Result<T, BumpErrorCode>;

#[error_code]
#[derive(PartialEq, Eq)]
pub enum BumpErrorCode {
    #[msg("AmountNotEnough")]
    AmountNotEnough,
    #[msg("Invalid transfer")] // 6004
    InvalidTransfer,
    #[msg("InvalidParam")]
    InvalidParam,
    #[msg("OnlyOneShortOrderAllowed")]
    OnlyOneShortOrderAllowed,
    #[msg("OrderNotExist")]
    OrderNotExist,
    #[msg("TokenNotMatch")]
    TokenNotMatch,
    #[msg("NoMoreOrderSpace")]
    NoMoreOrderSpace,
    #[msg("LeverageIsNotAllowed")]
    LeverageIsNotAllowed,
    #[msg("PriceIsNotAllowed")]
    PriceIsNotAllowed,
    #[msg("BalanceNotEnough")]
    BalanceNotEnough,
    #[msg("PythOffline")]
    PythOffline,
    #[msg("Overflow")]
    Overflow,
    #[msg("TransferFailed")]
    TransferFailed,
    #[msg("Unable to load AccountLoader")]
    UnableToLoadAccountLoader,
    #[msg("CantPayUserInitFee")]
    CantPayUserInitFee,
    #[msg("CouldNotFindUserToken")]
    CouldNotFindUserToken,
    #[msg("CouldNotFindUserOrder")]
    CouldNotFindUserOrder,
    #[msg("CouldNotFindUserPosition")]
    CouldNotFindUserPosition,
    #[msg("OnlyLiquidateIsolatePosition")]
    OnlyLiquidateIsolatePosition,
    #[msg("CouldNotFindUserStake")]
    CouldNotFindUserStake,
    #[msg("OracleNotFound")]
    OracleNotFound,
    #[msg("OraclePriceToOld")]
    OraclePriceToOld,
    #[msg("Unable To Load Oracles")]
    UnableToLoadOracle,
    #[msg("InvalidOracle")]
    InvalidOracle,
    #[msg("Conversion to u128/u128 failed with an overflow or underflow")]
    BnConversionError,
    #[msg("Math Error")]
    MathError,
    #[msg("Casting Failure")]
    CastingFailure,
    #[msg("CouldNotLoadMarketData")]
    CouldNotLoadMarketData,
    #[msg("InvalidMarketAccount")]
    InvalidMarketAccount,
    #[msg("MarketWrongMutability")]
    MarketWrongMutability,
    #[msg("Failed Unwrap")]
    FailedUnwrap,
    #[msg("User Not Enough Value")]
    UserNotEnoughValue,
    #[msg("AmountZero")]
    AmountZero,
    #[msg("CouldNotLoadTradeTokenData")]
    CouldNotLoadTradeTokenData,
    #[msg("InvalidTradeTokenAccount")]
    InvalidTradeTokenAccount,
    #[msg("InvalidPoolAccount")]
    InvalidPoolAccount,
    #[msg("CanNotFindTradeToken")]
    TradeTokenNotFind,
    #[msg("StakePaused")]
    StakePaused,
    #[msg("StakeToSmall")]
    StakeToSmall,
    #[msg("UnStakeNotEnough")]
    UnStakeNotEnough,
    #[msg("PositionSideNotSupport")]
    PositionSideNotSupport,
    #[msg("RewardsNotFound")]
    RewardsNotFound,
    #[msg("UserNotFound")]
    UserNotFound,
    #[msg("CouldNotLoadUserData")]
    CouldNotLoadUserData,
}
#[macro_export]
macro_rules! print_error {
    ($err:expr) => {{
        || {
            let error_code: BumpErrorCode = $err;
            msg!("{:?} thrown at {}:{}", error_code, file!(), line!());
            $err
        }
    }};
}

#[macro_export]
macro_rules! math_error {
    () => {{
        || {
            let error_code = $crate::errors::BumpErrorCode::MathError;
            msg!("Error {} thrown at {}:{}", error_code, file!(), line!());
            error_code
        }
    }};
}
