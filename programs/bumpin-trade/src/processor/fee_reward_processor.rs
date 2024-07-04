use anchor_lang::emit;

use crate::errors::BumpResult;
use crate::math::safe_math::SafeMath;
use crate::state::bump_events::UserRewardsUpdateEvent;
use crate::state::pool::Pool;
use crate::state::user::User;

