use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use crate::instructions::constraints::*;
use crate::{utils, validate};
use crate::errors::{BumpErrorCode};
use crate::math::casting::Cast;
use crate::math::safe_math::SafeMath;
use crate::processor::user_processor::UserProcessor;
use crate::state::oracle::oracle_map::OracleMap;
use crate::state::state::State;
use crate::state::trade_token_map::TradeTokenMap;
use crate::state::user::User;

#[derive(Accounts)]
#[instruction(token_index: u16,)]
pub struct Withdraw<'info> {
    pub state: Account<'info, State>,
    #[account(
        mut,
        constraint = can_sign_for_user(& user, & authority) ?
    )]
    pub user: AccountLoader<'info, User>,
    pub authority: Signer<'info>,
    #[account(
        mut,
        constraint = & trade_token_vault.mint.eq(& user_token_account.mint),
        token::authority = authority
    )]
    pub user_token_account: Box<Account<'info, TokenAccount>>,

    #[account(
        seeds = [b"trade_token_vault".as_ref(), token_index.to_le_bytes().as_ref()],
        bump
    )]
    pub trade_token_vault: Box<Account<'info, TokenAccount>>,
    #[account(
        constraint = state.bump_signer.eq(& bump_signer.key())
    )]
    /// CHECK: forced drift_signer
    pub bump_signer: AccountInfo<'info>,
    pub token_program: Program<'info, Token>,
}

pub fn handle_withdraw(ctx: Context<Withdraw>, token_index: u16, amount: u128) -> Result<()> {
    validate!(amount>0, BumpErrorCode::AmountZero);

    let mut user = &mut ctx.accounts.user.load_mut()?;

    let mut user_token = user.get_user_token_mut(&ctx.accounts.user_token_account.mint.key())?;

    validate!(user_token.amount>amount, BumpErrorCode::AmountNotEnough)?;
    let remaining_accounts_iter = &mut ctx.remaining_accounts.iter().peekable();

    let mut oracle_map = OracleMap::load(remaining_accounts_iter)?;
    let trade_token_map = TradeTokenMap::load(remaining_accounts_iter)?;

    let price_data = oracle_map.get_price_data(&ctx.accounts.trade_token_vault.mint)?;
    let withdraw_usd = price_data.price.cast::<i128>()?
        .safe_mul(amount.cast()?)?;

    let mut user_processor = UserProcessor { user };
    let available_value = user_processor.get_available_value(&mut oracle_map, &trade_token_map)?;
    validate!(available_value>withdraw_usd, BumpErrorCode::UserNotEnoughValue)?;

    let bump_signer_nonce = ctx.accounts.state.bump_signer_nonce;
    utils::token::send_from_program_vault(
        &ctx.accounts.token_program,
        &ctx.accounts.trade_token_vault,
        &ctx.accounts.user_token_account,
        &ctx.accounts.bump_signer,
        bump_signer_nonce,
        amount,
    )?;

    //   user_token.sub_token_amount(amount)?;

    user_processor.update_cross_position_balance(&ctx.accounts.user_token_account.mint,
                                                 amount,
                                                 false)?;
    Ok(())
}