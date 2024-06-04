use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};
use crate::{utils, validate};
use crate::errors::{BumpErrorCode};
use crate::math::safe_math::SafeMath;
use crate::processor::fee_reward_processor::update_account_fee_reward;
use crate::processor::pool_processor::PoolProcessor;
use crate::state::pool::Pool;
use crate::state::state::State;
use crate::state::user::User;
use crate::can_sign_for_user;
use crate::processor::optional_accounts::load_maps;
use std::iter::Peekable;
use std::slice::Iter;
#[derive(Accounts)]
#[instruction(pool_index: u16, trade_token_index: u16)]
pub struct PoolUnStake<'info> {
    #[account(
        mut,
        seeds = [b"bump_state".as_ref()],
        bump,
    )]
    pub state: Account<'info, State>,
    #[account(
        mut,
        seeds = [b"user", authority.key.as_ref()],
        bump,
        constraint = can_sign_for_user(& user, & authority) ?
    )]
    pub user: AccountLoader<'info, User>,

    #[account(
        mut,
        seeds = [b"pool".as_ref(), pool_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub pool: AccountLoader<'info, Pool>,

    #[account(
        mut,
        seeds = [b"pool_mint_vault".as_ref(), pool_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub pool_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        seeds = [b"trade_token_vault".as_ref(), trade_token_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub trade_token_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        constraint = pool_vault.mint.eq(& user_token_account.mint) && trade_token_vault.mint.eq(& user_token_account.mint),
        token::authority = authority
    )]
    pub user_token_account: Box<Account<'info, TokenAccount>>,

    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,

}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Eq, PartialEq)]
pub struct UnStakeParams {
    un_stake_token_amount: u128,
    portfolio: bool,
}

pub fn handle_pool_un_stake<'a, 'b, 'c: 'info, 'info>(ctx: Context<'a, 'b, 'c, 'info,PoolUnStake>, pool_index: usize, un_stake_params: UnStakeParams) -> Result<()> {
    let pool = &mut ctx.accounts.pool.load_mut()?;
    let user = &mut ctx.accounts.user.load_mut()?;

    let user_stake = user.get_user_stake_mut(pool_index)?;
    validate!(user_stake.amount>=un_stake_params.un_stake_token_amount, BumpErrorCode::UnStakeNotEnough);

    let remaining_accounts :&mut Peekable<Iter<'info, AccountInfo<'info>>>= &mut ctx.remaining_accounts.iter().peekable();
    let mut account_maps = load_maps(remaining_accounts)?;


    validate!(pool.total_supply==0, BumpErrorCode::UnStakeNotEnough);

    let mut pool_processor = PoolProcessor { pool };

    let un_stake_token_amount = pool_processor.un_stake(&ctx.accounts.pool,
                                                        &ctx.accounts.user,
                                                        un_stake_params.un_stake_token_amount,
                                                        &mut account_maps.oracle_map,
                                                        &account_maps.market_map)?;

    let un_stake_token_amount_fee = pool_processor.
        collect_un_stake_fee(&ctx.accounts.state, un_stake_token_amount)?;

    update_account_fee_reward(&ctx.accounts.user, &ctx.accounts.pool)?;

    let transfer_amount = un_stake_token_amount.safe_sub(un_stake_token_amount_fee)?;
    if un_stake_params.portfolio {
        let user_token = user.get_user_token_mut(&pool.pool_mint)?;

        utils::token::receive(
            &ctx.accounts.token_program,
            &ctx.accounts.pool_vault,
            &ctx.accounts.trade_token_vault,
            &ctx.accounts.authority,
            transfer_amount,
        )?;

        ctx.accounts.pool_vault.reload()?;
        ctx.accounts.trade_token_vault.reload()?;
        user_token.add_token_amount(transfer_amount)?;
    } else {
        let bump_signer_nonce = ctx.accounts.state.bump_signer_nonce;

        utils::token::send_from_program_vault(
            &ctx.accounts.token_program,
            &ctx.accounts.pool_vault,
            &ctx.accounts.user_token_account,
            &ctx.accounts.authority,
            bump_signer_nonce,
            transfer_amount,
        )?;
        ctx.accounts.pool_vault.reload()?;
    }
    Ok(())
}