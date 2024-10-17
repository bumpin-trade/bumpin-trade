#[macro_export]
macro_rules! get_struct_values {
    ($struct:expr, $($property: ident),+) => {{
        ($(
            $struct.$property,
        )+)
    }};
}

#[macro_export]
macro_rules! price {
    ($trade_token:expr, $oracles:expr) => {
        $oracles.get_price_data(&$trade_token.feed_id)?.price
    };
}

#[macro_export]
macro_rules! get_then_update_id {
    ($struct:expr, $property: ident) => {{
        let current_id = $struct.$property;
        $struct.$property = current_id.checked_add(1).or(Some(1)).unwrap();
        current_id
    }};
}

#[macro_export]
macro_rules! validate {
    ($assert:expr, $err:expr) => {{
        if ($assert) {
            Ok(())
        } else {
            let error_code: BumpErrorCode = $err;
            Err(error_code)
        }
    }};
    ($assert:expr, $err:expr, $($arg:tt)+) => {{
        if ($assert) {
            Ok(())
        } else {
            let error_code: BumpErrorCode = $err;
            Err(error_code)
        }
    }};
}

#[macro_export]
macro_rules! dlog {
    ($($variable: expr),+) => {{
        $(
            msg!("{}: {}", stringify!($variable), $variable);
        )+
    }};
    ($($arg:tt)+) => {{
            #[cfg(not(feature = "mainnet-beta"))]
            msg!($($arg)+);
    }};
}

#[macro_export]
macro_rules! load_mut {
    ($account_loader:expr) => {{
        $account_loader.load_mut().map_err(|e| {
            msg!("e {:?}", e);
            let error_code = BumpErrorCode::UnableToLoadAccountLoader;
            msg!("Error {} thrown at {}:{}", error_code, file!(), line!());
            error_code
        })
    }};
}

#[macro_export]
macro_rules! load {
    ($account_loader:expr) => {{
        $account_loader.load().map_err(|_| {
            let error_code = BumpErrorCode::UnableToLoadAccountLoader;
            msg!("Error {} thrown at {}:{}", error_code, file!(), line!());
            error_code
        })
    }};
}

#[macro_export]
macro_rules! safe_increment {
    ($struct:expr, $value:expr) => {{
        $struct = $struct.checked_add($value).ok_or_else(math_error!())?
    }};
}

#[macro_export]
macro_rules! safe_decrement {
    ($struct:expr, $value:expr) => {{
        $struct = $struct.checked_sub($value).ok_or_else(math_error!())?
    }};
}
#[macro_export]
macro_rules! position_mut {
    ($user_positions:expr, $position_key:expr) => {{
        let mut found = None;
        for user_position in &mut *$user_positions {
            if user_position.status == PositionStatus::USING
                && user_position.position_key == *$position_key
            {
                found = Some(user_position);
                break;
            }
        }
        match found {
            Some(pos) => Ok(pos),
            None => Err($crate::errors::BumpErrorCode::CouldNotFindUserPosition),
        }
    }};
}

#[macro_export]
macro_rules! position {
    ($user_positions:expr, $position_key:expr) => {{
        let mut found = None;
        for user_position in &*($user_positions) {
            if user_position.status == PositionStatus::USING
                && user_position.position_key == *$position_key
            {
                found = Some(user_position);
                break;
            }
        }
        match found {
            Some(pos) => Ok(pos),
            None => Err($crate::errors::BumpErrorCode::CouldNotFindUserPosition),
        }
    }};
}
