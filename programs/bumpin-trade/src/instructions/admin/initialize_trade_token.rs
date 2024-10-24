use anchor_lang::error;
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

use crate::math::casting::Cast;
use crate::math_error;
use crate::safe_increment;
use crate::state::state::State;
use crate::state::trade_token::TradeToken;

#[derive(Accounts)]
pub struct InitializeTradeToken<'info> {
    #[account(
        init,
        seeds = [b"trade_token", state.trade_token_sequence.to_le_bytes().as_ref()],
        space = std::mem::size_of::< TradeToken > () + 8,
        bump,
        payer = admin
    )]
    pub trade_token: AccountLoader<'info, TradeToken>,
    pub trade_token_mint: Box<Account<'info, Mint>>,
    #[account(
        init,
        seeds = [b"trade_token_vault".as_ref(), state.trade_token_sequence.to_le_bytes().as_ref()],
        bump,
        payer = admin,
        token::mint = trade_token_mint,
        token::authority = bump_signer
    )]
    pub trade_token_vault: Box<Account<'info, TokenAccount>>,
    /// CHECK: ?
    #[account(
        constraint = state.bump_signer.eq(& bump_signer.key())
    )]
    pub bump_signer: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [b"bump_state".as_ref()],
        bump,
        has_one = admin
    )]
    pub state: Account<'info, State>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,

    #[account(address = Token::id())]
    pub token_program: Program<'info, Token>,
}

pub fn handle_initialize_trade_token(
    ctx: Context<InitializeTradeToken>,
    discount: u32,
    name: [u8; 32],
    feed_id: Pubkey,
    liquidation_factor: u32,
) -> Result<()> {
    let state = &mut ctx.accounts.state;
    let trade_token = &mut ctx.accounts.trade_token.load_init()?;
    **trade_token = TradeToken {
        mint_key: ctx.accounts.trade_token_vault.mint,
        name,
        feed_id,
        index: state.trade_token_sequence,
        discount,
        liquidation_factor,
        decimals: ctx.accounts.trade_token_mint.decimals.cast::<u16>()?,
        total_liability: 0,
        total_amount: 0,
        vault_key: *ctx.accounts.trade_token_vault.to_account_info().key,
        padding: [0; 4],
        reserve_padding: [0; 32],
    };
    safe_increment!(state.trade_token_sequence, 1);
    Ok(())
}

#[derive(Accounts)]
#[instruction(
    params: ModifyTradeTokenParams,
)]
pub struct ModifyTradeToken<'info> {
    #[account(
        mut,
        seeds = [b"trade_token", params.trade_token_index.to_le_bytes().as_ref()],
        bump,
    )]
    pub trade_token: AccountLoader<'info, TradeToken>,

    #[account(
        seeds = [b"bump_state".as_ref()],
        bump,
        has_one = admin
    )]
    pub state: Account<'info, State>,

    #[account(
        mut,
        constraint = state.admin.eq(& admin.key())
    )]
    pub admin: Signer<'info>,
}

#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone, Copy)]
pub struct ModifyTradeTokenParams {
    pub trade_token_index: u16,
    pub name: Option<[u8; 32]>,
    pub discount: Option<u32>,
    pub feed_id: Option<[u8; 32]>,
    pub liquidation_factor: Option<u32>,
}

pub fn handle_modify_trade_token(
    ctx: Context<ModifyTradeToken>,
    params: ModifyTradeTokenParams,
) -> Result<()> {
    let trade_token = &mut ctx.accounts.trade_token.load_mut()?;
    if let Some(name) = params.name {
        trade_token.name = name;
    }
    if let Some(discount) = params.discount {
        trade_token.discount = discount;
    }
    if let Some(feed_id) = params.feed_id {
        trade_token.feed_id = Pubkey::new_from_array(feed_id);
    }
    if let Some(liquidation_factor) = params.liquidation_factor {
        trade_token.liquidation_factor = liquidation_factor;
    }
    Ok(())
}
